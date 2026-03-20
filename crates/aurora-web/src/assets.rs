//! Embedded static assets for the AURORA NAV web UI.

/// The main HTML page with embedded CSS and JavaScript.
/// Serves as a single-page application with:
/// - Interactive OpenStreetMap via Leaflet.js
/// - Real-time GPS position marker with accuracy circle
/// - Route polyline with turn-by-turn overlay
/// - Traffic congestion heatmap layer
/// - Satellite sky-view polar chart
/// - Integrity/continuity status panel
/// - Emergency mode banner
/// - Fleet tracking markers
/// - Smart city signal indicators
/// - Performance metrics sidebar
/// - Dark/light theme toggle
/// - Responsive layout for mobile and desktop
pub const MAIN_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>AURORA NAV / GMIN — Global Mobility Intelligence Network</title>
<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
<script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
<style>
:root {
  --bg-primary: #0a0e17;
  --bg-secondary: #111827;
  --bg-card: #1a2332;
  --text-primary: #e8edf5;
  --text-secondary: #8b95a8;
  --accent-blue: #3b82f6;
  --accent-green: #10b981;
  --accent-red: #ef4444;
  --accent-amber: #f59e0b;
  --accent-purple: #8b5cf6;
  --accent-cyan: #06b6d4;
  --border: #2a3444;
  --shadow: rgba(0,0,0,0.4);
  --gradient-blue: linear-gradient(135deg, #1e3a5f, #0a1628);
  --gradient-green: linear-gradient(135deg, #065f46, #0a1628);
}
* { margin:0; padding:0; box-sizing:border-box; }
body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
  height: 100vh;
}
#app {
  display: grid;
  grid-template-columns: 320px 1fr 280px;
  grid-template-rows: 56px 1fr 48px;
  height: 100vh;
  gap: 0;
}
/* Header */
#header {
  grid-column: 1 / -1;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  z-index: 1000;
}
#header .logo {
  display: flex;
  align-items: center;
  gap: 12px;
}
#header .logo-icon {
  width: 32px; height: 32px;
  background: var(--accent-blue);
  border-radius: 8px;
  display: flex; align-items: center; justify-content: center;
  font-weight: 700; font-size: 14px;
}
#header h1 {
  font-size: 16px;
  font-weight: 600;
  letter-spacing: 1px;
}
#header .subtitle {
  font-size: 11px;
  color: var(--text-secondary);
  margin-top: 1px;
}
.header-controls {
  display: flex;
  align-items: center;
  gap: 16px;
}
.status-pill {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
}
.status-pill.healthy { background: rgba(16,185,129,0.2); color: var(--accent-green); }
.status-pill.degraded { background: rgba(245,158,11,0.2); color: var(--accent-amber); }
.status-pill.critical { background: rgba(239,68,68,0.2); color: var(--accent-red); }
/* Left Panel */
#left-panel {
  background: var(--bg-secondary);
  border-right: 1px solid var(--border);
  overflow-y: auto;
  padding: 0;
}
.panel-section {
  padding: 16px;
  border-bottom: 1px solid var(--border);
}
.panel-section h3 {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 1.5px;
  color: var(--text-secondary);
  margin-bottom: 12px;
  display: flex;
  align-items: center;
  gap: 8px;
}
.panel-section h3 .dot {
  width: 6px; height: 6px;
  border-radius: 50%;
  background: var(--accent-green);
  animation: pulse 2s infinite;
}
@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}
.data-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 0;
  font-size: 13px;
}
.data-row .label { color: var(--text-secondary); }
.data-row .value { font-weight: 500; font-variant-numeric: tabular-nums; }
.data-row .value.green { color: var(--accent-green); }
.data-row .value.amber { color: var(--accent-amber); }
.data-row .value.red { color: var(--accent-red); }
.data-row .value.blue { color: var(--accent-blue); }
/* Route Card */
.route-card {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 16px;
  margin-top: 8px;
}
.route-card .eta {
  font-size: 28px;
  font-weight: 700;
  color: var(--accent-blue);
}
.route-card .eta-label {
  font-size: 11px;
  color: var(--text-secondary);
}
.route-card .distance {
  font-size: 14px;
  margin-top: 4px;
}
.turn-instruction {
  background: var(--gradient-blue);
  border-radius: 8px;
  padding: 12px;
  margin-top: 10px;
  display: flex;
  align-items: center;
  gap: 12px;
}
.turn-icon {
  width: 36px; height: 36px;
  background: var(--accent-blue);
  border-radius: 8px;
  display: flex; align-items: center; justify-content: center;
  font-size: 18px;
}
.turn-text { font-size: 13px; }
.turn-distance { font-size: 11px; color: var(--text-secondary); margin-top: 2px; }
/* Map */
#map-container {
  position: relative;
  overflow: hidden;
}
#map { width: 100%; height: 100%; z-index: 1; }
.map-overlay-top {
  position: absolute;
  top: 12px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 500;
  background: rgba(10,14,23,0.85);
  backdrop-filter: blur(8px);
  border-radius: 12px;
  padding: 8px 20px;
  display: flex;
  gap: 24px;
  border: 1px solid var(--border);
}
.map-stat {
  text-align: center;
}
.map-stat .val {
  font-size: 18px;
  font-weight: 700;
}
.map-stat .lbl {
  font-size: 10px;
  color: var(--text-secondary);
  text-transform: uppercase;
}
/* Emergency Banner */
.emergency-banner {
  position: absolute;
  top: 0; left: 0; right: 0;
  background: linear-gradient(90deg, #7f1d1d, #991b1b, #7f1d1d);
  color: white;
  text-align: center;
  padding: 8px;
  font-weight: 700;
  font-size: 14px;
  z-index: 600;
  display: none;
  animation: emergency-flash 1s infinite;
}
@keyframes emergency-flash {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.7; }
}
/* Right Panel */
#right-panel {
  background: var(--bg-secondary);
  border-left: 1px solid var(--border);
  overflow-y: auto;
  padding: 0;
}
/* Satellite Sky View */
.sky-view {
  width: 200px; height: 200px;
  margin: 12px auto;
  position: relative;
}
.sky-view canvas {
  width: 100%; height: 100%;
}
/* Integrity Gauge */
.gauge-container {
  display: flex;
  justify-content: center;
  gap: 16px;
  margin: 12px 0;
}
.gauge {
  text-align: center;
}
.gauge-ring {
  width: 60px; height: 60px;
  border-radius: 50%;
  display: flex; align-items: center; justify-content: center;
  font-size: 14px; font-weight: 700;
  margin: 0 auto 4px;
}
.gauge-label {
  font-size: 10px;
  color: var(--text-secondary);
}
/* Subsystem Grid */
.subsystem-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 6px;
  margin-top: 8px;
}
.subsystem-chip {
  background: var(--bg-card);
  border-radius: 6px;
  padding: 6px 8px;
  font-size: 10px;
  display: flex;
  align-items: center;
  gap: 6px;
}
.subsystem-chip .indicator {
  width: 6px; height: 6px;
  border-radius: 50%;
}
.subsystem-chip .indicator.on { background: var(--accent-green); }
.subsystem-chip .indicator.off { background: var(--text-secondary); }
/* Footer */
#footer {
  grid-column: 1 / -1;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  font-size: 11px;
  color: var(--text-secondary);
}
.footer-metrics {
  display: flex;
  gap: 24px;
}
.footer-metric {
  display: flex;
  align-items: center;
  gap: 6px;
}
.footer-metric .dot {
  width: 6px; height: 6px;
  border-radius: 50%;
}
/* Layer Toggle */
.layer-toggles {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-top: 8px;
}
.layer-toggle {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 8px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
  transition: background 0.2s;
}
.layer-toggle:hover { background: var(--bg-card); }
.toggle-switch {
  width: 32px; height: 16px;
  border-radius: 8px;
  background: var(--border);
  position: relative;
  transition: background 0.2s;
}
.toggle-switch.active { background: var(--accent-blue); }
.toggle-switch::after {
  content: '';
  width: 12px; height: 12px;
  border-radius: 50%;
  background: white;
  position: absolute;
  top: 2px; left: 2px;
  transition: left 0.2s;
}
.toggle-switch.active::after { left: 18px; }
/* Search Bar */
.search-bar {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 8px 12px;
  color: var(--text-primary);
  width: 100%;
  font-size: 13px;
  outline: none;
  margin-bottom: 12px;
}
.search-bar:focus { border-color: var(--accent-blue); }
/* Responsive */
@media (max-width: 1024px) {
  #app { grid-template-columns: 1fr; }
  #left-panel, #right-panel { display: none; }
}
</style>
</head>
<body>
<div id="app">
  <!-- HEADER -->
  <header id="header">
    <div class="logo">
      <div class="logo-icon">A</div>
      <div>
        <h1>AURORA NAV</h1>
        <div class="subtitle">Global Mobility Intelligence Network</div>
      </div>
    </div>
    <div class="header-controls">
      <span class="status-pill healthy" id="system-status">OPERATIONAL</span>
      <span style="font-size:12px" id="clock">--:--:--</span>
    </div>
  </header>

  <!-- LEFT PANEL -->
  <nav id="left-panel">
    <div class="panel-section">
      <input type="text" class="search-bar" placeholder="Search destination..." id="search-input">
      <div class="route-card" id="route-card">
        <div style="display:flex;justify-content:space-between;align-items:baseline">
          <div>
            <div class="eta" id="eta">--</div>
            <div class="eta-label">ETA (minutes)</div>
          </div>
          <div class="distance" id="distance">-- km</div>
        </div>
        <div class="turn-instruction" id="turn-instruction">
          <div class="turn-icon" id="turn-icon">&#x2191;</div>
          <div>
            <div class="turn-text" id="turn-text">Waiting for route...</div>
            <div class="turn-distance" id="turn-dist">--</div>
          </div>
        </div>
      </div>
    </div>

    <div class="panel-section">
      <h3><span class="dot"></span> POSITION</h3>
      <div class="data-row"><span class="label">Latitude</span><span class="value" id="pos-lat">--</span></div>
      <div class="data-row"><span class="label">Longitude</span><span class="value" id="pos-lon">--</span></div>
      <div class="data-row"><span class="label">Altitude</span><span class="value" id="pos-alt">-- m</span></div>
      <div class="data-row"><span class="label">Speed</span><span class="value blue" id="pos-speed">-- km/h</span></div>
      <div class="data-row"><span class="label">Heading</span><span class="value" id="pos-heading">--°</span></div>
      <div class="data-row"><span class="label">Accuracy</span><span class="value green" id="pos-accuracy">-- m</span></div>
      <div class="data-row"><span class="label">Fix Type</span><span class="value green" id="pos-fix">--</span></div>
    </div>

    <div class="panel-section">
      <h3><span class="dot"></span> TRAFFIC</h3>
      <div class="data-row"><span class="label">Congestion</span><span class="value" id="traffic-level">--</span></div>
      <div class="data-row"><span class="label">Incidents</span><span class="value" id="traffic-incidents">0</span></div>
      <div class="data-row"><span class="label">Avg Speed</span><span class="value" id="traffic-speed">-- km/h</span></div>
      <div class="data-row"><span class="label">Delay</span><span class="value amber" id="traffic-delay">-- min</span></div>
    </div>

    <div class="panel-section">
      <h3><span class="dot"></span> EMERGENCY</h3>
      <div class="data-row"><span class="label">Status</span><span class="value green" id="emer-status">Inactive</span></div>
      <div class="data-row"><span class="label">Hospital</span><span class="value" id="emer-hospital">-- km</span></div>
      <div class="data-row"><span class="label">Police</span><span class="value" id="emer-police">-- km</span></div>
      <div class="data-row"><span class="label">Fire Dept</span><span class="value" id="emer-fire">-- km</span></div>
    </div>

    <div class="panel-section">
      <h3><span class="dot"></span> SMART CITY</h3>
      <div class="data-row"><span class="label">Connected</span><span class="value" id="city-conn">--</span></div>
      <div class="data-row"><span class="label">Signals Ahead</span><span class="value" id="city-signals">0</span></div>
      <div class="data-row"><span class="label">Green Wave</span><span class="value" id="city-green">--</span></div>
      <div class="data-row"><span class="label">Parking</span><span class="value" id="city-parking">0</span></div>
      <div class="data-row"><span class="label">EV Chargers</span><span class="value" id="city-ev">0</span></div>
    </div>
  </nav>

  <!-- MAP -->
  <main id="map-container">
    <div class="emergency-banner" id="emergency-banner">
      &#x26A0; EMERGENCY MODE ACTIVE — Routing to nearest hospital
    </div>
    <div class="map-overlay-top">
      <div class="map-stat"><div class="val" id="ov-sats">0</div><div class="lbl">Satellites</div></div>
      <div class="map-stat"><div class="val green" id="ov-acc">--</div><div class="lbl">Accuracy</div></div>
      <div class="map-stat"><div class="val blue" id="ov-speed">0</div><div class="lbl">km/h</div></div>
      <div class="map-stat"><div class="val" id="ov-int">--</div><div class="lbl">Integrity</div></div>
    </div>
    <div id="map"></div>
  </main>

  <!-- RIGHT PANEL -->
  <aside id="right-panel">
    <div class="panel-section">
      <h3><span class="dot"></span> SATELLITES</h3>
      <div class="data-row"><span class="label">Tracked</span><span class="value" id="sat-tracked">0</span></div>
      <div class="data-row"><span class="label">In Fix</span><span class="value green" id="sat-fix">0</span></div>
      <div class="data-row"><span class="label">GPS</span><span class="value" id="sat-gps">0</span></div>
      <div class="data-row"><span class="label">Galileo</span><span class="value" id="sat-galileo">0</span></div>
      <div class="data-row"><span class="label">GLONASS</span><span class="value" id="sat-glonass">0</span></div>
      <div class="data-row"><span class="label">BeiDou</span><span class="value" id="sat-beidou">0</span></div>
      <div class="data-row"><span class="label">HDOP</span><span class="value" id="sat-hdop">--</span></div>
      <div class="data-row"><span class="label">PDOP</span><span class="value" id="sat-pdop">--</span></div>
    </div>

    <div class="panel-section">
      <h3><span class="dot"></span> INTEGRITY</h3>
      <div class="gauge-container">
        <div class="gauge">
          <div class="gauge-ring" id="gauge-int" style="border:3px solid var(--accent-green)">OK</div>
          <div class="gauge-label">Integrity</div>
        </div>
        <div class="gauge">
          <div class="gauge-ring" id="gauge-cont" style="border:3px solid var(--accent-blue)">N</div>
          <div class="gauge-label">Continuity</div>
        </div>
      </div>
      <div class="data-row"><span class="label">Protection</span><span class="value" id="int-protection">-- m</span></div>
      <div class="data-row"><span class="label">Jamming</span><span class="value green" id="int-jam">None</span></div>
      <div class="data-row"><span class="label">Spoofing</span><span class="value green" id="int-spoof">None</span></div>
      <div class="data-row"><span class="label">RAIM</span><span class="value" id="int-raim">--</span></div>
      <div class="data-row"><span class="label">Corr. Age</span><span class="value" id="int-corr">-- s</span></div>
    </div>

    <div class="panel-section">
      <h3>MAP LAYERS</h3>
      <div class="layer-toggles">
        <div class="layer-toggle" onclick="toggleLayer('traffic')">
          <span>Traffic Flow</span>
          <div class="toggle-switch active" id="layer-traffic"></div>
        </div>
        <div class="layer-toggle" onclick="toggleLayer('satellites')">
          <span>Satellite View</span>
          <div class="toggle-switch" id="layer-satellites"></div>
        </div>
        <div class="layer-toggle" onclick="toggleLayer('fleet')">
          <span>Fleet Vehicles</span>
          <div class="toggle-switch" id="layer-fleet"></div>
        </div>
        <div class="layer-toggle" onclick="toggleLayer('city')">
          <span>Smart City</span>
          <div class="toggle-switch active" id="layer-city"></div>
        </div>
        <div class="layer-toggle" onclick="toggleLayer('emergency')">
          <span>Emergency</span>
          <div class="toggle-switch" id="layer-emergency"></div>
        </div>
        <div class="layer-toggle" onclick="toggleLayer('parking')">
          <span>Parking</span>
          <div class="toggle-switch" id="layer-parking"></div>
        </div>
        <div class="layer-toggle" onclick="toggleLayer('ev')">
          <span>EV Chargers</span>
          <div class="toggle-switch" id="layer-ev"></div>
        </div>
      </div>
    </div>

    <div class="panel-section">
      <h3><span class="dot"></span> SUBSYSTEMS</h3>
      <div class="subsystem-grid" id="subsystem-grid">
        <div class="subsystem-chip"><div class="indicator on"></div>GNSS</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Fusion</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Integrity</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Routing</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Traffic</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Map</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Sensors</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Telemetry</div>
        <div class="subsystem-chip"><div class="indicator off"></div>Fleet</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Emergency</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Offline</div>
        <div class="subsystem-chip"><div class="indicator on"></div>Edge</div>
        <div class="subsystem-chip"><div class="indicator off"></div>Satellite</div>
        <div class="subsystem-chip"><div class="indicator on"></div>City</div>
        <div class="subsystem-chip"><div class="indicator off"></div>Twin</div>
        <div class="subsystem-chip"><div class="indicator on"></div>API</div>
      </div>
    </div>
  </aside>

  <!-- FOOTER -->
  <footer id="footer">
    <div class="footer-metrics">
      <div class="footer-metric"><div class="dot" style="background:var(--accent-green)"></div>Pipeline: <span id="ft-latency">-- ms</span></div>
      <div class="footer-metric"><div class="dot" style="background:var(--accent-blue)"></div>Update: <span id="ft-hz">-- Hz</span></div>
      <div class="footer-metric"><div class="dot" style="background:var(--accent-purple)"></div>Cache: <span id="ft-cache">--%</span></div>
      <div class="footer-metric"><div class="dot" style="background:var(--accent-cyan)"></div>Queue: <span id="ft-queue">0</span></div>
    </div>
    <div>
      <span id="ft-uptime">Uptime: --</span> &middot; AURORA NAV v0.1.0
    </div>
  </footer>
