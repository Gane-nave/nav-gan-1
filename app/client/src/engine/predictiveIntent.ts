/**
 * G.A.N.E Predictive Intent Recognition Engine
 * 
 * Predicts user destination before they type it, using:
 * - Temporal patterns (time-of-day, day-of-week)
 * - Behavioral habits (frequent destinations, route preferences)
 * - Calendar integration (upcoming events with locations)
 * - Contextual signals (speed, heading, recent searches)
 * 
 * Algorithm: Weighted Bayesian scoring with exponential decay
 * for recency, combined with Markov chain transitions.
 */

// ═══════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════

export interface PredictedDestination {
  id: string;
  name: string;
  lat: number;
  lng: number;
  confidence: number;        // 0-1 probability
  reason: PredictionReason;
  eta?: number;              // estimated minutes
  category: DestinationCategory;
  lastVisited?: number;      // timestamp
  visitCount: number;
}

export type PredictionReason =
  | 'calendar_event'
  | 'daily_routine'
  | 'weekly_pattern'
  | 'frequent_destination'
  | 'recent_search'
  | 'heading_trajectory'
  | 'time_correlation'
  | 'contextual_inference';

export type DestinationCategory =
  | 'home' | 'work' | 'school' | 'gym' | 'restaurant'
  | 'shopping' | 'medical' | 'social' | 'recreation' | 'other';

export interface CalendarEvent {
  id: string;
  title: string;
  startTime: number;
  endTime: number;
  location?: {
    name: string;
    lat: number;
    lng: number;
  };
}

export interface VisitRecord {
  destinationId: string;
  name: string;
  lat: number;
  lng: number;
  category: DestinationCategory;
  timestamp: number;
  dayOfWeek: number;       // 0-6
  hourOfDay: number;       // 0-23
  dwellTime: number;       // minutes spent
  arrivalHeading: number;  // degrees
}

export interface UserContext {
  currentLat: number;
  currentLng: number;
  currentHeading: number;
  currentSpeed: number;    // m/s
  timestamp: number;
  recentSearches: string[];
}

interface TemporalBucket {
  dayOfWeek: number;
  hourSlot: number;        // 0-47 (30-min slots)
  destinations: Map<string, number>; // destId -> visit count
}

interface MarkovState {
  fromId: string;
  transitions: Map<string, number>; // toId -> count
}

// ═══════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════

const CONFIG = {
  maxPredictions: 5,
  recencyDecayHalfLife: 7 * 24 * 3600 * 1000,  // 7 days in ms
  calendarLookahead: 2 * 3600 * 1000,           // 2 hours ahead
  minConfidenceThreshold: 0.05,
  temporalWeight: 0.30,
  frequencyWeight: 0.25,
  calendarWeight: 0.20,
  markovWeight: 0.15,
  trajectoryWeight: 0.10,
  maxHistoryRecords: 10000,
  temporalSlotMinutes: 30,
};

// ═══════════════════════════════════════════════════════════
// PREDICTIVE INTENT ENGINE
// ═══════════════════════════════════════════════════════════

export class PredictiveIntentEngine {
  private visitHistory: VisitRecord[] = [];
  private calendarEvents: CalendarEvent[] = [];
  private temporalBuckets: Map<string, TemporalBucket> = new Map();
  private markovChain: Map<string, MarkovState> = new Map();
  private destinationProfiles: Map<string, {
    name: string;
    lat: number;
    lng: number;
    category: DestinationCategory;
    totalVisits: number;
    avgDwellTime: number;
    lastVisit: number;
  }> = new Map();
  private lastPrediction: PredictedDestination[] = [];
  private initialized: boolean = false;

  constructor() {
    console.log('[PredictiveIntent] Engine created');
  }

  // ─── INITIALIZATION ────────────────────────────────────

  async initialize(): Promise<void> {
    // Load history from IndexedDB if available
    try {
      if (typeof indexedDB !== 'undefined') {
        await this.loadFromStorage();
      }
    } catch (e) {
      console.warn('[PredictiveIntent] Failed to load history from storage', e);
    }
    this.initialized = true;
    console.log(`[PredictiveIntent] Initialized with ${this.visitHistory.length} records`);
  }

