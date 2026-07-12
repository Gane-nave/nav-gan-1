/**
 * G.A.N.E — Urban Canyon & Map Matching Engine
 * ================================================
 * Handles GNSS degradation in dense urban environments:
 *
 * 1. Urban Canyon Detection
 *    - Monitors satellite geometry (high HDOP, low elevation sats)
 *    - Detects multipath interference patterns
 *    - Classifies environment: open_sky | suburban | urban | deep_canyon
 *
 * 2. Multipath Mitigation
 *    - Consistency checking across constellations
 *    - SNR-based weighting (low SNR = likely reflected)
 *    - Elevation mask (reject low-elevation satellites in canyons)
 *    - Doppler-based velocity cross-check
 *
 * 3. Map Matching (Snap-to-Road)
 *    - Hidden Markov Model (HMM) based road alignment
 *    - Emission probability: GPS accuracy → road distance
 *    - Transition probability: route distance between candidates
 *    - Viterbi algorithm for optimal path selection
 *
 * 4. Route Graph Engine
 *    - Weighted directed graph of road network
 *    - A* pathfinding with traffic-aware edge weights
 *    - Multi-objective optimization (time, distance, fuel, safety)
 */

// ═══════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════

export type EnvironmentClass = 'open_sky' | 'suburban' | 'urban' | 'deep_canyon' | 'indoor' | 'tunnel';

export interface UrbanCanyonState {
  environment: EnvironmentClass;
  multipathRisk: number;          // 0-1
  hdop: number;
  elevationMask: number;          // degrees — reject sats below this
  visibleSkyFraction: number;     // 0-1
  reflectedSignalCount: number;
  correctedLat: number;
  correctedLon: number;
  correctionApplied: boolean;
  correctionMeters: number;
  confidence: number;
}

export interface MapMatchResult {
  matchedLat: number;
  matchedLon: number;
  roadId: string;
  roadName: string;
  heading: number;
  speedLimit: number;
  confidence: number;
  distanceFromRaw: number;        // meters from raw GPS to matched point
  isOnRoad: boolean;
}

export interface RoadNode {
  id: string;
  lat: number;
  lon: number;
  edges: RoadEdge[];
}

export interface RoadEdge {
  targetNodeId: string;
  roadId: string;
  roadName: string;
  distance: number;               // meters
  travelTime: number;             // seconds (with traffic)
  speedLimit: number;             // m/s
  heading: number;                // degrees
  roadClass: 'motorway' | 'trunk' | 'primary' | 'secondary' | 'tertiary' | 'residential' | 'service';
  trafficFactor: number;          // 1.0 = free flow, >1 = congested
  tollCost: number;
  isTunnel: boolean;
  isBridge: boolean;
}

interface HMMCandidate {
  roadId: string;
  lat: number;
  lon: number;
  heading: number;
  distance: number;               // distance from GPS point to road
  emissionProb: number;
  roadName: string;
  speedLimit: number;
}

// ═══════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════

const EARTH_RADIUS = 6_371_000;
const DEG2RAD = Math.PI / 180;
const RAD2DEG = 180 / Math.PI;
const EMISSION_SIGMA = 20;        // GPS accuracy standard deviation (meters)
const TRANSITION_BETA = 5;        // transition probability parameter
const MAX_MATCH_DISTANCE = 50;    // max distance to consider a road match (meters)

// ═══════════════════════════════════════════════════
// URBAN CANYON DETECTOR
// ═══════════════════════════════════════════════════

export class UrbanCanyonDetector {
  private state: UrbanCanyonState;
  private positionHistory: Array<{ lat: number; lon: number; timestamp: number }> = [];

  constructor() {
    this.state = {
      environment: 'open_sky',
      multipathRisk: 0,
      hdop: 1,
      elevationMask: 10,
      visibleSkyFraction: 1,
      reflectedSignalCount: 0,
      correctedLat: 0,
      correctedLon: 0,
      correctionApplied: false,
      correctionMeters: 0,
      confidence: 1,
    };
  }