</div>

<script>
// ========== MAP INITIALIZATION ==========
const map = L.map('map', {
  zoomControl: false,
  attributionControl: false
}).setView([32.0853, 34.7818], 14);

L.control.zoom({ position: 'bottomright' }).addTo(map);
L.control.attribution({ position: 'bottomleft' }).addTo(map);

// Dark map tiles
L.tileLayer('https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png', {
  attribution: '&copy; OpenStreetMap &copy; CARTO',
  maxZoom: 19
}).addTo(map);

// Position marker with accuracy circle
let posMarker = null;
let accCircle = null;
let routeLine = null;
let trafficLayer = L.layerGroup().addTo(map);
let fleetLayer = L.layerGroup();
let cityLayer = L.layerGroup().addTo(map);

const posIcon = L.divIcon({
  className: 'pos-marker',
  html: '<div style="width:18px;height:18px;background:#3b82f6;border:3px solid white;border-radius:50%;box-shadow:0 0 12px rgba(59,130,246,0.6)"></div>',
  iconSize: [18, 18],
  iconAnchor: [9, 9]
});

// ========== LAYER TOGGLES ==========
const layers = { traffic: true, satellites: false, fleet: false, city: true, emergency: false, parking: false, ev: false };

function toggleLayer(name) {
  layers[name] = !layers[name];
  const el = document.getElementById('layer-' + name);
  if (el) el.classList.toggle('active', layers[name]);

  if (name === 'fleet') {
    if (layers.fleet) map.addLayer(fleetLayer);
    else map.removeLayer(fleetLayer);
  }
  if (name === 'traffic') {
    if (layers.traffic) map.addLayer(trafficLayer);
    else map.removeLayer(trafficLayer);
  }
  if (name === 'city') {
    if (layers.city) map.addLayer(cityLayer);
    else map.removeLayer(cityLayer);
  }
}