  // ─── RECORD VISITS ────────────────────────────────────

  recordVisit(record: VisitRecord): void {
    this.visitHistory.push(record);

    // Trim history if too large
    if (this.visitHistory.length > CONFIG.maxHistoryRecords) {
      this.visitHistory = this.visitHistory.slice(-CONFIG.maxHistoryRecords);
    }

    // Update destination profile
    const profile = this.destinationProfiles.get(record.destinationId);
    if (profile) {
      profile.totalVisits++;
      profile.avgDwellTime = (profile.avgDwellTime * (profile.totalVisits - 1) + record.dwellTime) / profile.totalVisits;
      profile.lastVisit = record.timestamp;
    } else {
      this.destinationProfiles.set(record.destinationId, {
        name: record.name,
        lat: record.lat,
        lng: record.lng,
        category: record.category,
        totalVisits: 1,
        avgDwellTime: record.dwellTime,
        lastVisit: record.timestamp,
      });
    }

    // Update temporal buckets
    const bucketKey = `${record.dayOfWeek}-${Math.floor(record.hourOfDay * 2)}`;
    let bucket = this.temporalBuckets.get(bucketKey);
    if (!bucket) {
      bucket = {
        dayOfWeek: record.dayOfWeek,
        hourSlot: Math.floor(record.hourOfDay * 2),
        destinations: new Map(),
      };
      this.temporalBuckets.set(bucketKey, bucket);
    }
    bucket.destinations.set(
      record.destinationId,
      (bucket.destinations.get(record.destinationId) || 0) + 1
    );

    // Update Markov chain (transition from previous destination)
    if (this.visitHistory.length >= 2) {
      const prev = this.visitHistory[this.visitHistory.length - 2];
      let state = this.markovChain.get(prev.destinationId);
      if (!state) {
        state = { fromId: prev.destinationId, transitions: new Map() };
        this.markovChain.set(prev.destinationId, state);
      }
      state.transitions.set(
        record.destinationId,
        (state.transitions.get(record.destinationId) || 0) + 1
      );
    }

    // Persist asynchronously
    this.saveToStorage().catch(() => {});
  }

  // ─── CALENDAR INTEGRATION ─────────────────────────────

  updateCalendar(events: CalendarEvent[]): void {
    this.calendarEvents = events;
  }

  // ─── PREDICTION ────────────────────────────────────────

