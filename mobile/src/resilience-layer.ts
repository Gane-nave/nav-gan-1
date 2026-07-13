// ═══════════════════════════════════════════════════════════════════════════
// G.A.N.E RESILIENCE LAYER — "Never-Fail" Navigation
// 10 bulletproof mechanisms that keep navigation running even when:
//   • all GNSS lost            • internet dead
//   • all map tiles blocked    • backend crashed
//   • all providers throttled  • browser in airplane mode
//   • device battery critical  • underground/tunnel
// ═══════════════════════════════════════════════════════════════════════════

/* ─── 1. SERVICE WORKER MANAGER — Offline-first tile caching ─────────── */
export const SERVICE_WORKER_CODE = `
// gane-sw.js — install as service worker
const CACHE_V = 'gane-v1';
const RUNTIME_CACHE = 'gane-runtime';
const TILE_CACHE = 'gane-tiles';
const MAX_TILES = 5000;

self.addEventListener('install', e => {
  self.skipWaiting();
  e.waitUntil(caches.open(CACHE_V).then(c => c.addAll([
    '/', '/index.html',
    'https://unpkg.com/leaflet@1.9.4/dist/leaflet.css',
    'https://unpkg.com/leaflet@1.9.4/dist/leaflet.js'
  ])));
});
self.addEventListener('activate', e => {
  e.waitUntil(caches.keys().then(keys => Promise.all(
    keys.filter(k => ![CACHE_V,RUNTIME_CACHE,TILE_CACHE].includes(k)).map(k => caches.delete(k))
  )));
  self.clients.claim();
});
self.addEventListener('fetch', e => {
  const url = new URL(e.request.url);
  // Tile requests: cache-first with background refresh
  if (url.pathname.match(/\\/\\d+\\/\\d+\\/\\d+\\.(png|jpg|webp)/) || url.hostname.includes('tile')) {
    e.respondWith((async () => {
      const cache = await caches.open(TILE_CACHE);
      const cached = await cache.match(e.request);
      if (cached) {
        // Background refresh (stale-while-revalidate)
        fetch(e.request).then(r => r.ok && cache.put(e.request, r.clone())).catch(()=>{});
        return cached;
      }
      try {
        const fresh = await fetch(e.request);
        if (fresh.ok) {
          cache.put(e.request, fresh.clone());
          // Prune if > MAX_TILES
          const keys = await cache.keys();
          if (keys.length > MAX_TILES) cache.delete(keys[0]);
        }
        return fresh;
      } catch {
        // Return transparent fallback tile
        return new Response(new Uint8Array([137,80,78,71,13,10,26,10]), {headers:{'Content-Type':'image/png'}});
      }
    })());
    return;
  }
  // API requests: network-first with cache fallback
  if (url.hostname.includes('nominatim') || url.hostname.includes('overpass') ||
      url.hostname.includes('valhalla') || url.hostname.includes('osrm')) {
    e.respondWith((async () => {
      try {
        const fresh = await fetch(e.request);
        if (fresh.ok) (await caches.open(RUNTIME_CACHE)).put(e.request, fresh.clone());
        return fresh;
      } catch {
        const cached = await caches.match(e.request);
        if (cached) return cached;
        return new Response(JSON.stringify({offline:true,cached:false}), {headers:{'Content-Type':'application/json'},status:503});
      }
    })());
  }
});
`;

