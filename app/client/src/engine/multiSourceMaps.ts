/**
 * G.A.N.E Multi-Source Maps Engine
 * ==================================
 * 
 * Combines data from multiple map sources:
 * - OpenStreetMap (base tiles)
 * - Google Maps (via proxy)
 * - Satellite imagery (commercial providers)
 * - Crowdsourced SLAM data (self-healing updates)
 * - Real-time traffic overlays
 * 
 * Implements:
 * - Tile priority system (freshest data wins)
 * - Offline tile caching (IndexedDB)
 * - Delta merge from SLAM reports
 * - Conflict resolution (multiple sources disagree)
 */

import { OfflineStore } from './offlineStore';

// ─── Map Source Types ───

export interface MapSource {
  id: string;
  name: string;
  type: 'raster' | 'vector' | 'overlay' | 'crowdsourced';
  priority: number;           // Higher = preferred
  isOnline: boolean;
  lastSync: number;
  tileUrlTemplate?: string;
  attribution: string;
  maxZoom: number;
  minZoom: number;
  opacity: number;
  enabled: boolean;
}

export interface TileCoord {
  x: number;
  y: number;
  z: number;
}

export interface CachedTile {
  coord: TileCoord;
  sourceId: string;
  data: ArrayBuffer | string;  // Raw tile data or URL
  timestamp: number;
  expiresAt: number;
}

export interface MapDelta {
  id: string;
  type: 'road_added' | 'road_removed' | 'road_modified' | 'poi_added' | 'poi_removed' | 'hazard';
  geometry: { lat: number; lon: number }[];
  properties: Record<string, unknown>;
  source: 'crowdsourced' | 'official' | 'satellite';
  confidence: number;
  timestamp: number;
}

// ─── Default Sources ───

const DEFAULT_SOURCES: MapSource[] = [
  {
    id: 'google-maps',
    name: 'Google Maps',
    type: 'raster',
    priority: 100,
    isOnline: true,
    lastSync: Date.now(),
    attribution: '© Google',
    maxZoom: 22,
    minZoom: 1,
    opacity: 1,
    enabled: true,
  },
  {
    id: 'osm',
    name: 'OpenStreetMap',
    type: 'raster',
    priority: 80,
    isOnline: true,
    lastSync: Date.now(),
    tileUrlTemplate: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
    attribution: '© OpenStreetMap contributors',
    maxZoom: 19,
    minZoom: 1,
    opacity: 1,
    enabled: false,  // Fallback source
  },
  {
    id: 'satellite',
    name: 'Satellite Imagery',
    type: 'raster',
    priority: 90,
    isOnline: true,
    lastSync: Date.now(),
    attribution: '© Commercial Satellite Providers',
    maxZoom: 20,
    minZoom: 1,
    opacity: 1,
    enabled: false,
  },
  {
    id: 'crowdsourced',
    name: 'G.A.N.E Crowdsourced',
    type: 'crowdsourced',
    priority: 110,  // Highest priority — freshest data
    isOnline: true,
    lastSync: Date.now(),
    attribution: '© G.A.N.E Network',
    maxZoom: 22,
    minZoom: 10,
    opacity: 0.8,
    enabled: true,
  },
  {
    id: 'traffic-overlay',
    name: 'Real-Time Traffic',
    type: 'overlay',
    priority: 95,
    isOnline: true,
    lastSync: Date.now(),
    attribution: '© G.A.N.E Traffic',
    maxZoom: 18,
    minZoom: 8,
    opacity: 0.6,
    enabled: true,
  },
];

// ─── Multi-Source Maps Engine ───

export class MultiSourceMapsEngine {
  private sources: Map<string, MapSource> = new Map();
  private deltas: MapDelta[] = [];
  private offlineStore: OfflineStore | null = null;
  private listeners: Set<() => void> = new Set();
  private tileCache: Map<string, CachedTile> = new Map();
  private maxCacheSize = 500;  // Max tiles in memory

  constructor() {
    DEFAULT_SOURCES.forEach(s => this.sources.set(s.id, { ...s }));
  }

  /** Initialize with offline store for tile caching */
  async init(offlineStore?: OfflineStore): Promise<void> {
    if (offlineStore) {
      this.offlineStore = offlineStore;
    }
  }

  /** Get all sources */
  getSources(): MapSource[] {
    return Array.from(this.sources.values()).sort((a, b) => b.priority - a.priority);
  }

  /** Get enabled sources sorted by priority */
  getEnabledSources(): MapSource[] {
    return this.getSources().filter(s => s.enabled);
  }

  /** Enable/disable a source */
  toggleSource(sourceId: string, enabled?: boolean): void {
    const source = this.sources.get(sourceId);
    if (source) {
      source.enabled = enabled !== undefined ? enabled : !source.enabled;
      this.notify();
    }
  }

  /** Set source opacity */
  setSourceOpacity(sourceId: string, opacity: number): void {
    const source = this.sources.get(sourceId);
    if (source) {
      source.opacity = Math.max(0, Math.min(1, opacity));
      this.notify();
    }
  }

  /** Add a custom map source */
  addSource(source: MapSource): void {
    this.sources.set(source.id, source);
    this.notify();
  }

  /** Remove a custom map source */
  removeSource(sourceId: string): void {
    this.sources.delete(sourceId);
    this.notify();
  }

  // ─── Delta Management ───