  predict(context: UserContext): PredictedDestination[] {
    const now = context.timestamp;
    const date = new Date(now);
    const dayOfWeek = date.getDay();
    const hourSlot = Math.floor(date.getHours() * 2 + date.getMinutes() / 30);

    const scores = new Map<string, {
      score: number;
      reasons: Map<PredictionReason, number>;
    }>();

    // Helper to add score
    const addScore = (destId: string, amount: number, reason: PredictionReason) => {
      let entry = scores.get(destId);
      if (!entry) {
        entry = { score: 0, reasons: new Map() };
        scores.set(destId, entry);
      }
      entry.score += amount;
      entry.reasons.set(reason, (entry.reasons.get(reason) || 0) + amount);
    };

    // ── 1. Temporal Pattern Scoring ──────────────────────
    const bucketKey = `${dayOfWeek}-${hourSlot}`;
    const bucket = this.temporalBuckets.get(bucketKey);
    if (bucket) {
      let totalInBucket = 0;
      bucket.destinations.forEach(count => { totalInBucket += count; });
      bucket.destinations.forEach((count, destId) => {
        const temporalScore = (count / totalInBucket) * CONFIG.temporalWeight;
        addScore(destId, temporalScore, 'time_correlation');
      });
    }

    // Also check adjacent time slots for smoothing
    for (const offset of [-1, 1]) {
      const adjSlot = ((hourSlot + offset) + 48) % 48;
      const adjKey = `${dayOfWeek}-${adjSlot}`;
      const adjBucket = this.temporalBuckets.get(adjKey);
      if (adjBucket) {
        let total = 0;
        adjBucket.destinations.forEach(count => { total += count; });
        adjBucket.destinations.forEach((count, destId) => {
          const score = (count / total) * CONFIG.temporalWeight * 0.3; // 30% weight for adjacent
          addScore(destId, score, 'daily_routine');
        });
      }
    }

    // ── 2. Frequency + Recency Scoring ───────────────────
    this.destinationProfiles.forEach((profile, destId) => {
      // Exponential decay based on recency
      const age = now - profile.lastVisit;
      const recencyFactor = Math.pow(0.5, age / CONFIG.recencyDecayHalfLife);

      // Frequency score (log scale to prevent dominant destinations)
      const freqScore = Math.log2(1 + profile.totalVisits) / 10;

      const combinedScore = (freqScore * recencyFactor) * CONFIG.frequencyWeight;
      addScore(destId, combinedScore, 'frequent_destination');
    });

    // ── 3. Calendar Event Scoring ────────────────────────
    for (const event of this.calendarEvents) {
      if (!event.location) continue;
      const timeUntilEvent = event.startTime - now;
      if (timeUntilEvent < 0 || timeUntilEvent > CONFIG.calendarLookahead) continue;

      // Higher score as event approaches
      const urgency = 1 - (timeUntilEvent / CONFIG.calendarLookahead);
      const calScore = urgency * CONFIG.calendarWeight;

      // Find or create destination ID for calendar location
      const calDestId = `cal-${event.id}`;
      if (!this.destinationProfiles.has(calDestId)) {
        this.destinationProfiles.set(calDestId, {
          name: event.location.name,
          lat: event.location.lat,
          lng: event.location.lng,
          category: 'other',
          totalVisits: 0,
          avgDwellTime: (event.endTime - event.startTime) / 60000,
          lastVisit: 0,
        });
      }
      addScore(calDestId, calScore, 'calendar_event');
    }

    // ── 4. Markov Chain Scoring ──────────────────────────
    if (this.visitHistory.length > 0) {
      const lastVisit = this.visitHistory[this.visitHistory.length - 1];
      const state = this.markovChain.get(lastVisit.destinationId);
      if (state) {
        let totalTransitions = 0;
        state.transitions.forEach(count => { totalTransitions += count; });
        state.transitions.forEach((count, destId) => {
          const transProb = count / totalTransitions;
          addScore(destId, transProb * CONFIG.markovWeight, 'weekly_pattern');
        });
      }
    }

    // ── 5. Trajectory/Heading Scoring ────────────────────
    if (context.currentSpeed > 1) { // Moving faster than 1 m/s
      this.destinationProfiles.forEach((profile, destId) => {
        const bearing = this.calculateBearing(
          context.currentLat, context.currentLng,
          profile.lat, profile.lng
        );
        const headingDiff = Math.abs(this.normalizeAngle(context.currentHeading - bearing));

        // Score inversely proportional to heading difference
        if (headingDiff < 30) {
          const trajectoryScore = (1 - headingDiff / 30) * CONFIG.trajectoryWeight;
          addScore(destId, trajectoryScore, 'heading_trajectory');
        }
      });
    }

    // ── 6. Recent Search Scoring ─────────────────────────
    if (context.recentSearches.length > 0) {
      this.destinationProfiles.forEach((profile, destId) => {
        for (const search of context.recentSearches) {
          if (profile.name.toLowerCase().includes(search.toLowerCase())) {
            addScore(destId, 0.1, 'recent_search');
          }
        }
      });
    }

    // ── Normalize & Rank ─────────────────────────────────
    let maxScore = 0;
    scores.forEach(entry => {
      if (entry.score > maxScore) maxScore = entry.score;
    });

    const predictions: PredictedDestination[] = [];
    scores.forEach((entry, destId) => {
      const profile = this.destinationProfiles.get(destId);
      if (!profile) return;

      const confidence = maxScore > 0 ? entry.score / maxScore : 0;
      if (confidence < CONFIG.minConfidenceThreshold) return;

      // Find primary reason
      let primaryReason: PredictionReason = 'contextual_inference';
      let maxReasonScore = 0;
      entry.reasons.forEach((score, reason) => {
        if (score > maxReasonScore) {
          maxReasonScore = score;
          primaryReason = reason;
        }
      });

      // Estimate ETA
      const distance = this.haversineDistance(
        context.currentLat, context.currentLng,
        profile.lat, profile.lng
      );
      const avgSpeedKmh = context.currentSpeed > 2 ? context.currentSpeed * 3.6 : 40;
      const etaMinutes = (distance / avgSpeedKmh) * 60;

      predictions.push({
        id: destId,
        name: profile.name,
        lat: profile.lat,
        lng: profile.lng,
        confidence: Math.min(1, confidence),
        reason: primaryReason,
        eta: Math.round(etaMinutes),
        category: profile.category,
        lastVisited: profile.lastVisit || undefined,
        visitCount: profile.totalVisits,
      });
    });

    // Sort by confidence descending
    predictions.sort((a, b) => b.confidence - a.confidence);

    this.lastPrediction = predictions.slice(0, CONFIG.maxPredictions);
    return this.lastPrediction;
  }