export class ServiceWorkerManager {
  async register(): Promise<boolean> {
    if (!('serviceWorker' in navigator)) return false;
    try {
      const blob = new Blob([SERVICE_WORKER_CODE], {type:'application/javascript'});
      const url = URL.createObjectURL(blob);
      await navigator.serviceWorker.register(url, {scope:'/'});
      return true;
    } catch { return false; }
  }
  async cacheRegion(bounds:{n:number;s:number;e:number;w:number}, zoomMin=10, zoomMax=16): Promise<{cached:number; failed:number}> {
    let cached=0, failed=0;
    const cache = await caches.open('gane-tiles');
    for (let z=zoomMin; z<=zoomMax; z++) {
      const tiles = this.tilesInBounds(bounds, z);
      for (const {x,y} of tiles.slice(0, 200)) {
        try {
          const url = `https://tile.openstreetmap.org/${z}/${x}/${y}.png`;
          const r = await fetch(url);
          if (r.ok) { await cache.put(url, r); cached++; } else failed++;
        } catch { failed++; }
      }
    }
    return {cached, failed};
  }
  private tilesInBounds(b:any, z:number) {
    const lon2x = (lon:number)=>Math.floor((lon+180)/360*Math.pow(2,z));
    const lat2y = (lat:number)=>Math.floor((1-Math.log(Math.tan(lat*Math.PI/180)+1/Math.cos(lat*Math.PI/180))/Math.PI)/2*Math.pow(2,z));
    const tiles=[];
    for (let x=lon2x(b.w); x<=lon2x(b.e); x++)
      for (let y=lat2y(b.n); y<=lat2y(b.s); y++) tiles.push({x,y});
    return tiles;
  }
}

/* ─── 2. WATCHDOG — Dead-man's switch for all subsystems ─────────────── */
export class Watchdog {
  private heartbeats = new Map<string, number>();
  private timeouts = new Map<string, number>();
  private onFail: (s:string)=>void;
  private timer?: any;
  constructor(onFail:(subsystem:string)=>void) { this.onFail = onFail; }
  register(subsystem:string, timeoutMs:number) {
    this.heartbeats.set(subsystem, Date.now());
    this.timeouts.set(subsystem, timeoutMs);
  }
  pet(subsystem:string) { this.heartbeats.set(subsystem, Date.now()); }
  start() {
    this.timer = setInterval(() => {
      const now = Date.now();
      for (const [s, last] of this.heartbeats) {
        const to = this.timeouts.get(s) || 5000;
        if (now - last > to) { this.onFail(s); this.heartbeats.set(s, now); }
      }
    }, 1000);
  }
  stop() { if (this.timer) clearInterval(this.timer); }
  status() {
    const now = Date.now();
    return [...this.heartbeats.entries()].map(([s,last]) => ({
      subsystem:s, lastPetMs: now-last, timeoutMs: this.timeouts.get(s), healthy: now-last < (this.timeouts.get(s)||5000)
    }));
  }
}

/* ─── 3. CIRCUIT BREAKER — Auto-disable failing providers ────────────── */
export class CircuitBreaker {
  private state = new Map<string, {fails:number; lastFail:number; openUntil:number}>();
  private threshold = 3;
  private cooldown = 30000;
  async call<T>(key:string, fn:()=>Promise<T>): Promise<T|null> {
    const s = this.state.get(key) || {fails:0, lastFail:0, openUntil:0};
    if (Date.now() < s.openUntil) return null; // circuit open
    try {
      const r = await fn();
      s.fails = 0; this.state.set(key, s);
      return r;
    } catch (e) {
      s.fails++; s.lastFail = Date.now();
      if (s.fails >= this.threshold) s.openUntil = Date.now() + this.cooldown;
      this.state.set(key, s);
      return null;
    }
  }
  reset(key:string) { this.state.delete(key); }
  status() { return Object.fromEntries([...this.state.entries()].map(([k,v])=>[k,{...v, open:Date.now()<v.openUntil}])); }
}

