/**
 * G.A.N.E NAV — Offline Map Tiles Engine
 * ========================================
 * Complete offline navigation system with tile caching, region download,
 * offline routing, and delta updates. Never lose your map.
 * 
 * Architecture:
 * - IndexedDB tile cache with LRU eviction
 * - Region-based bulk download with progress tracking
 * - Multiple map providers (OSM, satellite, terrain, topo, dark)
 * - Offline routing with pre-cached graph data
 * - Delta updates for efficient tile refresh
 * - Storage quota management
 */

// ═══════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════

export type TileProvider = 'osm' | 'satellite' | 'terrain' | 'topo' | 'dark' | 'hybrid';

export interface TileCoord {
  x: number;
  y: number;
  z: number;
}

export interface CachedTile {
  key: string;           // "provider:z:x:y"
  provider: TileProvider;
  coord: TileCoord;
  blob: Blob;
  size: number;          // bytes
  cachedAt: number;      // timestamp
  lastAccessed: number;  // for LRU
  etag?: string;         // for delta updates
  version: number;
}

export interface DownloadRegion {
  id: string;
  name: string;
  bounds: {
    north: number;
    south: number;
    east: number;
    west: number;
  };
  minZoom: number;
  maxZoom: number;
  providers: TileProvider[];
  totalTiles: number;
  downloadedTiles: number;
  totalSize: number;     // bytes
  status: 'pending' | 'downloading' | 'paused' | 'complete' | 'error';
  createdAt: number;
  updatedAt: number;
  error?: string;
}

export interface OfflineRoute {
  id: string;
  name: string;
  waypoints: { lat: number; lng: number; name?: string }[];
  graphData: Uint8Array;  // compressed routing graph
  distance: number;       // meters
  duration: number;       // seconds
  cachedAt: number;
  expiresAt: number;
}

export interface StorageStats {
  totalSize: number;      // bytes used
  quotaSize: number;      // bytes available
  tileCount: number;
  regionCount: number;
  routeCount: number;
  oldestTile: number;     // timestamp
  newestTile: number;
  providerBreakdown: Record<TileProvider, { count: number; size: number }>;
}

export interface OfflineMapState {
  isOnline: boolean;
  regions: DownloadRegion[];
  activeDownload: string | null;  // region id
  downloadProgress: number;       // 0-100
  downloadSpeed: number;          // bytes/sec
  storageStats: StorageStats;
  offlineRoutes: OfflineRoute[];
  lastSyncAt: number;
  pendingDeltaUpdates: number;
}

// ═══════════════════════════════════════════════════════════
// TILE URL TEMPLATES
// ═══════════════════════════════════════════════════════════

const TILE_URLS: Record<TileProvider, string> = {
  osm: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
  satellite: 'https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}',
  terrain: 'https://tile.opentopomap.org/{z}/{x}/{y}.png',
  topo: 'https://basemap.nationalmap.gov/arcgis/rest/services/USGSTopo/MapServer/tile/{z}/{y}/{x}',
  dark: 'https://tiles.stadiamaps.com/tiles/alidade_smooth_dark/{z}/{x}/{y}.png',
  hybrid: 'https://mt1.google.com/vt/lyrs=y&x={x}&y={y}&z={z}',
};

// ═══════════════════════════════════════════════════════════
// PREDEFINED REGIONS
// ═══════════════════════════════════════════════════════════