  // ─── LEARNING FEEDBACK ─────────────────────────────────

  /**
   * Called when user selects a destination.
   * Reinforces the prediction model.
   */
  onDestinationSelected(destId: string): void {
    const prediction = this.lastPrediction.find(p => p.id === destId);
    if (prediction) {
      console.log(
        `[PredictiveIntent] Prediction hit: "${prediction.name}" (confidence: ${(prediction.confidence * 100).toFixed(1)}%)`
      );
    }
  }

  /**
   * Called when user navigates to an unpredicted destination.
   * The model learns from the miss.
   */
  onUnpredictedNavigation(dest: { name: string; lat: number; lng: number; category: DestinationCategory }): void {
    const now = Date.now();
    const date = new Date(now);
    const destId = `dest-${dest.lat.toFixed(4)}-${dest.lng.toFixed(4)}`;

    this.recordVisit({
      destinationId: destId,
      name: dest.name,
      lat: dest.lat,
      lng: dest.lng,
      category: dest.category,
      timestamp: now,
      dayOfWeek: date.getDay(),
      hourOfDay: date.getHours() + date.getMinutes() / 60,
      dwellTime: 0,
      arrivalHeading: 0,
    });
  }

  // ─── ANALYTICS ─────────────────────────────────────────

  getStats(): {
    totalRecords: number;
    uniqueDestinations: number;
    topDestinations: { name: string; visits: number }[];
    predictionAccuracy: number;
    modelAge: number;
  } {
    const topDests: { name: string; visits: number }[] = [];
    this.destinationProfiles.forEach(profile => {
      topDests.push({ name: profile.name, visits: profile.totalVisits });
    });
    topDests.sort((a, b) => b.visits - a.visits);

    return {
      totalRecords: this.visitHistory.length,
      uniqueDestinations: this.destinationProfiles.size,
      topDestinations: topDests.slice(0, 10),
      predictionAccuracy: this.calculateAccuracy(),
      modelAge: this.visitHistory.length > 0
        ? Date.now() - this.visitHistory[0].timestamp
        : 0,
    };
  }

  getLastPredictions(): PredictedDestination[] {
    return [...this.lastPrediction];
  }

  // ─── PRIVATE HELPERS ───────────────────────────────────

  private calculateAccuracy(): number {
    // Simplified accuracy: ratio of visits to known destinations
    if (this.visitHistory.length < 10) return 0;
    const known = this.visitHistory.filter(v =>
      this.destinationProfiles.has(v.destinationId) &&
      (this.destinationProfiles.get(v.destinationId)?.totalVisits || 0) > 1
    );
    return known.length / this.visitHistory.length;
  }

  private haversineDistance(lat1: number, lng1: number, lat2: number, lng2: number): number {
    const R = 6371; // km
    const dLat = (lat2 - lat1) * Math.PI / 180;
    const dLng = (lng2 - lng1) * Math.PI / 180;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * Math.PI / 180) * Math.cos(lat2 * Math.PI / 180) *
      Math.sin(dLng / 2) ** 2;
    return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  private calculateBearing(lat1: number, lng1: number, lat2: number, lng2: number): number {
    const dLng = (lng2 - lng1) * Math.PI / 180;
    const lat1R = lat1 * Math.PI / 180;
    const lat2R = lat2 * Math.PI / 180;
    const y = Math.sin(dLng) * Math.cos(lat2R);
    const x = Math.cos(lat1R) * Math.sin(lat2R) - Math.sin(lat1R) * Math.cos(lat2R) * Math.cos(dLng);
    return ((Math.atan2(y, x) * 180 / Math.PI) + 360) % 360;
  }