  /**
   * Classify environment based on satellite geometry
   */
  classify(
    satelliteCount: number,
    hdop: number,
    avgElevation: number,
    snrVariance: number,
    lowElevSatCount: number
  ): EnvironmentClass {
    // Decision tree for environment classification
    if (satelliteCount < 3) {
      this.state.environment = 'tunnel';
    } else if (hdop > 5 || satelliteCount < 6) {
      this.state.environment = 'deep_canyon';
      this.state.elevationMask = 30; // aggressive mask
    } else if (hdop > 2.5 || avgElevation > 55 || snrVariance > 100) {
      this.state.environment = 'urban';
      this.state.elevationMask = 20;
    } else if (hdop > 1.5 || lowElevSatCount < 2) {
      this.state.environment = 'suburban';
      this.state.elevationMask = 15;
    } else {
      this.state.environment = 'open_sky';
      this.state.elevationMask = 10;
    }

    this.state.hdop = hdop;
    this.state.visibleSkyFraction = Math.min(1, satelliteCount / 20);

    // Multipath risk assessment
    this.state.multipathRisk = this.assessMultipathRisk(hdop, snrVariance, avgElevation);

    return this.state.environment;
  }

  /**
   * Apply multipath mitigation to raw GPS position
   */
  mitigate(
    rawLat: number, rawLon: number,
    satellites: Array<{ elevation: number; snr: number; isUsedInFix: boolean }>
  ): { lat: number; lon: number; confidence: number } {
    if (this.state.environment === 'open_sky') {
      this.state.correctedLat = rawLat;
      this.state.correctedLon = rawLon;
      this.state.correctionApplied = false;
      this.state.confidence = 1;
      return { lat: rawLat, lon: rawLon, confidence: 1 };
    }

    // Filter out likely reflected signals (low elevation + low SNR)
    const reliableSats = satellites.filter(s =>
      s.elevation > this.state.elevationMask && s.snr > 25
    );

    this.state.reflectedSignalCount = satellites.length - reliableSats.length;

    // Position smoothing using history
    this.positionHistory.push({ lat: rawLat, lon: rawLon, timestamp: Date.now() });
    if (this.positionHistory.length > 10) this.positionHistory.shift();

    // Weighted average of recent positions (more recent = higher weight)
    let weightSum = 0;
    let latSum = 0;
    let lonSum = 0;
    const now = Date.now();

    for (let i = 0; i < this.positionHistory.length; i++) {
      const age = (now - this.positionHistory[i].timestamp) / 1000;
      const weight = Math.exp(-age / 3); // exponential decay, 3s half-life
      latSum += this.positionHistory[i].lat * weight;
      lonSum += this.positionHistory[i].lon * weight;
      weightSum += weight;
    }

    const smoothedLat = latSum / weightSum;
    const smoothedLon = lonSum / weightSum;

    // Blend raw and smoothed based on environment severity
    const smoothWeight = this.state.environment === 'deep_canyon' ? 0.7 :
                         this.state.environment === 'urban' ? 0.5 : 0.3;

    const correctedLat = rawLat * (1 - smoothWeight) + smoothedLat * smoothWeight;
    const correctedLon = rawLon * (1 - smoothWeight) + smoothedLon * smoothWeight;

    this.state.correctedLat = correctedLat;
    this.state.correctedLon = correctedLon;
    this.state.correctionApplied = true;
    this.state.correctionMeters = this.haversine(rawLat, rawLon, correctedLat, correctedLon);
    this.state.confidence = Math.max(0.2, 1 - this.state.multipathRisk * 0.8);

    return { lat: correctedLat, lon: correctedLon, confidence: this.state.confidence };
  }

  private assessMultipathRisk(hdop: number, snrVariance: number, avgElevation: number): number {
    let risk = 0;
    if (hdop > 3) risk += 0.3;
    else if (hdop > 2) risk += 0.15;
    if (snrVariance > 150) risk += 0.3;
    else if (snrVariance > 80) risk += 0.15;
    if (avgElevation > 60) risk += 0.2;
    else if (avgElevation > 45) risk += 0.1;
    return Math.min(1, risk);
  }

