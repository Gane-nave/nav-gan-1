/**
 * G.A.N.E — Offline-First Storage Engine
 * =========================================
 * IndexedDB-based local caching with CRDT synchronization.
 * Ensures navigation continues seamlessly during network drops.
 *
 * Features:
 * - Offline map tile caching
 * - Route & waypoint persistence
 * - Telemetry buffering for later sync
 * - CRDT-based conflict resolution (LWW-Register + G-Counter)
 * - Automatic sync on reconnection
 */

// ─── CRDT Types ───
export interface LWWRegister<T> {
  value: T;
  timestamp: number;
  nodeId: string;
}

export interface GCounter {
  counts: Record<string, number>;
}

export interface CRDTDocument {
  id: string;
  type: string;
  data: LWWRegister<unknown>;
  version: GCounter;
  synced: boolean;
}

// ─── CRDT Operations ───
export function lwwMerge<T>(local: LWWRegister<T>, remote: LWWRegister<T>): LWWRegister<T> {
  if (remote.timestamp > local.timestamp) return remote;
  if (remote.timestamp === local.timestamp && remote.nodeId > local.nodeId) return remote;
  return local;
}

export function gCounterIncrement(counter: GCounter, nodeId: string): GCounter {
  const newCounts = { ...counter.counts };
  newCounts[nodeId] = (newCounts[nodeId] || 0) + 1;
  return { counts: newCounts };
}

export function gCounterMerge(a: GCounter, b: GCounter): GCounter {
  const merged: Record<string, number> = { ...a.counts };
  for (const [node, count] of Object.entries(b.counts)) {
    merged[node] = Math.max(merged[node] || 0, count);
  }
  return { counts: merged };
}

export function gCounterValue(counter: GCounter): number {
  return Object.values(counter.counts).reduce((sum, v) => sum + v, 0);
}

// ─── IndexedDB Store Names ───
const DB_NAME = 'gane-offline-store';
const DB_VERSION = 3;

const STORES = {
  MAP_TILES: 'map_tiles',
  ROUTES: 'routes',
  WAYPOINTS: 'waypoints',
  TELEMETRY_BUFFER: 'telemetry_buffer',
  ANOMALIES: 'anomalies',
  SETTINGS: 'settings',
  SYNC_QUEUE: 'sync_queue',
} as const;

// ─── Offline Store ───
export class OfflineStore {
  private db: IDBDatabase | null = null;
  private nodeId: string;
  private syncInProgress = false;
  private pendingSyncCount = 0;

  constructor() {
    this.nodeId = this.getNodeId();
  }