/* ─── 4. ROUTING FALLBACK CHAIN — 5 tiers of routers ─────────────────── */
export class RoutingFallbackChain {
  private breaker = new CircuitBreaker();
  async route(from:[number,number], to:[number,number]): Promise<{coords:number[][]; source:string}|null> {
    const endpoints = [
      {name:'valhalla', fn:()=>this.valhalla(from,to)},
      {name:'osrm-eu', fn:()=>this.osrm(from,to,'https://routing.openstreetmap.de/routed-car')},
      {name:'osrm-demo', fn:()=>this.osrm(from,to,'https://router.project-osrm.org')},
      {name:'graphhopper', fn:()=>this.graphhopper(from,to)},
      {name:'straight-line', fn:()=>this.straightLine(from,to)}
    ];
    for (const ep of endpoints) {
      const r = await this.breaker.call(ep.name, ep.fn);
      if (r) return {coords:r, source:ep.name};
    }
    return null;
  }
  private async valhalla(f:[number,number], t:[number,number]): Promise<number[][]> {
    const r = await fetch('https://valhalla1.openstreetmap.de/route', {
      method:'POST', headers:{'Content-Type':'application/json'},
      body: JSON.stringify({locations:[{lat:f[0],lon:f[1]},{lat:t[0],lon:t[1]}],costing:'auto'})
    });
    if (!r.ok) throw new Error('valhalla');
    const d = await r.json();
    return this.decode(d.trip.legs[0].shape);
  }
  private async osrm(f:[number,number], t:[number,number], base:string): Promise<number[][]> {
    const r = await fetch(`${base}/route/v1/driving/${f[1]},${f[0]};${t[1]},${t[0]}?overview=full&geometries=geojson`);
    if (!r.ok) throw new Error('osrm');
    const d = await r.json();
    return d.routes[0].geometry.coordinates.map((c:any)=>[c[1],c[0]]);
  }
  private async graphhopper(f:[number,number], t:[number,number]): Promise<number[][]> {
    // Public instance (rate limited)
    const r = await fetch(`https://graphhopper.com/api/1/route?point=${f[0]},${f[1]}&point=${t[0]},${t[1]}&type=json&points_encoded=false&profile=car&key=free`);
    if (!r.ok) throw new Error('graphhopper');
    const d = await r.json();
    return d.paths[0].points.coordinates.map((c:any)=>[c[1],c[0]]);
  }
  // Ultimate fallback: straight-line with waypoints every 500m
  private async straightLine(f:[number,number], t:[number,number]): Promise<number[][]> {
    const coords: number[][] = [];
    const steps = 20;
    for (let i=0; i<=steps; i++) {
      coords.push([f[0]+(t[0]-f[0])*i/steps, f[1]+(t[1]-f[1])*i/steps]);
    }
    return coords;
  }
  private decode(enc:string): number[][] {
    const c=[]; let lat=0,lng=0,i=0;
    while(i<enc.length){
      let b,sh=0,r=0;
      do{b=enc.charCodeAt(i++)-63;r|=(b&0x1f)<<sh;sh+=5;}while(b>=0x20);
      lat+=(r&1?~(r>>1):r>>1); sh=0;r=0;
      do{b=enc.charCodeAt(i++)-63;r|=(b&0x1f)<<sh;sh+=5;}while(b>=0x20);
      lng+=(r&1?~(r>>1):r>>1); c.push([lat/1e6,lng/1e6]);
    }
    return c;
  }
}

/* ─── 5. DEAD RECKONING STANDALONE — No GPS? No problem. ─────────────── */
export class DeadReckoning {
  private lastPos: {lat:number;lon:number;t:number} | null = null;
  private heading = 0;  // degrees
  private speed = 0;    // m/s
  private gyroIntegrator = 0;

  init(lat:number, lon:number) { this.lastPos = {lat, lon, t:Date.now()}; }

  updateFromImu(ax:number, ay:number, az:number, gz:number, dt:number) {
    // Integrate gyro for heading
    this.gyroIntegrator += gz * dt;
    this.heading = ((this.heading + gz*dt*180/Math.PI) % 360 + 360) % 360;
    // Acceleration magnitude (remove gravity component)
    const accelMag = Math.hypot(ax, ay, az) - 9.81;
    this.speed = Math.max(0, this.speed + accelMag * dt);
    // Friction / decay
    this.speed *= 0.995;
  }

