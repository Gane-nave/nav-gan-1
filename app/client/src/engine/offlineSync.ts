/**
 * G.A.N.E — Offline Sync Engine
 * ===============================
 * IndexedDB-backed offline data persistence with sync queue.
 *
 * FEATURES:
 *   - Store map tiles, routes, POIs offline
 *   - Queue mutations for later sync
 *   - Conflict resolution (last-write-wins)
 *   - Storage quota management
 *   - Sync status tracking
 *
 * STORAGE LAYERS:
 *   1. In-memory cache (hot data)
 *   2. IndexedDB (persistent offline)
 *   3. Server (authoritative)
 */

// ─── Types ───────────────────────────────────────────────

export interface OfflineConfig {
  dbName: string;
  maxStorageMB: number;           // max offline storage (default: 100MB)
  syncIntervalMs: number;        // auto-sync interval (default: 30000)
  maxQueueSize: number;           // max pending mutations (default: 500)
  enabled: boolean;
}

export interface SyncQueueItem {
  id: string;
  type: 'create' | 'update' | 'delete';
  store: string;
  key: string;
  data: unknown;
  timestamp: number;
  retryCount: number;
  maxRetries: number;
}

export interface SyncStatus {
  isOnline: boolean;
  isSyncing: boolean;
  pendingCount: number;
  lastSyncAt: number;
  lastSyncResult: 'success' | 'partial' | 'failed' | 'none';
  totalSynced: number;
  totalFailed: number;
}

export interface OfflineStore {
  name: string;
  keyPath: string;
  indexes: { name: string; keyPath: string; unique: boolean }[];
}

export interface StorageStats {
  totalBytes: number;
  usedBytes: number;
  availableBytes: number;
  storeBreakdown: Record<string, number>;
}

// ─── Constants ──────────────────────────────────────────

const DEFAULT_CONFIG: OfflineConfig = {
  dbName: 'gane_offline',
  maxStorageMB: 100,
  syncIntervalMs: 30000,
  maxQueueSize: 500,
  enabled: true,
};

const STORES: OfflineStore[] = [
  {
    name: 'routes',
    keyPath: 'id',
    indexes: [
      { name: 'userId', keyPath: 'userId', unique: false },
      { name: 'createdAt', keyPath: 'createdAt', unique: false },
    ],
  },
  {
    name: 'pois',
    keyPath: 'id',
    indexes: [
      { name: 'type', keyPath: 'type', unique: false },
      { name: 'lat', keyPath: 'lat', unique: false },
    ],
  },
  {
    name: 'mapTiles',
    keyPath: 'key',
    indexes: [
      { name: 'zoom', keyPath: 'zoom', unique: false },
      { name: 'cachedAt', keyPath: 'cachedAt', unique: false },
    ],
  },
  {
    name: 'telemetry',
    keyPath: 'id',
    indexes: [
      { name: 'timestamp', keyPath: 'timestamp', unique: false },
      { name: 'synced', keyPath: 'synced', unique: false },
    ],
  },
  {
    name: 'incidents',
    keyPath: 'id',
    indexes: [
      { name: 'type', keyPath: 'type', unique: false },
      { name: 'createdAt', keyPath: 'createdAt', unique: false },
    ],
  },
  {
    name: 'syncQueue',
    keyPath: 'id',
    indexes: [
      { name: 'timestamp', keyPath: 'timestamp', unique: false },
      { name: 'store', keyPath: 'store', unique: false },
    ],
  },
  {
    name: 'settings',
    keyPath: 'key',
    indexes: [],
  },
];

// ─── Offline Sync Engine ────────────────────────────────

export class OfflineSyncEngine {
  private config: OfflineConfig;
  private db: IDBDatabase | null = null;
  private syncInterval: ReturnType<typeof setInterval> | null = null;
  private memoryCache: Map<string, Map<string, unknown>> = new Map();

  private status: SyncStatus = {
    isOnline: typeof navigator !== 'undefined' ? navigator.onLine : true,
    isSyncing: false,
    pendingCount: 0,
    lastSyncAt: 0,
    lastSyncResult: 'none',
    totalSynced: 0,
    totalFailed: 0,
  };

  private onStatusChange: ((status: SyncStatus) => void) | null = null;
  private syncHandler: ((items: SyncQueueItem[]) => Promise<string[]>) | null = null;