  /** Open/create the IndexedDB database */
  async init(): Promise<void> {
    return new Promise((resolve, reject) => {
      if (typeof indexedDB === 'undefined') {
        reject(new Error('IndexedDB not available'));
        return;
      }

      const request = indexedDB.open(DB_NAME, DB_VERSION);

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;

        // Map tiles store (cached for offline use)
        if (!db.objectStoreNames.contains(STORES.MAP_TILES)) {
          const tileStore = db.createObjectStore(STORES.MAP_TILES, { keyPath: 'key' });
          tileStore.createIndex('timestamp', 'timestamp');
          tileStore.createIndex('zoom', 'zoom');
        }

        // Routes store
        if (!db.objectStoreNames.contains(STORES.ROUTES)) {
          const routeStore = db.createObjectStore(STORES.ROUTES, { keyPath: 'id' });
          routeStore.createIndex('timestamp', 'timestamp');
          routeStore.createIndex('synced', 'synced');
        }

        // Waypoints store
        if (!db.objectStoreNames.contains(STORES.WAYPOINTS)) {
          const wpStore = db.createObjectStore(STORES.WAYPOINTS, { keyPath: 'id' });
          wpStore.createIndex('synced', 'synced');
        }

        // Telemetry buffer (for offline telemetry)
        if (!db.objectStoreNames.contains(STORES.TELEMETRY_BUFFER)) {
          const telStore = db.createObjectStore(STORES.TELEMETRY_BUFFER, { keyPath: 'id', autoIncrement: true });
          telStore.createIndex('timestamp', 'timestamp');
        }

        // Anomalies (crowdsourced reports)
        if (!db.objectStoreNames.contains(STORES.ANOMALIES)) {
          const anomStore = db.createObjectStore(STORES.ANOMALIES, { keyPath: 'id' });
          anomStore.createIndex('synced', 'synced');
          anomStore.createIndex('lat_lon', ['lat', 'lon']);
        }

        // Settings
        if (!db.objectStoreNames.contains(STORES.SETTINGS)) {
          db.createObjectStore(STORES.SETTINGS, { keyPath: 'key' });
        }

        // Sync queue
        if (!db.objectStoreNames.contains(STORES.SYNC_QUEUE)) {
          const syncStore = db.createObjectStore(STORES.SYNC_QUEUE, { keyPath: 'id', autoIncrement: true });
          syncStore.createIndex('timestamp', 'timestamp');
          syncStore.createIndex('type', 'type');
        }
      };

      request.onsuccess = (event) => {
        this.db = (event.target as IDBOpenDBRequest).result;
        resolve();
      };

      request.onerror = () => {
        reject(request.error);
      };
    });
  }

  // ─── Map Tile Caching ───

  /** Cache a map tile for offline use */
  async cacheTile(key: string, zoom: number, data: ArrayBuffer): Promise<void> {
    await this.put(STORES.MAP_TILES, {
      key,
      zoom,
      data,
      timestamp: Date.now(),
      size: data.byteLength,
    });
  }

  /** Get a cached map tile */
  async getTile(key: string): Promise<ArrayBuffer | null> {
    const record = await this.get(STORES.MAP_TILES, key);
    return (record?.data as ArrayBuffer) ?? null;
  }

  /** Pre-cache tiles for an area (smart download) */
  async preCacheArea(centerLat: number, centerLon: number, radiusKm: number, maxZoom: number = 15): Promise<number> {
    // Calculate tile coordinates for the area
    let cachedCount = 0;
    const minZoom = 10;

    for (let zoom = minZoom; zoom <= maxZoom; zoom++) {
      const tilesPerDegree = Math.pow(2, zoom) / 360;
      const latRange = radiusKm / 111; // ~111km per degree
      const lonRange = radiusKm / (111 * Math.cos(centerLat * Math.PI / 180));

      const minTileX = Math.floor((centerLon - lonRange + 180) * tilesPerDegree);
      const maxTileX = Math.ceil((centerLon + lonRange + 180) * tilesPerDegree);
      const minTileY = Math.floor((90 - centerLat - latRange) * tilesPerDegree);
      const maxTileY = Math.ceil((90 - centerLat + latRange) * tilesPerDegree);

      for (let x = minTileX; x <= maxTileX; x++) {
        for (let y = minTileY; y <= maxTileY; y++) {
          const key = `${zoom}/${x}/${y}`;
          const existing = await this.getTile(key);
          if (!existing) {
            // Queue for download (actual download handled by map component)
            await this.addToSyncQueue('tile_download', { key, zoom, x, y });
            cachedCount++;
          }
        }
      }
    }

    return cachedCount;
  }

  // ─── Telemetry Buffer ───

  /** Buffer telemetry data when offline */
  async bufferTelemetry(packet: Record<string, unknown>): Promise<void> {
    await this.put(STORES.TELEMETRY_BUFFER, {
      ...packet,
      timestamp: Date.now(),
    });
    this.pendingSyncCount++;
  }

  /** Get all buffered telemetry for sync */
  async getBufferedTelemetry(): Promise<Record<string, unknown>[]> {
    return this.getAll(STORES.TELEMETRY_BUFFER);
  }

  /** Clear synced telemetry */
  async clearSyncedTelemetry(ids: number[]): Promise<void> {
    for (const id of ids) {
      await this.delete(STORES.TELEMETRY_BUFFER, id);
    }
    this.pendingSyncCount = Math.max(0, this.pendingSyncCount - ids.length);
  }

  // ─── Anomaly Reports ───

  /** Store an anomaly report (with CRDT) */
  async reportAnomaly(anomaly: {
    id: string;
    type: string;
    lat: number;
    lon: number;
    description: string;
    severity: number;
  }): Promise<void> {
    const doc: CRDTDocument = {
      id: anomaly.id,
      type: 'anomaly',
      data: {
        value: anomaly,
        timestamp: Date.now(),
        nodeId: this.nodeId,
      },
      version: gCounterIncrement({ counts: {} }, this.nodeId),
      synced: false,
    };

    await this.put(STORES.ANOMALIES, doc as unknown as Record<string, unknown>);
    await this.addToSyncQueue('anomaly_report', anomaly);
  }

  /** Get nearby anomalies from local cache */
  async getNearbyAnomalies(lat: number, lon: number, radiusM: number): Promise<CRDTDocument[]> {
    const all = (await this.getAll(STORES.ANOMALIES)) as unknown as CRDTDocument[];
    return all.filter(doc => {
      const anomaly = doc.data.value as { lat: number; lon: number };
      const dist = this.haversineDistance(lat, lon, anomaly.lat, anomaly.lon);
      return dist <= radiusM;
    });
  }

  // ─── Route Caching ───

  /** Cache a route for offline use */
  async cacheRoute(route: Record<string, unknown>): Promise<void> {
    await this.put(STORES.ROUTES, {
      ...route,
      timestamp: Date.now(),
      synced: true,
    });
  }

  /** Get cached routes */
  async getCachedRoutes(): Promise<Record<string, unknown>[]> {
    return this.getAll(STORES.ROUTES);
  }

  // ─── Settings ───

  async setSetting(key: string, value: unknown): Promise<void> {
    await this.put(STORES.SETTINGS, { key, value, timestamp: Date.now() });
  }

  async getSetting<T>(key: string): Promise<T | null> {
    const record = await this.get(STORES.SETTINGS, key);
    return (record?.value as T) ?? null;
  }

  // ─── Sync Queue ───

  /** Add item to sync queue */
  async addToSyncQueue(type: string, data: unknown): Promise<void> {
    await this.put(STORES.SYNC_QUEUE, {
      type,
      data,
      timestamp: Date.now(),
      retries: 0,
    });
  }

  /** Process sync queue (called when online) */
  async processSyncQueue(syncFn: (type: string, data: unknown) => Promise<boolean>): Promise<number> {
    if (this.syncInProgress) return 0;
    this.syncInProgress = true;

    let synced = 0;
    try {
      const items = await this.getAll(STORES.SYNC_QUEUE) as Array<{
        id: number;
        type: string;
        data: unknown;
        retries: number;
      }>;

      for (const item of items) {
        try {
          const success = await syncFn(item.type, item.data);
          if (success) {
            await this.delete(STORES.SYNC_QUEUE, item.id);
            synced++;
          } else {
            // Increment retry count
            await this.put(STORES.SYNC_QUEUE, { ...item, retries: item.retries + 1 });
          }
        } catch {
          // Keep in queue for next sync attempt
        }
      }
    } finally {
      this.syncInProgress = false;
    }

    return synced;
  }

  /** Get pending sync count */
  getPendingSyncCount(): number {
    return this.pendingSyncCount;
  }

  // ─── Storage Stats ───

  async getStorageStats(): Promise<{
    tileCount: number;
    routeCount: number;
    bufferCount: number;
    anomalyCount: number;
    syncQueueCount: number;
    estimatedSizeMB: number;
  }> {
    const tiles = await this.count(STORES.MAP_TILES);
    const routes = await this.count(STORES.ROUTES);
    const buffer = await this.count(STORES.TELEMETRY_BUFFER);
    const anomalies = await this.count(STORES.ANOMALIES);
    const syncQueue = await this.count(STORES.SYNC_QUEUE);

    // Estimate storage usage
    const estimate = await navigator.storage?.estimate?.();
    const usageMB = (estimate?.usage ?? 0) / (1024 * 1024);

    return {
      tileCount: tiles,
      routeCount: routes,
      bufferCount: buffer,
      anomalyCount: anomalies,
      syncQueueCount: syncQueue,
      estimatedSizeMB: Math.round(usageMB * 100) / 100,
    };
  }

  // ─── Generic IndexedDB Operations ───

  private async put(storeName: string, data: Record<string, unknown> | object): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!this.db) { reject(new Error('DB not initialized')); return; }
      const tx = this.db.transaction(storeName, 'readwrite');
      tx.objectStore(storeName).put(data);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }

  private async get(storeName: string, key: string | number): Promise<Record<string, unknown> | null> {
    return new Promise((resolve, reject) => {
      if (!this.db) { reject(new Error('DB not initialized')); return; }
      const tx = this.db.transaction(storeName, 'readonly');
      const req = tx.objectStore(storeName).get(key);
      req.onsuccess = () => resolve(req.result ?? null);
      req.onerror = () => reject(req.error);
    });
  }

  private async getAll(storeName: string): Promise<Record<string, unknown>[]> {
    return new Promise((resolve, reject) => {
      if (!this.db) { reject(new Error('DB not initialized')); return; }
      const tx = this.db.transaction(storeName, 'readonly');
      const req = tx.objectStore(storeName).getAll();
      req.onsuccess = () => resolve(req.result ?? []);
      req.onerror = () => reject(req.error);
    });
  }

  private async delete(storeName: string, key: string | number): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!this.db) { reject(new Error('DB not initialized')); return; }
      const tx = this.db.transaction(storeName, 'readwrite');
      tx.objectStore(storeName).delete(key);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }

  private async count(storeName: string): Promise<number> {
    return new Promise((resolve, reject) => {
      if (!this.db) { reject(new Error('DB not initialized')); return; }
      const tx = this.db.transaction(storeName, 'readonly');
      const req = tx.objectStore(storeName).count();
      req.onsuccess = () => resolve(req.result);
      req.onerror = () => reject(req.error);
    });
  }

  // ─── Helpers ───

  private getNodeId(): string {
    if (typeof localStorage === 'undefined') return 'node-' + Math.random().toString(36).slice(2);
    let id = localStorage.getItem('gane-node-id');
    if (!id) {
      id = 'node-' + Date.now().toString(36) + '-' + Math.random().toString(36).slice(2);
      localStorage.setItem('gane-node-id', id);
    }
    return id;
  }

  private haversineDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const R = 6371000;
    const toRad = Math.PI / 180;
    const dLat = (lat2 - lat1) * toRad;
    const dLon = (lon2 - lon1) * toRad;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * toRad) * Math.cos(lat2 * toRad) * Math.sin(dLon / 2) ** 2;
    return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  /** Close database connection */
  close(): void {
    this.db?.close();
    this.db = null;
  }
}

// ─── Singleton ───
let _store: OfflineStore | null = null;

export function getOfflineStore(): OfflineStore {
  if (!_store) {
    _store = new OfflineStore();
  }
  return _store;
}
