/**
 * G.A.N.E — Service Worker
 * 
 * Caching Strategies:
 * - Map tiles: Cache-first with LRU eviction (max 2000 entries, 7-day TTL)
 * - Static assets (JS/CSS/fonts): Stale-while-revalidate
 * - API responses: Network-first with cache fallback
 * - App shell (HTML): Cache-first for instant offline loads
 */

const APP_CACHE = 'gane-app-v1';
const TILE_CACHE = 'gane-tiles-v1';
const STATIC_CACHE = 'gane-static-v1';
const API_CACHE = 'gane-api-v1';

const MAX_TILE_ENTRIES = 2000;
const TILE_TTL_MS = 7 * 24 * 60 * 60 * 1000; // 7 days

// Tile URL patterns
const TILE_PATTERNS = [
  /tile\.openstreetmap\.org/,
  /maps\.googleapis\.com\/maps\/vt/,
  /maps\.googleapis\.com\/maps\/api\/js/,
  /api\.mapbox\.com\/v4/,
  /basemaps\.cartocdn\.com/,
  /server\.arcgisonline\.com\/ArcGIS\/rest\/services/,
  /khms\d*\.googleapis\.com/,
  /mt\d*\.googleapis\.com/,
];

// Static asset extensions
const STATIC_EXTENSIONS = ['.js', '.css', '.woff', '.woff2', '.ttf', '.otf', '.png', '.jpg', '.jpeg', '.svg', '.webp', '.ico'];

// ─── Install: Precache app shell ───
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(APP_CACHE).then((cache) => {
      return cache.addAll([
        './',
      ]);
    }).then(() => self.skipWaiting())
  );
});

// ─── Activate: Clean old caches ───
self.addEventListener('activate', (event) => {
  const validCaches = [APP_CACHE, TILE_CACHE, STATIC_CACHE, API_CACHE];
  event.waitUntil(
    caches.keys().then((keys) => {
      return Promise.all(
        keys
          .filter((key) => !validCaches.includes(key))
          .map((key) => caches.delete(key))
      );
    }).then(() => self.clients.claim())
  );
});

// ─── Fetch: Route-based caching ───
self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url);

  // Only handle GET requests
  if (event.request.method !== 'GET') return;

  // Skip chrome-extension and non-http(s) requests
  if (!url.protocol.startsWith('http')) return;

  // Map tiles → Cache-first
  if (isTileRequest(url)) {
    event.respondWith(tilesCacheFirst(event.request));
    return;
  }

  // Static assets → Stale-while-revalidate
  if (isStaticAsset(url)) {
    event.respondWith(staleWhileRevalidate(event.request));
    return;
  }

  // API requests → Network-first
  if (isApiRequest(url)) {
    event.respondWith(networkFirst(event.request));
    return;
  }

  // Navigation requests → Cache-first (app shell)
  if (event.request.mode === 'navigate') {
    event.respondWith(appShellCacheFirst(event.request));
    return;
  }
});

// ─── URL Classification ───
function isTileRequest(url) {
  return TILE_PATTERNS.some((pattern) => pattern.test(url.href));
}

function isStaticAsset(url) {
  const pathname = url.pathname.toLowerCase();
  return STATIC_EXTENSIONS.some((ext) => pathname.endsWith(ext));
}

function isApiRequest(url) {
  return url.pathname.startsWith('/api/');
}

// ─── Cache-First for Map Tiles (with LRU eviction) ───
async function tilesCacheFirst(request) {
  const cache = await caches.open(TILE_CACHE);
  const cached = await cache.match(request);

  if (cached) {
    // Check TTL
    const cachedDate = cached.headers.get('sw-cached-at');
    if (cachedDate) {
      const age = Date.now() - parseInt(cachedDate, 10);
      if (age < TILE_TTL_MS) {
        return cached;
      }
    } else {
      return cached;
    }
  }

  try {
    const response = await fetch(request);
    if (response.ok) {
      // Clone and add timestamp header
      const cloned = response.clone();
      const headers = new Headers(cloned.headers);
      headers.set('sw-cached-at', String(Date.now()));
      const timedResponse = new Response(await cloned.blob(), {
        status: cloned.status,
        statusText: cloned.statusText,
        headers,
      });
      
      await cache.put(request, timedResponse);
      await evictOldTiles(cache);
    }
    return response;
  } catch (err) {
    // Offline: return stale cached version if available
    if (cached) return cached;
    return new Response('Tile unavailable offline', { status: 503 });
  }
}

// ─── LRU Eviction for Tile Cache ───
async function evictOldTiles(cache) {
  const keys = await cache.keys();
  if (keys.length <= MAX_TILE_ENTRIES) return;

  // Evict oldest entries (FIFO approximation)
  const toDelete = keys.length - MAX_TILE_ENTRIES;
  const deletePromises = keys.slice(0, toDelete).map((key) => cache.delete(key));
  await Promise.all(deletePromises);
}

// ─── Stale-While-Revalidate for Static Assets ───
async function staleWhileRevalidate(request) {
  const cache = await caches.open(STATIC_CACHE);
  const cached = await cache.match(request);

  const fetchPromise = fetch(request).then((response) => {
    if (response.ok) {
      cache.put(request, response.clone());
    }
    return response;
  }).catch(() => {
    // Network failed, cached version is all we have
    return cached || new Response('Asset unavailable offline', { status: 503 });
  });

  // Return cached immediately, update in background
  return cached || fetchPromise;
}

// ─── Network-First for API Requests ───
async function networkFirst(request) {
  const cache = await caches.open(API_CACHE);

  try {
    const response = await fetch(request);
    if (response.ok) {
      cache.put(request, response.clone());
    }
    return response;
  } catch (err) {
    const cached = await cache.match(request);
    if (cached) return cached;
    return new Response(JSON.stringify({ error: 'Offline', offline: true }), {
      status: 503,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}

// ─── App Shell Cache-First ───
async function appShellCacheFirst(request) {
  const cache = await caches.open(APP_CACHE);
  const cached = await cache.match('./');

  if (cached) {
    // Update in background
    fetch(request).then((response) => {
      if (response.ok) cache.put('./', response.clone());
    }).catch(() => {});
    return cached;
  }

  try {
    const response = await fetch(request);
    if (response.ok) cache.put('./', response.clone());
    return response;
  } catch (err) {
    return new Response('<html><body><h1>G.A.N.E — Offline</h1><p>Please reconnect to continue.</p></body></html>', {
      status: 503,
      headers: { 'Content-Type': 'text/html' },
    });
  }
}

// ─── Message Handler (for cache management from app) ───
self.addEventListener('message', (event) => {
  if (event.data === 'SKIP_WAITING') {
    self.skipWaiting();
  }
  if (event.data === 'CLEAR_TILE_CACHE') {
    caches.delete(TILE_CACHE);
  }
  if (event.data === 'CLEAR_ALL_CACHES') {
    caches.keys().then((keys) => keys.forEach((key) => caches.delete(key)));
  }
});