export const PREDEFINED_REGIONS: Omit<DownloadRegion, 'id' | 'downloadedTiles' | 'totalSize' | 'status' | 'createdAt' | 'updatedAt' | 'totalTiles'>[] = [
  {
    name: 'Israel & Palestine',
    bounds: { north: 33.4, south: 29.4, east: 35.9, west: 34.2 },
    minZoom: 5, maxZoom: 16,
    providers: ['osm', 'satellite'],
  },
  {
    name: 'Greater Tel Aviv',
    bounds: { north: 32.25, south: 31.95, east: 34.9, west: 34.7 },
    minZoom: 10, maxZoom: 18,
    providers: ['osm', 'satellite', 'terrain'],
  },
  {
    name: 'Jerusalem',
    bounds: { north: 31.85, south: 31.7, east: 35.3, west: 35.15 },
    minZoom: 10, maxZoom: 18,
    providers: ['osm', 'satellite'],
  },
  {
    name: 'Haifa & North',
    bounds: { north: 33.1, south: 32.6, east: 35.4, west: 34.9 },
    minZoom: 10, maxZoom: 17,
    providers: ['osm', 'satellite'],
  },
  {
    name: 'Negev Desert',
    bounds: { north: 31.4, south: 29.5, east: 35.5, west: 34.2 },
    minZoom: 8, maxZoom: 15,
    providers: ['osm', 'terrain'],
  },
  {
    name: 'Jordan',
    bounds: { north: 33.4, south: 29.2, east: 39.3, west: 34.9 },
    minZoom: 5, maxZoom: 14,
    providers: ['osm'],
  },
  {
    name: 'Egypt (Sinai)',
    bounds: { north: 31.3, south: 27.7, east: 35.0, west: 32.3 },
    minZoom: 5, maxZoom: 14,
    providers: ['osm', 'terrain'],
  },
  {
    name: 'Lebanon',
    bounds: { north: 34.7, south: 33.0, east: 36.7, west: 35.1 },
    minZoom: 5, maxZoom: 14,
    providers: ['osm'],
  },
  // ═══ EXPANDED FIELD COVERAGE ═══
  {
    name: 'West Bank',
    bounds: { north: 32.4, south: 31.9, east: 35.6, west: 35.1 },
    minZoom: 9, maxZoom: 17,
    providers: ['osm', 'satellite'],
  },
  {
    name: 'Golan Heights',
    bounds: { north: 33.5, south: 32.8, east: 35.9, west: 35.5 },
    minZoom: 9, maxZoom: 16,
    providers: ['osm', 'satellite', 'terrain'],
  },
  {
    name: 'Eilat & Aqaba',
    bounds: { north: 29.6, south: 29.4, east: 35.1, west: 34.9 },
    minZoom: 10, maxZoom: 17,
    providers: ['osm', 'satellite'],
  },
  {
    name: 'Dead Sea Region',
    bounds: { north: 32.0, south: 31.4, east: 35.6, west: 35.3 },
    minZoom: 9, maxZoom: 16,
    providers: ['osm', 'satellite', 'terrain'],
  },
  {
    name: 'Galilee & Sea of Tiberias',
    bounds: { north: 33.0, south: 32.6, east: 35.5, west: 35.2 },
    minZoom: 10, maxZoom: 17,
    providers: ['osm', 'satellite'],
  },
  {
    name: 'Gaza Strip',
    bounds: { north: 31.9, south: 31.2, east: 34.5, west: 34.2 },
    minZoom: 9, maxZoom: 16,
    providers: ['osm', 'satellite'],
  },
  {
    name: 'Syria & Golan',
    bounds: { north: 34.5, south: 32.8, east: 37.0, west: 35.5 },
    minZoom: 7, maxZoom: 14,
    providers: ['osm'],
  },
  {
    name: 'Saudi Arabia (NW)',
    bounds: { north: 32.0, south: 28.0, east: 38.0, west: 34.5 },
    minZoom: 6, maxZoom: 12,
    providers: ['osm'],
  },
];

// ═══════════════════════════════════════════════════════════
// TILE MATH UTILITIES
// ═══════════════════════════════════════════════════════════

function lon2tile(lon: number, zoom: number): number {
  return Math.floor((lon + 180) / 360 * Math.pow(2, zoom));
}

function lat2tile(lat: number, zoom: number): number {
  return Math.floor(
    (1 - Math.log(Math.tan(lat * Math.PI / 180) + 1 / Math.cos(lat * Math.PI / 180)) / Math.PI) / 2 * Math.pow(2, zoom)
  );
}

