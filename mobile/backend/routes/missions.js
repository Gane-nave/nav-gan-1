/**
 * G.A.N.E Missions Routes — pure function dispatcher
 * Returns { status, body } objects. Server adapter handles HTTP.
 */
const db = require('../db');

const isNum = v => typeof v === 'number' && isFinite(v);
const validLat = v => isNum(v) && v >= -90 && v <= 90;
const validLng = v => isNum(v) && v >= -180 && v <= 180;

function shapeMission(m) {
  return {
    id: m.id,
    created_at: m.created_at,
    origin: { lat: m.origin_lat, lng: m.origin_lng },
    dest: m.dest_lat != null ? { lat: m.dest_lat, lng: m.dest_lng } : null,
    status: m.status,
    metadata: m.metadata
  };
}

function handle({ method, pathname, query, body }) {
  // POST /missions  — create
  if (method === 'POST' && pathname === '/missions') {
    if (!body) return { status: 400, body: { error: 'body_required' } };
    const { origin_lat, origin_lng, dest_lat, dest_lng, metadata } = body;
    if (!validLat(origin_lat) || !validLng(origin_lng))
      return { status: 400, body: { error: 'invalid_origin' } };
    if (dest_lat !== undefined && dest_lat !== null && !validLat(dest_lat))
      return { status: 400, body: { error: 'invalid_dest_lat' } };
    if (dest_lng !== undefined && dest_lng !== null && !validLng(dest_lng))
      return { status: 400, body: { error: 'invalid_dest_lng' } };
    const m = db.createMission({ origin_lat, origin_lng, dest_lat, dest_lng, metadata });
    return { status: 201, body: shapeMission(m) };
  }

  // GET /missions — list latest 50
  if (method === 'GET' && pathname === '/missions') {
    const rows = db.listMissions(50);
    return { status: 200, body: { missions: rows.map(shapeMission), count: rows.length } };
  }

  // /missions/:id paths
  const idMatch = pathname.match(/^\/missions\/(\d+)(\/events)?$/);
  if (idMatch) {
    const id = parseInt(idMatch[1], 10);
    if (!Number.isInteger(id) || id < 1)
      return { status: 400, body: { error: 'invalid_id' } };

    // POST /missions/:id/events
    if (method === 'POST' && idMatch[2] === '/events') {
      if (!body) return { status: 400, body: { error: 'body_required' } };
      if (!body.type || typeof body.type !== 'string')
        return { status: 400, body: { error: 'type_required' } };
      const e = db.appendEvent(id, body.type, body.payload);
      if (!e) return { status: 404, body: { error: 'mission_not_found' } };
      return { status: 201, body: e };
    }

    // GET /missions/:id
    if (method === 'GET' && !idMatch[2]) {
      const m = db.getMission(id);
      if (!m) return { status: 404, body: { error: 'not_found' } };
      const events = db.getEventsForMission(id);
      return {
        status: 200,
        body: { mission: shapeMission(m), events, eventCount: events.length }
      };
    }

    // DELETE /missions/:id
    if (method === 'DELETE' && !idMatch[2]) {
      const ok = db.deleteMission(id);
      if (!ok) return { status: 404, body: { error: 'not_found' } };
      return { status: 200, body: { deleted: true, id } };
    }
  }

  return { status: 404, body: { error: 'not_found', method, pathname } };
}

module.exports = handle;