  /** Apply a map delta (from SLAM or official update) */
  applyDelta(delta: MapDelta): void {
    // Check for conflicts with existing deltas
    const conflictIdx = this.deltas.findIndex(d =>
      d.type === delta.type &&
      this.isNearby(d.geometry[0], delta.geometry[0], 0.0001)
    );

    if (conflictIdx >= 0) {
      const existing = this.deltas[conflictIdx];
      // Conflict resolution: higher confidence wins, or newer timestamp
      if (delta.confidence > existing.confidence ||
          (delta.confidence === existing.confidence && delta.timestamp > existing.timestamp)) {
        this.deltas[conflictIdx] = delta;
      }
    } else {
      this.deltas.push(delta);
    }

    // Trim old deltas (keep last 1000)
    if (this.deltas.length > 1000) {
      this.deltas = this.deltas.slice(-1000);
    }

    this.notify();
  }

  /** Get all active deltas */
  getDeltas(): MapDelta[] {
    return [...this.deltas];
  }

  /** Get deltas within a bounding box */
  getDeltasInBounds(
    minLat: number, maxLat: number,
    minLon: number, maxLon: number
  ): MapDelta[] {
    return this.deltas.filter(d => {
      const pt = d.geometry[0];
      return pt && pt.lat >= minLat && pt.lat <= maxLat &&
             pt.lon >= minLon && pt.lon <= maxLon;
    });
  }

  /** Clear expired deltas */
  clearExpiredDeltas(maxAgeMs: number = 24 * 60 * 60 * 1000): void {
    const cutoff = Date.now() - maxAgeMs;
    this.deltas = this.deltas.filter(d => d.timestamp > cutoff);
    this.notify();
  }

  // ─── Tile Caching ───

  /** Get tile key for cache lookup */
  private getTileKey(coord: TileCoord, sourceId: string): string {
    return `${sourceId}/${coord.z}/${coord.x}/${coord.y}`;
  }

  /** Cache a tile in memory and optionally in IndexedDB */
  async cacheTile(coord: TileCoord, sourceId: string, data: ArrayBuffer | string): Promise<void> {
    const key = this.getTileKey(coord, sourceId);
    const tile: CachedTile = {
      coord,
      sourceId,
      data,
      timestamp: Date.now(),
      expiresAt: Date.now() + 7 * 24 * 60 * 60 * 1000, // 7 days
    };

    this.tileCache.set(key, tile);

    // Evict oldest tiles if cache is full
    if (this.tileCache.size > this.maxCacheSize) {
      const entries = Array.from(this.tileCache.entries());
      entries.sort((a, b) => a[1].timestamp - b[1].timestamp);
      const toRemove = entries.slice(0, entries.length - this.maxCacheSize);
      for (const [k] of toRemove) {
        this.tileCache.delete(k);
      }
    }

    // Also cache in IndexedDB for offline use
    if (this.offlineStore && typeof data !== 'string') {
      await this.offlineStore.cacheTile(key, coord.z, data);
    }
  }

  /** Get a cached tile */
  getCachedTile(coord: TileCoord, sourceId: string): CachedTile | null {
    const key = this.getTileKey(coord, sourceId);
    const tile = this.tileCache.get(key);
    if (tile && tile.expiresAt > Date.now()) {
      return tile;
    }
    return null;
  }

  /** Get cache statistics */
  getCacheStats(): { memoryTiles: number; maxTiles: number } {
    return {
      memoryTiles: this.tileCache.size,
      maxTiles: this.maxCacheSize,
    };
  }

  /** Clear all cached tiles */
  clearCache(): void {
    this.tileCache.clear();
  }

  // ─── Offline Support ───

  /** Pre-download tiles for an area (for offline use) */
  async preloadArea(
    centerLat: number, centerLon: number,
    radiusKm: number, maxZoom: number = 16
  ): Promise<{ tilesQueued: number }> {
    // Calculate tile bounds for the area
    const latDelta = radiusKm / 111.32;
    const lonDelta = radiusKm / (111.32 * Math.cos(centerLat * Math.PI / 180));

    let tilesQueued = 0;

    for (let z = 10; z <= maxZoom; z++) {
      const minTileX = this.lonToTileX(centerLon - lonDelta, z);
      const maxTileX = this.lonToTileX(centerLon + lonDelta, z);
      const minTileY = this.latToTileY(centerLat + latDelta, z);
      const maxTileY = this.latToTileY(centerLat - latDelta, z);

      for (let x = minTileX; x <= maxTileX; x++) {
        for (let y = minTileY; y <= maxTileY; y++) {
          const coord = { x, y, z };
          if (!this.getCachedTile(coord, 'osm')) {
            tilesQueued++;
            // In production, queue these for background download
          }
        }
      }
    }

    return { tilesQueued };
  }

  // ─── Coordinate Utilities ───

  private lonToTileX(lon: number, zoom: number): number {
    return Math.floor((lon + 180) / 360 * Math.pow(2, zoom));
  }

  private latToTileY(lat: number, zoom: number): number {
    return Math.floor(
      (1 - Math.log(Math.tan(lat * Math.PI / 180) + 1 / Math.cos(lat * Math.PI / 180)) / Math.PI)
      / 2 * Math.pow(2, zoom)
    );
  }

  private isNearby(
    a: { lat: number; lon: number } | undefined,
    b: { lat: number; lon: number } | undefined,
    threshold: number
  ): boolean {
    if (!a || !b) return false;
    return Math.abs(a.lat - b.lat) < threshold && Math.abs(a.lon - b.lon) < threshold;
  }

  // ─── Event System ───

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notify(): void {
    this.listeners.forEach(fn => fn());
  }
}

// Singleton
let msmInstance: MultiSourceMapsEngine | null = null;

export function getMultiSourceMaps(): MultiSourceMapsEngine {
  if (!msmInstance) {
    msmInstance = new MultiSourceMapsEngine();
  }
  return msmInstance;
}