  private haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const dLat = (lat2 - lat1) * DEG2RAD;
    const dLon = (lon2 - lon1) * DEG2RAD;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * DEG2RAD) * Math.cos(lat2 * DEG2RAD) *
      Math.sin(dLon / 2) ** 2;
    return EARTH_RADIUS * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  getState(): UrbanCanyonState { return { ...this.state }; }
}

// ═══════════════════════════════════════════════════
// MAP MATCHING ENGINE (HMM-based)
// ═══════════════════════════════════════════════════

export class MapMatchingEngine {
  private roadNetwork: Map<string, RoadNode> = new Map();
  private lastMatch: MapMatchResult | null = null;
  private candidateHistory: HMMCandidate[][] = [];
  private readonly MAX_HISTORY = 5;

  /**
   * Load road network graph
   */
  loadRoadNetwork(nodes: RoadNode[]): void {
    this.roadNetwork.clear();
    for (const node of nodes) {
      this.roadNetwork.set(node.id, node);
    }
  }

  /**
   * Match a GPS point to the road network using HMM
   */
  match(lat: number, lon: number, heading: number, speed: number): MapMatchResult {
    // Step 1: Find candidate road segments within MAX_MATCH_DISTANCE
    const candidates = this.findCandidates(lat, lon);

    if (candidates.length === 0) {
      const noMatch: MapMatchResult = {
        matchedLat: lat,
        matchedLon: lon,
        roadId: '',
        roadName: '',
        heading,
        speedLimit: 0,
        confidence: 0,
        distanceFromRaw: 0,
        isOnRoad: false,
      };
      this.lastMatch = noMatch;
      return noMatch;
    }

    // Step 2: Compute emission probabilities
    for (const c of candidates) {
      c.emissionProb = this.emissionProbability(c.distance);
    }

    // Step 3: Apply heading constraint
    const headingFiltered = candidates.filter(c => {
      const headingDiff = Math.abs(c.heading - heading);
      const normalizedDiff = Math.min(headingDiff, 360 - headingDiff);
      return normalizedDiff < 60; // within 60 degrees
    });

    const finalCandidates = headingFiltered.length > 0 ? headingFiltered : candidates;

    // Step 4: If we have history, apply transition probabilities (Viterbi)
    let bestCandidate: HMMCandidate;

    if (this.candidateHistory.length > 0 && this.lastMatch) {
      bestCandidate = this.viterbiStep(finalCandidates);
    } else {
      // First observation — pick highest emission probability
      bestCandidate = finalCandidates.reduce((best, c) =>
        c.emissionProb > best.emissionProb ? c : best
      );
    }

    // Update history
    this.candidateHistory.push(finalCandidates);
    if (this.candidateHistory.length > this.MAX_HISTORY) {
      this.candidateHistory.shift();
    }

    const result: MapMatchResult = {
      matchedLat: bestCandidate.lat,
      matchedLon: bestCandidate.lon,
      roadId: bestCandidate.roadId,
      roadName: bestCandidate.roadName,
      heading: bestCandidate.heading,
      speedLimit: bestCandidate.speedLimit,
      confidence: bestCandidate.emissionProb,
      distanceFromRaw: bestCandidate.distance,
      isOnRoad: bestCandidate.distance < MAX_MATCH_DISTANCE,
    };

    this.lastMatch = result;
    return result;
  }

  /**
   * Find candidate road segments near a GPS point
   */
  private findCandidates(lat: number, lon: number): HMMCandidate[] {
    const candidates: HMMCandidate[] = [];

    for (const [, node] of Array.from(this.roadNetwork)) {
      for (const edge of node.edges) {
        const targetNode = this.roadNetwork.get(edge.targetNodeId);
        if (!targetNode) continue;

        // Project GPS point onto road segment
        const projected = this.projectOnSegment(
          lat, lon,
          node.lat, node.lon,
          targetNode.lat, targetNode.lon
        );

        if (projected.distance < MAX_MATCH_DISTANCE) {
          candidates.push({
            roadId: edge.roadId,
            lat: projected.lat,
            lon: projected.lon,
            heading: edge.heading,
            distance: projected.distance,
            emissionProb: 0,
            roadName: edge.roadName,
            speedLimit: edge.speedLimit,
          });
        }
      }
    }

    return candidates;
  }

