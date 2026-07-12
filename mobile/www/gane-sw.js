/* G.A.N.E Service Worker — Offline-first, tile cache, background sync */
const CACHE_VERSION = 'gane-v6';
const STATIC_CACHE = `${CACHE_VERSION}-static`;
const TILE_CACHE = `${CACHE_VERSION}-tiles`;
const API_CACHE = `${CACHE_VERSION}-api`;
const MAX_TILES = 5000;
const MAX_API_AGE = 3600000; // 1 hour

const STATIC_ASSETS = [
  './',
  './gane-v6-integrated.html',
  './gane-core-bundle.js',
  './manifest.json',
  'https://unpkg.com/leaflet@1.9.4/dist/leaflet.css',
  'https://unpkg.com/leaflet@1.9.4/dist/leaflet.js'
];

self.addEventListener('install', e => {
  self.skipWaiting();
  e.waitUntil(caches.open(STATIC_CACHE).then(c =>
    Promise.allSettled(STATIC_ASSETS.map(u => c.add(u).catch(()=>null)))
  ));
});

self.addEventListener('activate', e => {
  e.waitUntil(Promise.all([
    caches.keys().then(keys => Promise.all(
      keys.filter(k => ![STATIC_CACHE, TILE_CACHE, API_CACHE].includes(k)).map(k => caches.delete(k))
    )),
    self.clients.claim()
  ]));
});

self.addEventListener('fetch', e => {
  const url = new URL(e.request.url);
  if (e.request.method !== 'GET') return;

  // TILE STRATEGY: cache-first with stale-while-revalidate
  if (url.pathname.match(/\/\d+\/\d+\/\d+\.(png|jpg|webp)/) ||
      url.hostname.includes('tile') || url.hostname.includes('tilecache')) {
    e.respondWith((async () => {
      const cache = await caches.open(TILE_CACHE);
      const cached = await cache.match(e.request);
      if (cached) {
        // Background refresh
        fetch(e.request).then(r => r.ok && cache.put(e.request, r.clone())).catch(()=>{});
        return cached;
      }
      try {
        const fresh = await fetch(e.request);
        if (fresh.ok) {
          cache.put(e.request, fresh.clone());
          // LRU prune
          const keys = await cache.keys();
          if (keys.length > MAX_TILES) cache.delete(keys[0]);
        }
        return fresh;
      } catch {
        // 1x1 transparent PNG fallback
        return new Response(
          Uint8Array.from(atob('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII='), c=>c.charCodeAt(0)),
          {headers:{'Content-Type':'image/png'}}
        );
      }
    })());
    return;
  }

  // API STRATEGY: network-first with cache fallback
  if (url.hostname.includes('nominatim') || url.hostname.includes('overpass') ||
      url.hostname.includes('valhalla') || url.hostname.includes('osrm') ||
      url.hostname.includes('open-meteo') || url.hostname.includes('rainviewer') ||
      url.hostname.includes('photon') || url.hostname.includes('earthquake.usgs')) {
    e.respondWith((async () => {
      try {
        const fresh = await fetch(e.request);
        if (fresh.ok) {
          const cache = await caches.open(API_CACHE);
          const clone = fresh.clone();
          // Add timestamp header for aging
          const headers = new Headers(clone.headers);
          headers.set('sw-cached-at', Date.now().toString());
          const aged = new Response(await clone.blob(), {status:clone.status, headers});
          cache.put(e.request, aged);
        }
        return fresh;
      } catch {
        const cached = await caches.match(e.request);
        if (cached) {
          const cachedAt = parseInt(cached.headers.get('sw-cached-at') || '0');
          if (Date.now() - cachedAt < MAX_API_AGE * 24) return cached; // stale but available
        }
        return new Response(JSON.stringify({offline:true, source:'sw-fallback'}), {
          status: 503,
          headers: {'Content-Type':'application/json'}
        });
      }
    })());
    return;
  }

  // STATIC STRATEGY: cache-first
  e.respondWith((async () => {
    const cached = await caches.match(e.request);
    if (cached) return cached;
    try {
      const fresh = await fetch(e.request);
      if (fresh.ok) {
        const cache = await caches.open(STATIC_CACHE);
        cache.put(e.request, fresh.clone());
      }
      return fresh;
    } catch {
      return cached || new Response('Offline', {status:503});
    }
  })());
});

// Background sync for telemetry
self.addEventListener('sync', e => {
  if (e.tag === 'gane-telemetry') {
    e.waitUntil(flushPendingTelemetry());
  }
});

async function flushPendingTelemetry() {
  // Sends buffered events to backend when network returns
  const db = await openDB();
  const tx = db.transaction('pending', 'readwrite');
  const store = tx.objectStore('pending');
  const all = await new Promise(res => { const r = store.getAll(); r.onsuccess = () => res(r.result||[]); });
  const backend = self.registration.scope + '/api/telemetry';
  try {
    await fetch(backend, {method:'POST', body: JSON.stringify({events:all}), headers:{'Content-Type':'application/json'}});
    await new Promise(res => { const r = store.clear(); r.onsuccess = () => res(); });
  } catch {}
}

function openDB() {
  return new Promise(res => {
    const req = indexedDB.open('gane-sw-queue', 1);
    req.onupgradeneeded = e => e.target.result.createObjectStore('pending', {autoIncrement:true});
    req.onsuccess = e => res(e.target.result);
  });
}

// Listen for messages from app (for mission save, cache region commands)
self.addEventListener('message', async e => {
  if (e.data?.type === 'CACHE_TILES') {
    const {urls} = e.data;
    const cache = await caches.open(TILE_CACHE);
    let ok = 0, fail = 0;
    for (const u of urls) {
      try { const r = await fetch(u); if (r.ok) { await cache.put(u, r); ok++; } else fail++; }
      catch { fail++; }
    }
    e.source.postMessage({type:'CACHE_COMPLETE', ok, fail});
  }
  if (e.data?.type === 'CLEAR_CACHES') {
    for (const k of await caches.keys()) await caches.delete(k);
    e.source.postMessage({type:'CACHES_CLEARED'});
  }
});