  private normalizeAngle(angle: number): number {
    while (angle > 180) angle -= 360;
    while (angle < -180) angle += 360;
    return angle;
  }

  // ─── PERSISTENCE (IndexedDB) ───────────────────────────

  private async loadFromStorage(): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open('gane-predictive-intent', 1);

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains('visits')) {
          db.createObjectStore('visits', { keyPath: 'timestamp' });
        }
        if (!db.objectStoreNames.contains('profiles')) {
          db.createObjectStore('profiles', { keyPath: 'id' });
        }
      };

      request.onsuccess = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        const tx = db.transaction(['visits', 'profiles'], 'readonly');

        const visitStore = tx.objectStore('visits');
        const visitReq = visitStore.getAll();
        visitReq.onsuccess = () => {
          this.visitHistory = visitReq.result || [];
          // Rebuild temporal buckets and markov chain from history
          for (const record of this.visitHistory) {
            const bucketKey = `${record.dayOfWeek}-${Math.floor(record.hourOfDay * 2)}`;
            let bucket = this.temporalBuckets.get(bucketKey);
            if (!bucket) {
              bucket = {
                dayOfWeek: record.dayOfWeek,
                hourSlot: Math.floor(record.hourOfDay * 2),
                destinations: new Map(),
              };
              this.temporalBuckets.set(bucketKey, bucket);
            }
            bucket.destinations.set(
              record.destinationId,
              (bucket.destinations.get(record.destinationId) || 0) + 1
            );
          }
          // Rebuild Markov chain
          for (let i = 1; i < this.visitHistory.length; i++) {
            const prev = this.visitHistory[i - 1];
            const curr = this.visitHistory[i];
            let state = this.markovChain.get(prev.destinationId);
            if (!state) {
              state = { fromId: prev.destinationId, transitions: new Map() };
              this.markovChain.set(prev.destinationId, state);
            }
            state.transitions.set(
              curr.destinationId,
              (state.transitions.get(curr.destinationId) || 0) + 1
            );
          }
        };

        const profileStore = tx.objectStore('profiles');
        const profileReq = profileStore.getAll();
        profileReq.onsuccess = () => {
          const profiles = profileReq.result || [];
          for (const p of profiles) {
            this.destinationProfiles.set(p.id, p);
          }
        };

        tx.oncomplete = () => {
          db.close();
          resolve();
        };
        tx.onerror = () => {
          db.close();
          reject(tx.error);
        };
      };

      request.onerror = () => reject(request.error);
    });
  }

  private async saveToStorage(): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open('gane-predictive-intent', 1);

      request.onsuccess = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        const tx = db.transaction(['visits', 'profiles'], 'readwrite');

        // Save last 1000 visits
        const visitStore = tx.objectStore('visits');
        visitStore.clear();
        const recentVisits = this.visitHistory.slice(-1000);
        for (const visit of recentVisits) {
          visitStore.put(visit);
        }

        // Save profiles
        const profileStore = tx.objectStore('profiles');
        profileStore.clear();
        this.destinationProfiles.forEach((profile, id) => {
          profileStore.put({ id, ...profile });
        });

        tx.oncomplete = () => {
          db.close();
          resolve();
        };
        tx.onerror = () => {
          db.close();
          reject(tx.error);
        };
      };

      request.onerror = () => reject(request.error);
    });
  }

  destroy(): void {
    this.visitHistory = [];
    this.calendarEvents = [];
    this.temporalBuckets.clear();
    this.markovChain.clear();
    this.destinationProfiles.clear();
    this.lastPrediction = [];
    console.log('[PredictiveIntent] Engine destroyed');
  }
}