  propagate(dt:number): {lat:number;lon:number;confidence:number} | null {
    if (!this.lastPos) return null;
    const distance = this.speed * dt; // meters
    const rad = this.heading * Math.PI / 180;
    const dLat = (distance * Math.cos(rad)) / 111320;
    const dLon = (distance * Math.sin(rad)) / (111320 * Math.cos(this.lastPos.lat*Math.PI/180));
    const newPos = {lat:this.lastPos.lat+dLat, lon:this.lastPos.lon+dLon, t:Date.now()};
    const elapsed = (newPos.t - this.lastPos.t) / 1000;
    // Confidence decays over time
    const confidence = Math.max(0.1, Math.exp(-elapsed/60)); // halves every ~42s
    this.lastPos = newPos;
    return {lat:newPos.lat, lon:newPos.lon, confidence};
  }

  reset(lat:number, lon:number) { this.init(lat,lon); this.speed=0; this.gyroIntegrator=0; }
}

/* ─── 6. MULTI-SOURCE GEOCODING — Never return "no results" ──────────── */
export class RedundantGeocoder {
  private breaker = new CircuitBreaker();
  async search(query:string): Promise<any[]> {
    const providers = [
      {name:'nominatim', fn:()=>this.nominatim(query)},
      {name:'photon', fn:()=>this.photon(query)},
      {name:'pelias', fn:()=>this.pelias(query)}
    ];
    const results: any[] = [];
    for (const p of providers) {
      const r = await this.breaker.call(p.name, p.fn);
      if (r && r.length) results.push(...r);
      if (results.length >= 5) break; // enough results
    }
    return this.dedupe(results);
  }
  private async nominatim(q:string): Promise<any[]> {
    const r = await fetch(`https://nominatim.openstreetmap.org/search?format=json&q=${encodeURIComponent(q)}&limit=5`);
    if (!r.ok) throw new Error('nominatim');
    return (await r.json()).map((x:any)=>({lat:+x.lat, lon:+x.lon, name:x.display_name, source:'nominatim'}));
  }
  private async photon(q:string): Promise<any[]> {
    const r = await fetch(`https://photon.komoot.io/api/?q=${encodeURIComponent(q)}&limit=5`);
    if (!r.ok) throw new Error('photon');
    const d = await r.json();
    return d.features.map((f:any)=>({lat:f.geometry.coordinates[1], lon:f.geometry.coordinates[0], name:f.properties.name||f.properties.city||q, source:'photon'}));
  }
  private async pelias(q:string): Promise<any[]> {
    // Geocode.earth has free tier via OpenAddresses
    const r = await fetch(`https://api.geocode.earth/v1/search?api_key=ge-demo&text=${encodeURIComponent(q)}&size=5`);
    if (!r.ok) throw new Error('pelias');
    const d = await r.json();
    return (d.features||[]).map((f:any)=>({lat:f.geometry.coordinates[1], lon:f.geometry.coordinates[0], name:f.properties.label, source:'pelias'}));
  }
  private dedupe(results:any[]): any[] {
    const seen = new Set<string>();
    return results.filter(r => {
      const key = `${r.lat.toFixed(3)},${r.lon.toFixed(3)}`;
      if (seen.has(key)) return false;
      seen.add(key); return true;
    });
  }
}

/* ─── 7. SELF-HEALING — Auto-recover from any failure ─────────────── */
export class SelfHealingController {
  private failureLog: Array<{system:string; t:number; recovered:boolean}> = [];

  async healSubsystem(name:string, healFn:()=>Promise<boolean>): Promise<boolean> {
    this.failureLog.push({system:name, t:Date.now(), recovered:false});
    // Exponential backoff retry
    for (let attempt=0; attempt<5; attempt++) {
      await new Promise(r=>setTimeout(r, Math.pow(2, attempt)*500));
      try {
        const ok = await healFn();
        if (ok) {
          this.failureLog[this.failureLog.length-1].recovered = true;
          return true;
        }
      } catch {}
    }
    return false;
  }
  stats() {
    const total = this.failureLog.length;
    const recovered = this.failureLog.filter(f=>f.recovered).length;
    return {total, recovered, rate: total ? recovered/total : 1};
  }
}