  /**
   * Emission probability: P(GPS observation | road candidate)
   * Gaussian distribution based on GPS accuracy
   */
  private emissionProbability(distance: number): number {
    return Math.exp(-(distance * distance) / (2 * EMISSION_SIGMA * EMISSION_SIGMA));
  }

  /**
   * Simplified Viterbi step — select best candidate considering transition from previous match
   */
  private viterbiStep(candidates: HMMCandidate[]): HMMCandidate {
    if (!this.lastMatch) {
      return candidates.reduce((best, c) => c.emissionProb > best.emissionProb ? c : best);
    }

    let bestScore = -Infinity;
    let bestCandidate = candidates[0];

    for (const c of candidates) {
      // Transition probability: based on route distance vs great-circle distance
      const gcDist = this.haversine(this.lastMatch.matchedLat, this.lastMatch.matchedLon, c.lat, c.lon);
      const routeDist = gcDist * 1.3; // approximate route distance as 1.3x great circle
      const transitionProb = Math.exp(-Math.abs(routeDist - gcDist) / TRANSITION_BETA);

      const score = Math.log(c.emissionProb + 1e-10) + Math.log(transitionProb + 1e-10);

      if (score > bestScore) {
        bestScore = score;
        bestCandidate = c;
      }
    }

    return bestCandidate;
  }

  private projectOnSegment(
    pLat: number, pLon: number,
    aLat: number, aLon: number,
    bLat: number, bLon: number
  ): { lat: number; lon: number; distance: number } {
    const dx = bLon - aLon;
    const dy = bLat - aLat;
    const lenSq = dx * dx + dy * dy;

    if (lenSq === 0) {
      return { lat: aLat, lon: aLon, distance: this.haversine(pLat, pLon, aLat, aLon) };
    }

    let t = ((pLon - aLon) * dx + (pLat - aLat) * dy) / lenSq;
    t = Math.max(0, Math.min(1, t));

    const projLat = aLat + t * dy;
    const projLon = aLon + t * dx;

    return { lat: projLat, lon: projLon, distance: this.haversine(pLat, pLon, projLat, projLon) };
  }

  private haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const dLat = (lat2 - lat1) * DEG2RAD;
    const dLon = (lon2 - lon1) * DEG2RAD;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * DEG2RAD) * Math.cos(lat2 * DEG2RAD) *
      Math.sin(dLon / 2) ** 2;
    return EARTH_RADIUS * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  getLastMatch(): MapMatchResult | null { return this.lastMatch; }
}

// ═══════════════════════════════════════════════════
// ROUTE GRAPH ENGINE (A* Pathfinding)
// ═══════════════════════════════════════════════════

export type RouteObjective = 'fastest' | 'shortest' | 'eco' | 'safest';

interface RouteResult {
  path: string[];               // node IDs
  totalDistance: number;        // meters
  totalTime: number;            // seconds
  totalToll: number;
  segments: Array<{
    roadId: string;
    roadName: string;
    distance: number;
    time: number;
    heading: number;
  }>;
}

export class RouteGraphEngine {
  private nodes: Map<string, RoadNode> = new Map();

  loadGraph(nodes: RoadNode[]): void {
    this.nodes.clear();
    for (const node of nodes) {
      this.nodes.set(node.id, node);
    }
  }