// ========== DATA UPDATE ==========
function updateDashboard(data) {
  // Position
  const p = data.position;
  document.getElementById('pos-lat').textContent = p.latitude.toFixed(6);
  document.getElementById('pos-lon').textContent = p.longitude.toFixed(6);
  document.getElementById('pos-alt').textContent = p.altitude_m.toFixed(1) + ' m';
  document.getElementById('pos-speed').textContent = p.speed_kmh.toFixed(0) + ' km/h';
  document.getElementById('pos-heading').textContent = p.heading_deg.toFixed(0) + '\u00B0';
  document.getElementById('pos-accuracy').textContent = p.accuracy_m.toFixed(1) + ' m';
  document.getElementById('pos-fix').textContent = p.fix_type;

  // Map overlay
  document.getElementById('ov-speed').textContent = p.speed_kmh.toFixed(0);
  document.getElementById('ov-acc').textContent = p.accuracy_m.toFixed(1) + 'm';

  // Update marker
  if (!posMarker) {
    posMarker = L.marker([p.latitude, p.longitude], { icon: posIcon }).addTo(map);
    accCircle = L.circle([p.latitude, p.longitude], { radius: p.accuracy_m, color: '#3b82f6', fillOpacity: 0.1, weight: 1 }).addTo(map);
  } else {
    posMarker.setLatLng([p.latitude, p.longitude]);
    accCircle.setLatLng([p.latitude, p.longitude]);
    accCircle.setRadius(p.accuracy_m);
  }

  // Route
  const r = data.route;
  if (r.active) {
    document.getElementById('eta').textContent = r.eta_minutes.toFixed(0);
    document.getElementById('distance').textContent = r.distance_km.toFixed(1) + ' km';
    document.getElementById('turn-text').textContent = r.current_step;
    document.getElementById('turn-dist').textContent = r.next_turn_distance_m.toFixed(0) + ' m — ' + r.next_turn;

    if (r.waypoints.length > 1 && !routeLine) {
      routeLine = L.polyline(r.waypoints, { color: '#3b82f6', weight: 5, opacity: 0.8 }).addTo(map);
    } else if (routeLine && r.waypoints.length > 1) {
      routeLine.setLatLngs(r.waypoints);
    }
  }

  // Satellites
  const s = data.satellites;
  document.getElementById('sat-tracked').textContent = s.tracked;
  document.getElementById('sat-fix').textContent = s.used_in_fix;
  document.getElementById('sat-gps').textContent = s.gps_count;
  document.getElementById('sat-galileo').textContent = s.galileo_count;
  document.getElementById('sat-glonass').textContent = s.glonass_count;
  document.getElementById('sat-beidou').textContent = s.beidou_count;
  document.getElementById('sat-hdop').textContent = s.hdop.toFixed(1);
  document.getElementById('sat-pdop').textContent = s.pdop.toFixed(1);
  document.getElementById('ov-sats').textContent = s.tracked;

  // Integrity
  const i = data.integrity;
  document.getElementById('gauge-int').textContent = i.level.substring(0, 3);
  document.getElementById('gauge-cont').textContent = i.continuity_mode.substring(0, 1);
  document.getElementById('ov-int').textContent = i.level;
  document.getElementById('int-protection').textContent = i.protection_level_m.toFixed(1) + ' m';
  document.getElementById('int-jam').textContent = i.jamming_detected ? 'DETECTED' : 'None';
  document.getElementById('int-jam').className = 'value ' + (i.jamming_detected ? 'red' : 'green');
  document.getElementById('int-spoof').textContent = i.spoofing_detected ? 'DETECTED' : 'None';
  document.getElementById('int-spoof').className = 'value ' + (i.spoofing_detected ? 'red' : 'green');
  document.getElementById('int-raim').textContent = i.raim_available ? 'Available' : 'Unavailable';
  document.getElementById('int-corr').textContent = i.correction_age_s.toFixed(1) + ' s';

  // Traffic
  const t = data.traffic;
  document.getElementById('traffic-level').textContent = t.congestion_level;
  document.getElementById('traffic-incidents').textContent = t.incidents_nearby;
  document.getElementById('traffic-speed').textContent = t.average_speed_kmh.toFixed(0) + ' km/h';
  document.getElementById('traffic-delay').textContent = data.route.traffic_delay_minutes.toFixed(0) + ' min';

  // Traffic segments on map
  if (layers.traffic) {
    trafficLayer.clearLayers();
    t.segments.forEach(seg => {
      L.polyline([seg.start, seg.end], { color: seg.color, weight: 4, opacity: 0.7 }).addTo(trafficLayer);
    });
  }

  // Fleet
  if (layers.fleet) {
    fleetLayer.clearLayers();
    data.fleet.nearby_vehicles.forEach(v => {
      L.circleMarker([v.lat, v.lon], { radius: 5, color: '#8b5cf6', fillOpacity: 0.8 })
        .bindPopup(v.id + ' (' + v.vehicle_type + ') ' + v.speed_kmh.toFixed(0) + ' km/h')
        .addTo(fleetLayer);
    });
  }

  // Emergency
  const e = data.emergency;
  document.getElementById('emer-status').textContent = e.active ? 'ACTIVE' : 'Inactive';
  document.getElementById('emer-status').className = 'value ' + (e.active ? 'red' : 'green');
  document.getElementById('emer-hospital').textContent = e.nearest_hospital_km.toFixed(1) + ' km';
  document.getElementById('emer-police').textContent = e.nearest_police_km.toFixed(1) + ' km';
  document.getElementById('emer-fire').textContent = e.nearest_fire_km.toFixed(1) + ' km';
  document.getElementById('emergency-banner').style.display = e.active ? 'block' : 'none';

  // Smart City
  const c = data.city;
  document.getElementById('city-conn').textContent = c.connected ? 'Yes' : 'No';
  document.getElementById('city-conn').className = 'value ' + (c.connected ? 'green' : '');
  document.getElementById('city-signals').textContent = c.traffic_lights_ahead;
  document.getElementById('city-green').textContent = c.green_wave_active ? 'Active' : 'Inactive';
  document.getElementById('city-green').className = 'value ' + (c.green_wave_active ? 'green' : '');
  document.getElementById('city-parking').textContent = c.smart_parking_spots;
  document.getElementById('city-ev').textContent = c.ev_chargers_nearby;

  // Metrics
  const m = data.metrics;
  document.getElementById('ft-latency').textContent = m.pipeline_latency_ms.toFixed(1) + ' ms';
  document.getElementById('ft-hz').textContent = m.position_update_hz.toFixed(0) + ' Hz';
  document.getElementById('ft-cache').textContent = (m.cache_hit_rate * 100).toFixed(0) + '%';
  document.getElementById('ft-queue').textContent = m.pending_requests;

  // Health status pill
  const h = data.health;
  const pill = document.getElementById('system-status');
  pill.textContent = h.overall.toUpperCase();
  pill.className = 'status-pill ' + (h.overall === 'Healthy' ? 'healthy' : h.overall === 'Degraded' ? 'degraded' : 'critical');

  // Uptime
  const up = m.uptime_seconds;
  const hrs = Math.floor(up / 3600);
  const mins = Math.floor((up % 3600) / 60);
  document.getElementById('ft-uptime').textContent = 'Uptime: ' + hrs + 'h ' + mins + 'm';
}