  // Bound event handlers for cleanup
  private boundOnline = () => { this.status.isOnline = true; this.notifyStatusChange(); this.sync(); };
  private boundOffline = () => { this.status.isOnline = false; this.notifyStatusChange(); };

  constructor(config: Partial<OfflineConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  // ─── Lifecycle ─────────────────────────────────────────

  async init(): Promise<boolean> {
    if (typeof indexedDB === 'undefined') {
      console.warn('[OfflineSync] IndexedDB not available');
      return false;
    }

    try {
      this.db = await this.openDatabase();

      // Listen for online/offline events
      if (typeof window !== 'undefined') {
        window.addEventListener('online', this.boundOnline);
        window.addEventListener('offline', this.boundOffline);
      }

      // Start auto-sync
      this.syncInterval = setInterval(() => {
        if (this.status.isOnline && !this.status.isSyncing) {
          this.sync();
        }
      }, this.config.syncIntervalMs);

      return true;
    } catch (e) {
      console.error('[OfflineSync] Init failed:', e);
      return false;
    }
  }

  destroy() {
    if (this.syncInterval) {
      clearInterval(this.syncInterval);
      this.syncInterval = null;
    }
    if (this.db) {
      this.db.close();
      this.db = null;
    }
    // Remove online/offline listeners
    if (typeof window !== 'undefined') {
      window.removeEventListener('online', this.boundOnline);
      window.removeEventListener('offline', this.boundOffline);
    }
    this.memoryCache.clear();
  }

  // ─── Database ─────────────────────────────────────────

  private openDatabase(): Promise<IDBDatabase> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(this.config.dbName, 2);

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;

        for (const store of STORES) {
          if (!db.objectStoreNames.contains(store.name)) {
            const objectStore = db.createObjectStore(store.name, { keyPath: store.keyPath });
            for (const index of store.indexes) {
              objectStore.createIndex(index.name, index.keyPath, { unique: index.unique });
            }
          }
        }
      };

      request.onsuccess = (event) => {
        resolve((event.target as IDBOpenDBRequest).result);
      };