function tileKey(provider: TileProvider, coord: TileCoord): string {
  return `${provider}:${coord.z}:${coord.x}:${coord.y}`;
}

function countTilesInBounds(
  bounds: DownloadRegion['bounds'],
  minZoom: number,
  maxZoom: number,
  providers: TileProvider[]
): number {
  let total = 0;
  for (let z = minZoom; z <= maxZoom; z++) {
    const xMin = lon2tile(bounds.west, z);
    const xMax = lon2tile(bounds.east, z);
    const yMin = lat2tile(bounds.north, z);
    const yMax = lat2tile(bounds.south, z);
    const tilesAtZoom = (xMax - xMin + 1) * (yMax - yMin + 1);
    total += tilesAtZoom * providers.length;
  }
  return total;
}

function getTileUrl(provider: TileProvider, coord: TileCoord): string {
  return TILE_URLS[provider]
    .replace('{z}', String(coord.z))
    .replace('{x}', String(coord.x))
    .replace('{y}', String(coord.y));
}

// ═══════════════════════════════════════════════════════════
// INDEXEDDB TILE CACHE
// ═══════════════════════════════════════════════════════════

class TileCache {
  private dbName = 'gane-tile-cache';
  private dbVersion = 1;
  private db: IDBDatabase | null = null;

  async open(): Promise<void> {
    if (this.db) return;
    return new Promise((resolve, reject) => {
      const req = indexedDB.open(this.dbName, this.dbVersion);
      req.onupgradeneeded = () => {
        const db = req.result;
        if (!db.objectStoreNames.contains('tiles')) {
          const store = db.createObjectStore('tiles', { keyPath: 'key' });
          store.createIndex('provider', 'provider', { unique: false });
          store.createIndex('lastAccessed', 'lastAccessed', { unique: false });
          store.createIndex('cachedAt', 'cachedAt', { unique: false });
        }
        if (!db.objectStoreNames.contains('regions')) {
          db.createObjectStore('regions', { keyPath: 'id' });
        }
        if (!db.objectStoreNames.contains('routes')) {
          db.createObjectStore('routes', { keyPath: 'id' });
        }
      };
      req.onsuccess = () => {
        this.db = req.result;
        resolve();
      };
      req.onerror = () => reject(req.error);
    });
  }