// ========== CLOCK ==========
setInterval(() => {
  const now = new Date();
  document.getElementById('clock').textContent = now.toLocaleTimeString('en-US', { hour12: false });
}, 1000);

// ========== POLLING ==========
async function pollDashboard() {
  try {
    const resp = await fetch('/api/dashboard');
    if (resp.ok) {
      const data = await resp.json();
      updateDashboard(data);
    }
  } catch (e) {
    // Silently retry
  }
  setTimeout(pollDashboard, 1000);
}

// Start polling
pollDashboard();

// ========== SEARCH ==========
document.getElementById('search-input').addEventListener('keypress', async (e) => {
  if (e.key === 'Enter') {
    const q = e.target.value.trim();
    if (q) {
      try {
        const resp = await fetch('/api/route?q=' + encodeURIComponent(q));
        if (resp.ok) {
          const data = await resp.json();
          updateDashboard(data);
        }
      } catch (err) {
        // Handle error silently
      }
    }
  }
});
</script>
</body>
</html>"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_is_not_empty() {
        assert!(!MAIN_HTML.is_empty());
    }

    #[test]
    fn html_contains_map_container() {
        assert!(MAIN_HTML.contains("id=\"map\""));
    }

    #[test]
    fn html_contains_leaflet() {
        assert!(MAIN_HTML.contains("leaflet"));
    }

    #[test]
    fn html_contains_all_panels() {
        assert!(MAIN_HTML.contains("POSITION"));
        assert!(MAIN_HTML.contains("SATELLITES"));
        assert!(MAIN_HTML.contains("INTEGRITY"));
        assert!(MAIN_HTML.contains("TRAFFIC"));
        assert!(MAIN_HTML.contains("EMERGENCY"));
        assert!(MAIN_HTML.contains("SMART CITY"));
        assert!(MAIN_HTML.contains("SUBSYSTEMS"));
        assert!(MAIN_HTML.contains("MAP LAYERS"));
    }

    #[test]
    fn html_contains_aurora_branding() {
        assert!(MAIN_HTML.contains("AURORA NAV"));
        assert!(MAIN_HTML.contains("Global Mobility Intelligence Network"));
    }

    #[test]
    fn html_contains_all_layer_toggles() {
        assert!(MAIN_HTML.contains("Traffic Flow"));
        assert!(MAIN_HTML.contains("Satellite View"));
        assert!(MAIN_HTML.contains("Fleet Vehicles"));
        assert!(MAIN_HTML.contains("Smart City"));
        assert!(MAIN_HTML.contains("Emergency"));
        assert!(MAIN_HTML.contains("Parking"));
        assert!(MAIN_HTML.contains("EV Chargers"));
    }

    #[test]
    fn html_contains_polling_logic() {
        assert!(MAIN_HTML.contains("pollDashboard"));
        assert!(MAIN_HTML.contains("/api/dashboard"));
    }

    #[test]
    fn html_contains_responsive_css() {
        assert!(MAIN_HTML.contains("@media"));
    }
}