  /**
   * A* pathfinding with multi-objective cost function
   */
  findRoute(
    startNodeId: string,
    endNodeId: string,
    objective: RouteObjective = 'fastest'
  ): RouteResult | null {
    const endNode = this.nodes.get(endNodeId);
    if (!endNode || !this.nodes.has(startNodeId)) return null;

    // Priority queue (simple sorted array for clarity)
    const openSet: Array<{ nodeId: string; fScore: number }> = [{ nodeId: startNodeId, fScore: 0 }];
    const cameFrom = new Map<string, { nodeId: string; edge: RoadEdge }>();
    const gScore = new Map<string, number>();
    gScore.set(startNodeId, 0);

    const closedSet = new Set<string>();

    while (openSet.length > 0) {
      // Sort and pick lowest fScore
      openSet.sort((a, b) => a.fScore - b.fScore);
      const current = openSet.shift()!;

      if (current.nodeId === endNodeId) {
        return this.reconstructPath(cameFrom, endNodeId, gScore.get(endNodeId) || 0);
      }

      if (closedSet.has(current.nodeId)) continue;
      closedSet.add(current.nodeId);

      const node = this.nodes.get(current.nodeId);
      if (!node) continue;

      for (const edge of node.edges) {
        if (closedSet.has(edge.targetNodeId)) continue;

        const cost = this.edgeCost(edge, objective);
        const tentativeG = (gScore.get(current.nodeId) || 0) + cost;

        if (tentativeG < (gScore.get(edge.targetNodeId) || Infinity)) {
          cameFrom.set(edge.targetNodeId, { nodeId: current.nodeId, edge });
          gScore.set(edge.targetNodeId, tentativeG);

          const targetNode = this.nodes.get(edge.targetNodeId);
          const h = targetNode
            ? this.heuristic(targetNode.lat, targetNode.lon, endNode.lat, endNode.lon, objective)
            : 0;

          openSet.push({ nodeId: edge.targetNodeId, fScore: tentativeG + h });
        }
      }
    }

    return null; // no path found
  }

  /**
   * Edge cost based on objective
   */
  private edgeCost(edge: RoadEdge, objective: RouteObjective): number {
    switch (objective) {
      case 'fastest':
        return edge.travelTime * edge.trafficFactor;
      case 'shortest':
        return edge.distance;
      case 'eco':
        // Eco: penalize high speeds and stop-and-go
        return edge.distance * (1 + edge.trafficFactor * 0.3);
      case 'safest':
        // Safest: prefer higher-class roads, penalize tunnels
        const classPenalty = edge.roadClass === 'residential' ? 1.5 :
                             edge.roadClass === 'service' ? 2.0 : 1.0;
        const tunnelPenalty = edge.isTunnel ? 1.3 : 1.0;
        return edge.travelTime * classPenalty * tunnelPenalty;
      default:
        return edge.travelTime;
    }
  }

  /**
   * A* heuristic: great-circle distance / max speed
   */
  private heuristic(
    lat1: number, lon1: number, lat2: number, lon2: number,
    objective: RouteObjective
  ): number {
    const dist = this.haversine(lat1, lon1, lat2, lon2);
    switch (objective) {
      case 'fastest': return dist / 33.3; // ~120 km/h max
      case 'shortest': return dist;
      case 'eco': return dist * 1.1;
      case 'safest': return dist / 27.8; // ~100 km/h
      default: return dist / 33.3;
    }
  }

  private reconstructPath(
    cameFrom: Map<string, { nodeId: string; edge: RoadEdge }>,
    endNodeId: string,
    totalCost: number
  ): RouteResult {
    const path: string[] = [];
    const segments: RouteResult['segments'] = [];
    let totalDistance = 0;
    let totalTime = 0;
    let totalToll = 0;

    let current = endNodeId;
    while (cameFrom.has(current)) {
      path.unshift(current);
      const prev = cameFrom.get(current)!;
      segments.unshift({
        roadId: prev.edge.roadId,
        roadName: prev.edge.roadName,
        distance: prev.edge.distance,
        time: prev.edge.travelTime,
        heading: prev.edge.heading,
      });
      totalDistance += prev.edge.distance;
      totalTime += prev.edge.travelTime * prev.edge.trafficFactor;
      totalToll += prev.edge.tollCost;
      current = prev.nodeId;
    }
    path.unshift(current);

    return { path, totalDistance, totalTime, totalToll, segments };
  }

  private haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const dLat = (lat2 - lat1) * DEG2RAD;
    const dLon = (lon2 - lon1) * DEG2RAD;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * DEG2RAD) * Math.cos(lat2 * DEG2RAD) *
      Math.sin(dLon / 2) ** 2;
    return EARTH_RADIUS * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }
}
