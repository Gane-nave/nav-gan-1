/**
 * G.A.N.E DB — Zero-dependency JSONL persistence
 * Append-only log files: missions.jsonl, events.jsonl
 * Indexed by id in memory on load. Write-through to disk on every mutation.
 *
 * Trade-off: simpler than SQLite, slower for large datasets, but
 * zero supply chain, zero compile, runs on any Node ≥18.
 */
const fs = require('fs');
const path = require('path');

const DATA_DIR = process.env.GANE_DATA_DIR || path.join(__dirname, 'data');
if (!fs.existsSync(DATA_DIR)) fs.mkdirSync(DATA_DIR, { recursive: true });

const MISSIONS_FILE = path.join(DATA_DIR, 'missions.jsonl');
const EVENTS_FILE   = path.join(DATA_DIR, 'events.jsonl');

// In-memory indexes
const missions = new Map();   // id -> mission
const events = [];            // ordered list
let nextMissionId = 1;
let nextEventId = 1;

function loadFile(filename, onRecord) {
  if (!fs.existsSync(filename)) return 0;
  const content = fs.readFileSync(filename, 'utf8');
  let count = 0;
  for (const line of content.split('\n')) {
    if (!line.trim()) continue;
    try { onRecord(JSON.parse(line)); count++; }
    catch (e) { console.warn('[db] skipping malformed line in', filename); }
  }
  return count;
}

function load() {
  // Load missions
  loadFile(MISSIONS_FILE, m => {
    if (m._deleted) { missions.delete(m.id); return; }
    missions.set(m.id, m);
    if (m.id >= nextMissionId) nextMissionId = m.id + 1;
  });
  // Load events
  loadFile(EVENTS_FILE, e => {
    events.push(e);
    if (e.id >= nextEventId) nextEventId = e.id + 1;
  });
  console.log('[db] loaded', missions.size, 'missions,', events.length, 'events from', DATA_DIR);
}

function appendLine(filename, obj) {
  fs.appendFileSync(filename, JSON.stringify(obj) + '\n');
}

// Public API
const db = {
  DATA_DIR,
  MISSIONS_FILE,
  EVENTS_FILE,

  createMission({ origin_lat, origin_lng, dest_lat, dest_lng, metadata }) {
    const m = {
      id: nextMissionId++,
      created_at: Date.now(),
      origin_lat, origin_lng,
      dest_lat: dest_lat ?? null,
      dest_lng: dest_lng ?? null,
      status: 'created',
      metadata: metadata ?? null
    };
    missions.set(m.id, m);
    appendLine(MISSIONS_FILE, m);
    return m;
  },

  getMission(id) {
    return missions.get(id) || null;
  },

  listMissions(limit = 50) {
    return Array.from(missions.values())
      .sort((a, b) => b.created_at - a.created_at)
      .slice(0, limit);
  },

  deleteMission(id) {
    if (!missions.has(id)) return false;
    missions.delete(id);
    appendLine(MISSIONS_FILE, { id, _deleted: true });
    return true;
  },

  appendEvent(mission_id, type, payload) {
    if (!missions.has(mission_id)) return null;
    const e = {
      id: nextEventId++,
      mission_id, type,
      payload: payload ?? null,
      timestamp: Date.now()
    };
    events.push(e);
    appendLine(EVENTS_FILE, e);
    return e;
  },

  getEventsForMission(mission_id) {
    return events.filter(e => e.mission_id === mission_id)
      .sort((a, b) => a.timestamp - b.timestamp);
  },

  stats() {
    return {
      missions: missions.size,
      events: events.length,
      data_dir: DATA_DIR
    };
  },

  // For testing: wipe all data (does not delete files)
  reset() {
    missions.clear();
    events.length = 0;
    nextMissionId = 1;
    nextEventId = 1;
    if (fs.existsSync(MISSIONS_FILE)) fs.unlinkSync(MISSIONS_FILE);
    if (fs.existsSync(EVENTS_FILE)) fs.unlinkSync(EVENTS_FILE);
  }
};

load();
module.exports = db;