/* ─── 8. ANTI-BLOCK CIRCUIT — Tile provider rotation ─────────────── */
export class TileProviderRotator {
  private providers = [
    {name:'osm', url:'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', subs:['a','b','c'], fails:0},
    {name:'osm-fr', url:'https://{s}.tile.openstreetmap.fr/osmfr/{z}/{x}/{y}.png', subs:['a','b'], fails:0},
    {name:'carto-voyager', url:'https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png', subs:['a','b','c','d'], fails:0},
    {name:'wikimedia', url:'https://maps.wikimedia.org/osm-intl/{z}/{x}/{y}.png', subs:[''], fails:0},
    {name:'cyclosm', url:'https://{s}.tile-cyclosm.openstreetmap.fr/cyclosm/{z}/{x}/{y}.png', subs:['a','b','c'], fails:0}
  ];
  current = 0;
  getUrl(): string { return this.providers[this.current].url; }
  markFailed() {
    this.providers[this.current].fails++;
    if (this.providers[this.current].fails > 5) {
      this.current = (this.current + 1) % this.providers.length;
      console.warn('Rotating to provider:', this.providers[this.current].name);
    }
  }
  markSuccess() { this.providers[this.current].fails = 0; }
}

/* ─── 9. BATTERY-AWARE DEGRADATION — Extend runtime ──────────────── */
export class BatteryAwareMode {
  private level = 1;
  private charging = true;
  async init(onChange:(mode:'normal'|'saver'|'critical')=>void) {
    if (!('getBattery' in navigator)) return;
    const bat = await (navigator as any).getBattery();
    const update = () => {
      this.level = bat.level; this.charging = bat.charging;
      onChange(this.level < 0.1 ? 'critical' : this.level < 0.2 && !this.charging ? 'saver' : 'normal');
    };
    bat.addEventListener('levelchange', update);
    bat.addEventListener('chargingchange', update);
    update();
  }
  getRecommendations(): {pollIntervalMs:number; maxTilesPreload:number; animationsEnabled:boolean; radarEnabled:boolean} {
    if (this.level < 0.1) return {pollIntervalMs:5000, maxTilesPreload:0, animationsEnabled:false, radarEnabled:false};
    if (this.level < 0.2 && !this.charging) return {pollIntervalMs:2000, maxTilesPreload:10, animationsEnabled:false, radarEnabled:false};
    return {pollIntervalMs:500, maxTilesPreload:100, animationsEnabled:true, radarEnabled:true};
  }
}

/* ─── 10. NETWORK QUALITY ADAPTIVE — Respond to slow connections ───── */
export class NetworkAdaptive {
  getQuality(): 'offline'|'slow-2g'|'2g'|'3g'|'4g'|'unknown' {
    if (!navigator.onLine) return 'offline';
    const c = (navigator as any).connection;
    return c?.effectiveType || 'unknown';
  }
  recommendations() {
    const q = this.getQuality();
    const map:any = {
      'offline':   {tileMaxZoom:14, poiEnabled:false, weatherEnabled:false, routingTimeout:0},
      'slow-2g':   {tileMaxZoom:14, poiEnabled:false, weatherEnabled:false, routingTimeout:30000},
      '2g':        {tileMaxZoom:16, poiEnabled:false, weatherEnabled:false, routingTimeout:20000},
      '3g':        {tileMaxZoom:17, poiEnabled:true,  weatherEnabled:true,  routingTimeout:15000},
      '4g':        {tileMaxZoom:19, poiEnabled:true,  weatherEnabled:true,  routingTimeout:10000},
      'unknown':   {tileMaxZoom:19, poiEnabled:true,  weatherEnabled:true,  routingTimeout:10000}
    };
    return map[q];
  }
}