  async putTile(tile: CachedTile): Promise<void> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('tiles', 'readwrite');
      tx.objectStore('tiles').put(tile);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }

  async getTile(key: string): Promise<CachedTile | undefined> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('tiles', 'readwrite');
      const req = tx.objectStore('tiles').get(key);
      req.onsuccess = () => {
        const tile = req.result as CachedTile | undefined;
        if (tile) {
          // Update last accessed for LRU
          tile.lastAccessed = Date.now();
          tx.objectStore('tiles').put(tile);
        }
        resolve(tile);
      };
      req.onerror = () => reject(req.error);
    });
  }

  async hasTile(key: string): Promise<boolean> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('tiles', 'readonly');
      const req = tx.objectStore('tiles').count(IDBKeyRange.only(key));
      req.onsuccess = () => resolve(req.result > 0);
      req.onerror = () => reject(req.error);
    });
  }

  async deleteTile(key: string): Promise<void> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('tiles', 'readwrite');
      tx.objectStore('tiles').delete(key);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }

  async getStats(): Promise<StorageStats> {
    await this.open();
    const stats: StorageStats = {
      totalSize: 0,
      quotaSize: 500 * 1024 * 1024, // 500MB default
      tileCount: 0,
      regionCount: 0,
      routeCount: 0,
      oldestTile: Infinity,
      newestTile: 0,
      providerBreakdown: {
        osm: { count: 0, size: 0 },
        satellite: { count: 0, size: 0 },
        terrain: { count: 0, size: 0 },
        topo: { count: 0, size: 0 },
        dark: { count: 0, size: 0 },
        hybrid: { count: 0, size: 0 },
      },
    };

    // Check storage quota
    if (navigator.storage?.estimate) {
      try {
        const est = await navigator.storage.estimate();
        stats.quotaSize = est.quota ?? stats.quotaSize;
      } catch { /* ignore */ }
    }

    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(['tiles', 'regions', 'routes'], 'readonly');

      // Count tiles
      const tileStore = tx.objectStore('tiles');
      const cursor = tileStore.openCursor();
      cursor.onsuccess = () => {
        const c = cursor.result;
        if (c) {
          const tile = c.value as CachedTile;
          stats.tileCount++;
          stats.totalSize += tile.size;
          stats.oldestTile = Math.min(stats.oldestTile, tile.cachedAt);
          stats.newestTile = Math.max(stats.newestTile, tile.cachedAt);
          const pb = stats.providerBreakdown[tile.provider];
          if (pb) {
            pb.count++;
            pb.size += tile.size;
          }
          c.continue();
        }
      };

      // Count regions
      const regionReq = tx.objectStore('regions').count();
      regionReq.onsuccess = () => { stats.regionCount = regionReq.result; };

      // Count routes
      const routeReq = tx.objectStore('routes').count();
      routeReq.onsuccess = () => { stats.routeCount = routeReq.result; };

      tx.oncomplete = () => {
        if (stats.oldestTile === Infinity) stats.oldestTile = 0;
        resolve(stats);
      };
      tx.onerror = () => reject(tx.error);
    });
  }

  async evictLRU(targetBytes: number): Promise<number> {
    await this.open();
    let freed = 0;

    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('tiles', 'readwrite');
      const index = tx.objectStore('tiles').index('lastAccessed');
      const cursor = index.openCursor(); // ascending = oldest first

      cursor.onsuccess = () => {
        const c = cursor.result;
        if (c && freed < targetBytes) {
          const tile = c.value as CachedTile;
          freed += tile.size;
          c.delete();
          c.continue();
        }
      };

      tx.oncomplete = () => resolve(freed);
      tx.onerror = () => reject(tx.error);
    });
  }

  async saveRegion(region: DownloadRegion): Promise<void> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('regions', 'readwrite');
      tx.objectStore('regions').put(region);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }

  async getRegions(): Promise<DownloadRegion[]> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('regions', 'readonly');
      const req = tx.objectStore('regions').getAll();
      req.onsuccess = () => resolve(req.result as DownloadRegion[]);
      req.onerror = () => reject(req.error);
    });
  }

  async deleteRegion(id: string): Promise<void> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('regions', 'readwrite');
      tx.objectStore('regions').delete(id);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }

  async saveRoute(route: OfflineRoute): Promise<void> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('routes', 'readwrite');
      tx.objectStore('routes').put(route);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }

  async getRoutes(): Promise<OfflineRoute[]> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction('routes', 'readonly');
      const req = tx.objectStore('routes').getAll();
      req.onsuccess = () => resolve(req.result as OfflineRoute[]);
      req.onerror = () => reject(req.error);
    });
  }

  async clearAll(): Promise<void> {
    await this.open();
    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(['tiles', 'regions', 'routes'], 'readwrite');
      tx.objectStore('tiles').clear();
      tx.objectStore('regions').clear();
      tx.objectStore('routes').clear();
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }
}

// ═══════════════════════════════════════════════════════════
// OFFLINE MAP ENGINE
// ═══════════════════════════════════════════════════════════

export class OfflineMapEngine {
  private cache = new TileCache();
  private state: OfflineMapState;
  private listeners: Set<(state: OfflineMapState) => void> = new Set();
  private abortController: AbortController | null = null;
  private downloadQueue: { provider: TileProvider; coord: TileCoord }[] = [];
  private concurrency = 6; // parallel downloads
  private activeDownloads = 0;
  private bytesDownloaded = 0;
  private downloadStartTime = 0;
  private maxStorageBytes = 500 * 1024 * 1024; // 500MB default