      request.onerror = () => {
        reject(new Error('Failed to open IndexedDB'));
      };
    });
  }

  // ─── CRUD Operations ─────────────────────────────────

  async put(storeName: string, data: Record<string, unknown>): Promise<void> {
    // Memory cache
    if (!this.memoryCache.has(storeName)) {
      this.memoryCache.set(storeName, new Map());
    }
    const keyPath = STORES.find(s => s.name === storeName)?.keyPath || 'id';
    const key = String(data[keyPath]);
    this.memoryCache.get(storeName)!.set(key, data);

    // IndexedDB
    if (this.db) {
      await this.idbPut(storeName, data);
    }
  }

  async get(storeName: string, key: string): Promise<unknown | null> {
    // Check memory cache first
    const cached = this.memoryCache.get(storeName)?.get(key);
    if (cached !== undefined) return cached;

    // Fall back to IndexedDB
    if (this.db) {
      return this.idbGet(storeName, key);
    }

    return null;
  }

  async getAll(storeName: string): Promise<unknown[]> {
    if (this.db) {
      return this.idbGetAll(storeName);
    }

    // Fall back to memory cache
    const cache = this.memoryCache.get(storeName);
    return cache ? Array.from(cache.values()) : [];
  }

  async delete(storeName: string, key: string): Promise<void> {
    this.memoryCache.get(storeName)?.delete(key);

    if (this.db) {
      await this.idbDelete(storeName, key);
    }
  }

  // ─── Sync Queue ───────────────────────────────────────

  async enqueue(item: Omit<SyncQueueItem, 'id' | 'timestamp' | 'retryCount' | 'maxRetries'>): Promise<void> {
    const queueItem: SyncQueueItem = {
      ...item,
      id: `sync_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`,
      timestamp: Date.now(),
      retryCount: 0,
      maxRetries: 3,
    };

    await this.put('syncQueue', queueItem as unknown as Record<string, unknown>);
    this.status.pendingCount++;
    this.notifyStatusChange();
  }

  async sync(): Promise<void> {
    if (!this.syncHandler || this.status.isSyncing || !this.status.isOnline) return;

    this.status.isSyncing = true;
    this.notifyStatusChange();

    try {
      const queue = (await this.getAll('syncQueue')) as SyncQueueItem[];
      if (queue.length === 0) {
        this.status.isSyncing = false;
        this.status.lastSyncAt = Date.now();
        this.status.lastSyncResult = 'success';
        this.notifyStatusChange();
        return;
      }

      // Sort by timestamp
      queue.sort((a, b) => a.timestamp - b.timestamp);

      // Send to sync handler
      const failedIds = await this.syncHandler(queue);

      // Remove successful items
      let synced = 0;
      let failed = 0;
      for (const item of queue) {
        if (!failedIds.includes(item.id)) {
          await this.delete('syncQueue', item.id);
          synced++;
        } else {
          // Increment retry count
          item.retryCount++;
          if (item.retryCount >= item.maxRetries) {
            await this.delete('syncQueue', item.id);
            failed++;
          } else {
            await this.put('syncQueue', item as unknown as Record<string, unknown>);
            failed++;
          }
        }
      }

      this.status.totalSynced += synced;
      this.status.totalFailed += failed;
      this.status.pendingCount = Math.max(0, this.status.pendingCount - synced);
      this.status.lastSyncAt = Date.now();
      this.status.lastSyncResult = failed === 0 ? 'success' : synced > 0 ? 'partial' : 'failed';
    } catch (e) {
      console.error('[OfflineSync] Sync failed:', e);
      this.status.lastSyncResult = 'failed';
    } finally {
      this.status.isSyncing = false;
      this.notifyStatusChange();
    }
  }

  // ─── Callbacks ────────────────────────────────────────

  setOnStatusChange(callback: (status: SyncStatus) => void) {
    this.onStatusChange = callback;
  }

  setSyncHandler(handler: (items: SyncQueueItem[]) => Promise<string[]>) {
    this.syncHandler = handler;
  }

  // ─── Storage Stats ────────────────────────────────────

  async getStorageStats(): Promise<StorageStats> {
    const stats: StorageStats = {
      totalBytes: this.config.maxStorageMB * 1024 * 1024,
      usedBytes: 0,
      availableBytes: 0,
      storeBreakdown: {},
    };

    if (typeof navigator !== 'undefined' && 'storage' in navigator && 'estimate' in navigator.storage) {
      try {
        const estimate = await navigator.storage.estimate();
        stats.usedBytes = estimate.usage || 0;
        stats.totalBytes = estimate.quota || stats.totalBytes;
        stats.availableBytes = stats.totalBytes - stats.usedBytes;
      } catch {
        // Fallback
      }
    }

    return stats;
  }

  // ─── Public API ───────────────────────────────────────

  getStatus(): SyncStatus {
    return { ...this.status };
  }

  isOnline(): boolean {
    return this.status.isOnline;
  }

  // ─── IDB Helpers ──────────────────────────────────────

  private idbPut(storeName: string, data: Record<string, unknown>): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!this.db) return reject(new Error('DB not open'));
      const tx = this.db.transaction(storeName, 'readwrite');
      tx.objectStore(storeName).put(data);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }

  private idbGet(storeName: string, key: string): Promise<unknown | null> {
    return new Promise((resolve, reject) => {
      if (!this.db) return resolve(null);
      const tx = this.db.transaction(storeName, 'readonly');
      const request = tx.objectStore(storeName).get(key);
      request.onsuccess = () => resolve(request.result || null);
      request.onerror = () => reject(request.error);
    });
  }

  private idbGetAll(storeName: string): Promise<unknown[]> {
    return new Promise((resolve, reject) => {
      if (!this.db) return resolve([]);
      const tx = this.db.transaction(storeName, 'readonly');
      const request = tx.objectStore(storeName).getAll();
      request.onsuccess = () => resolve(request.result || []);
      request.onerror = () => reject(request.error);
    });
  }

  private idbDelete(storeName: string, key: string): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!this.db) return reject(new Error('DB not open'));
      const tx = this.db.transaction(storeName, 'readwrite');
      tx.objectStore(storeName).delete(key);
      tx.oncomplete = () => resolve();
      tx.onerror = () => reject(tx.error);
    });
  }

  private notifyStatusChange() {
    if (this.onStatusChange) {
      this.onStatusChange(this.getStatus());
    }
  }
}