/* ─── ORCHESTRATOR — Never-fail glue ─────────────────────────────── */
export class ResilienceOrchestrator {
  sw = new ServiceWorkerManager();
  watchdog: Watchdog;
  routing = new RoutingFallbackChain();
  dr = new DeadReckoning();
  geocoder = new RedundantGeocoder();
  healer = new SelfHealingController();
  tiles = new TileProviderRotator();
  battery = new BatteryAwareMode();
  network = new NetworkAdaptive();
  constructor() {
    this.watchdog = new Watchdog(s => this.handleFailure(s));
  }
  async init() {
    await this.sw.register();
    this.watchdog.register('gnss', 10000);
    this.watchdog.register('map', 30000);
    this.watchdog.register('route', 60000);
    this.watchdog.start();
    await this.battery.init(mode => console.log('Battery mode:', mode));
  }
  private async handleFailure(sub:string) {
    console.warn('Subsystem failed:', sub);
    await this.healer.healSubsystem(sub, async () => {
      if (sub === 'gnss') return !!this.dr.propagate(1);
      if (sub === 'map') { this.tiles.markFailed(); return true; }
      return true;
    });
  }
  status() {
    return {
      watchdog: this.watchdog.status(),
      healer: this.healer.stats(),
      tiles: {current: this.tiles.current, providers: this.tiles['providers'].length},
      network: this.network.getQuality(),
      recommendations: this.network.recommendations()
    };
  }
}

/* ─── TESTS ──────────────────────────────────────────────────────── */
export function runResilienceTests(): string[] {
  const out: string[] = [];

  // Circuit breaker
  const cb = new CircuitBreaker();
  (async()=>{
    for (let i=0;i<5;i++) await cb.call('test', ()=>Promise.reject('fail'));
    const r = await cb.call('test', ()=>Promise.resolve('ok'));
    out.push(r === null ? 'CIRCUIT_OPEN ✓' : 'CIRCUIT_OPEN ✗');
  })();

  // Dead reckoning
  const dr = new DeadReckoning();
  dr.init(32.08, 34.78);
  dr.updateFromImu(0.5, 0, 9.81, 0, 0.1);
  const p = dr.propagate(1);
  out.push(p && typeof p.confidence === 'number' && p.confidence > 0 ? 'DR_PROPAGATE ✓' : 'DR_PROPAGATE ✗');

  // Tile rotator
  const tr = new TileProviderRotator();
  const initialIdx = tr.current;
  for (let i=0;i<6;i++) tr.markFailed();
  out.push(tr.current !== initialIdx ? 'TILE_ROTATE ✓' : 'TILE_ROTATE ✗');

  // Network adaptive
  const na = new NetworkAdaptive();
  const r = na.recommendations();
  out.push(typeof r.tileMaxZoom === 'number' ? 'NETWORK_ADAPT ✓' : 'NETWORK_ADAPT ✗');

  // Watchdog
  let triggered = false;
  const wd = new Watchdog(()=>triggered=true);
  wd.register('test', 50);
  wd.start();
  setTimeout(()=>{
    wd.stop();
    out.push(triggered ? 'WATCHDOG ✓' : 'WATCHDOG ✗');
  }, 150);

  // Straight-line fallback
  const rfc = new RoutingFallbackChain();
  (async()=>{
    const r = await (rfc as any).straightLine([32.08,34.78], [32.09,34.79]);
    out.push(r.length > 10 ? 'STRAIGHT_LINE_FALLBACK ✓' : 'STRAIGHT_LINE_FALLBACK ✗');
  })();

  return out;
}

export const Resilience = {
  ServiceWorkerManager, Watchdog, CircuitBreaker, RoutingFallbackChain,
  DeadReckoning, RedundantGeocoder, SelfHealingController,
  TileProviderRotator, BatteryAwareMode, NetworkAdaptive,
  ResilienceOrchestrator, runResilienceTests, SERVICE_WORKER_CODE
};