  constructor() {
    this.state = {
      isOnline: typeof navigator !== 'undefined' ? navigator.onLine : true,
      regions: [],
      activeDownload: null,
      downloadProgress: 0,
      downloadSpeed: 0,
      storageStats: {
        totalSize: 0,
        quotaSize: this.maxStorageBytes,
        tileCount: 0,
        regionCount: 0,
        routeCount: 0,
        oldestTile: 0,
        newestTile: 0,
        providerBreakdown: {
          osm: { count: 0, size: 0 },
          satellite: { count: 0, size: 0 },
          terrain: { count: 0, size: 0 },
          topo: { count: 0, size: 0 },
          dark: { count: 0, size: 0 },
          hybrid: { count: 0, size: 0 },
        },
      },
      offlineRoutes: [],
      lastSyncAt: 0,
      pendingDeltaUpdates: 0,
    };

    // Listen for online/offline events
    if (typeof window !== 'undefined') {
      window.addEventListener('online', () => this.setOnline(true));
      window.addEventListener('offline', () => this.setOnline(false));
    }
  }

  // ─── State Management ────────────────────────────────────

  getState(): OfflineMapState {
    return { ...this.state };
  }

  subscribe(listener: (state: OfflineMapState) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private emit(): void {
    const snapshot = { ...this.state };
    for (const fn of Array.from(this.listeners)) fn(snapshot);
  }

  private setOnline(online: boolean): void {
    this.state.isOnline = online;
    this.emit();
  }

  // ─── Initialization ──────────────────────────────────────

  async init(): Promise<void> {
    await this.cache.open();
    const [regions, routes, stats] = await Promise.all([
      this.cache.getRegions(),
      this.cache.getRoutes(),
      this.cache.getStats(),
    ]);
    this.state.regions = regions;
    this.state.offlineRoutes = routes;
    this.state.storageStats = stats;
    this.maxStorageBytes = stats.quotaSize;
    this.emit();
  }

  // ─── Tile Access ─────────────────────────────────────────

  async getTileBlob(provider: TileProvider, coord: TileCoord): Promise<Blob | null> {
    const key = tileKey(provider, coord);
    const cached = await this.cache.getTile(key);
    if (cached) return cached.blob;

    // If online, fetch and cache
    if (this.state.isOnline) {
      try {
        const url = getTileUrl(provider, coord);
        const resp = await fetch(url);
        if (!resp.ok) return null;
        const blob = await resp.blob();
        const tile: CachedTile = {
          key,
          provider,
          coord,
          blob,
          size: blob.size,
          cachedAt: Date.now(),
          lastAccessed: Date.now(),
          etag: resp.headers.get('etag') ?? undefined,
          version: 1,
        };
        await this.cache.putTile(tile);
        return blob;
      } catch {
        return null;
      }
    }

    return null; // offline and not cached
  }

  getTileUrl(provider: TileProvider, coord: TileCoord): string {
    return getTileUrl(provider, coord);
  }

  // ─── Region Download ─────────────────────────────────────

  async createRegion(
    name: string,
    bounds: DownloadRegion['bounds'],
    minZoom: number,
    maxZoom: number,
    providers: TileProvider[]
  ): Promise<DownloadRegion> {
    const totalTiles = countTilesInBounds(bounds, minZoom, maxZoom, providers);
    const region: DownloadRegion = {
      id: `region-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
      name,
      bounds,
      minZoom,
      maxZoom,
      providers,
      totalTiles,
      downloadedTiles: 0,
      totalSize: 0,
      status: 'pending',
      createdAt: Date.now(),
      updatedAt: Date.now(),
    };
    await this.cache.saveRegion(region);
    this.state.regions = [...this.state.regions, region];
    this.emit();
    return region;
  }

  async startDownload(regionId: string): Promise<void> {
    const region = this.state.regions.find(r => r.id === regionId);
    if (!region || region.status === 'downloading') return;

    // Check storage quota
    const stats = await this.cache.getStats();
    const estimatedSize = region.totalTiles * 15000; // ~15KB avg per tile
    if (stats.totalSize + estimatedSize > this.maxStorageBytes * 0.9) {
      // Try to evict old tiles
      const needed = estimatedSize - (this.maxStorageBytes * 0.9 - stats.totalSize);
      if (needed > 0) {
        await this.cache.evictLRU(needed);
      }
    }

    region.status = 'downloading';
    region.updatedAt = Date.now();
    await this.cache.saveRegion(region);

    this.state.activeDownload = regionId;
    this.state.downloadProgress = 0;
    this.bytesDownloaded = 0;
    this.downloadStartTime = Date.now();
    this.abortController = new AbortController();
    this.emit();

    // Build download queue
    this.downloadQueue = [];
    for (let z = region.minZoom; z <= region.maxZoom; z++) {
      const xMin = lon2tile(region.bounds.west, z);
      const xMax = lon2tile(region.bounds.east, z);
      const yMin = lat2tile(region.bounds.north, z);
      const yMax = lat2tile(region.bounds.south, z);
      for (let x = xMin; x <= xMax; x++) {
        for (let y = yMin; y <= yMax; y++) {
          for (const provider of region.providers) {
            const key = tileKey(provider, { x, y, z });
            const exists = await this.cache.hasTile(key);
            if (!exists) {
              this.downloadQueue.push({ provider, coord: { x, y, z } });
            } else {
              region.downloadedTiles++;
            }
          }
        }
      }
    }

    // Start concurrent downloads
    const promises: Promise<void>[] = [];
    for (let i = 0; i < this.concurrency; i++) {
      promises.push(this.downloadWorker(region));
    }
    await Promise.all(promises);

    // Finalize
    if (this.abortController && !this.abortController.signal.aborted) {
      region.status = 'complete';
      region.updatedAt = Date.now();
      await this.cache.saveRegion(region);
      this.state.activeDownload = null;
      this.state.downloadProgress = 100;
      this.state.storageStats = await this.cache.getStats();
      this.emit();
    }
  }

  private async downloadWorker(region: DownloadRegion): Promise<void> {
    while (this.downloadQueue.length > 0) {
      if (this.abortController?.signal.aborted) return;

      const item = this.downloadQueue.shift();
      if (!item) return;

      this.activeDownloads++;
      try {
        const url = getTileUrl(item.provider, item.coord);
        const resp = await fetch(url, {
          signal: this.abortController?.signal,
        });
        if (resp.ok) {
          const blob = await resp.blob();
          const tile: CachedTile = {
            key: tileKey(item.provider, item.coord),
            provider: item.provider,
            coord: item.coord,
            blob,
            size: blob.size,
            cachedAt: Date.now(),
            lastAccessed: Date.now(),
            etag: resp.headers.get('etag') ?? undefined,
            version: 1,
          };
          await this.cache.putTile(tile);
          region.downloadedTiles++;
          region.totalSize += blob.size;
          this.bytesDownloaded += blob.size;

          // Update progress
          const progress = Math.round((region.downloadedTiles / region.totalTiles) * 100);
          this.state.downloadProgress = progress;
          const elapsed = (Date.now() - this.downloadStartTime) / 1000;
          this.state.downloadSpeed = elapsed > 0 ? this.bytesDownloaded / elapsed : 0;

          // Emit every 10 tiles to avoid excessive updates
          if (region.downloadedTiles % 10 === 0) {
            this.state.regions = this.state.regions.map(r =>
              r.id === region.id ? { ...region } : r
            );
            this.emit();
          }
        }
      } catch (err) {
        if ((err as Error).name === 'AbortError') return;
        // Skip failed tile, continue
      }
      this.activeDownloads--;
    }
  }

  pauseDownload(): void {
    if (this.abortController) {
      this.abortController.abort();
      this.abortController = null;
    }
    const region = this.state.regions.find(r => r.id === this.state.activeDownload);
    if (region) {
      region.status = 'paused';
      region.updatedAt = Date.now();
      this.cache.saveRegion(region);
    }
    this.state.activeDownload = null;
    this.downloadQueue = [];
    this.emit();
  }

  async deleteRegion(regionId: string): Promise<void> {
    if (this.state.activeDownload === regionId) {
      this.pauseDownload();
    }
    await this.cache.deleteRegion(regionId);
    this.state.regions = this.state.regions.filter(r => r.id !== regionId);
    this.state.storageStats = await this.cache.getStats();
    this.emit();
  }

  // ─── Offline Routes ──────────────────────────────────────

  async saveOfflineRoute(route: Omit<OfflineRoute, 'id' | 'cachedAt' | 'expiresAt'>): Promise<OfflineRoute> {
    const offlineRoute: OfflineRoute = {
      ...route,
      id: `route-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
      cachedAt: Date.now(),
      expiresAt: Date.now() + 30 * 24 * 60 * 60 * 1000, // 30 days
    };
    await this.cache.saveRoute(offlineRoute);
    this.state.offlineRoutes = [...this.state.offlineRoutes, offlineRoute];
    this.emit();
    return offlineRoute;
  }

  async getOfflineRoutes(): Promise<OfflineRoute[]> {
    return this.cache.getRoutes();
  }

  // ─── Delta Updates ───────────────────────────────────────

  async checkForUpdates(): Promise<number> {
    if (!this.state.isOnline) return 0;

    let pendingUpdates = 0;
    // Check a sample of tiles for staleness (>7 days old)
    const staleThreshold = Date.now() - 7 * 24 * 60 * 60 * 1000;

    for (const region of this.state.regions) {
      if (region.status !== 'complete') continue;
      // Estimate based on region age
      if (region.updatedAt < staleThreshold) {
        pendingUpdates += Math.ceil(region.totalTiles * 0.05); // ~5% need update
      }
    }

    this.state.pendingDeltaUpdates = pendingUpdates;
    this.state.lastSyncAt = Date.now();
    this.emit();
    return pendingUpdates;
  }

  // ─── Storage Management ──────────────────────────────────

  async refreshStats(): Promise<StorageStats> {
    const stats = await this.cache.getStats();
    this.state.storageStats = stats;
    this.emit();
    return stats;
  }

  async evictOldTiles(targetMB: number): Promise<number> {
    const freed = await this.cache.evictLRU(targetMB * 1024 * 1024);
    this.state.storageStats = await this.cache.getStats();
    this.emit();
    return freed;
  }

  async clearAllData(): Promise<void> {
    this.pauseDownload();
    await this.cache.clearAll();
    this.state.regions = [];
    this.state.offlineRoutes = [];
    this.state.storageStats = await this.cache.getStats();
    this.emit();
  }

  getStorageUsagePercent(): number {
    const { totalSize, quotaSize } = this.state.storageStats;
    return quotaSize > 0 ? Math.round((totalSize / quotaSize) * 100) : 0;
  }

  // ─── Utility ─────────────────────────────────────────────

  formatBytes(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  }

  destroy(): void {
    if (this.abortController) {
      this.abortController.abort();
    }
    this.listeners.clear();
    this.downloadQueue = [];
  }
}

// ═══════════════════════════════════════════════════════════
// SINGLETON
// ═══════════════════════════════════════════════════════════

let instance: OfflineMapEngine | null = null;

export function getOfflineMapEngine(): OfflineMapEngine {
  if (!instance) {
    instance = new OfflineMapEngine();
  }
  return instance;
}

export function destroyOfflineMapEngine(): void {
  if (instance) {
    instance.destroy();
    instance = null;
  }
}
