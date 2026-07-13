/**
 * G.A.N.E Backend Integration Test
 * Pure Node, zero deps. Spawns server, makes real HTTP requests, verifies real DB writes.
 */
const http = require('http');
const fs = require('fs');
const path = require('path');

// Isolated test data dir
const TEST_DIR = path.join(__dirname, 'test-data');
process.env.GANE_DATA_DIR = TEST_DIR;
process.env.PORT = '3099';

if (fs.existsSync(TEST_DIR)) {
  fs.readdirSync(TEST_DIR).forEach(f => fs.unlinkSync(path.join(TEST_DIR, f)));
}

const server = require('./server');

function req(method, p, body) {
  return new Promise((resolve, reject) => {
    const data = body ? JSON.stringify(body) : null;
    const r = http.request({
      hostname: 'localhost', port: 3099, path: p, method,
      headers: { 'Content-Type': 'application/json',
                 ...(data && { 'Content-Length': Buffer.byteLength(data) }) }
    }, res => {
      let buf = '';
      res.on('data', c => buf += c);
      res.on('end', () => {
        try { resolve({ status: res.statusCode, body: buf ? JSON.parse(buf) : null }); }
        catch { resolve({ status: res.statusCode, body: buf }); }
      });
    });
    r.on('error', reject);
    if (data) r.write(data);
    r.end();
  });
}

async function main() {
  let pass = 0, fail = 0;
  const test = async (name, fn) => {
    try { await fn(); console.log('  ✓', name); pass++; }
    catch (e) { console.log('  ✗', name, '—', e.message); fail++; }
  };

  await new Promise(r => setTimeout(r, 200));

  console.log('\n═══ G.A.N.E BACKEND INTEGRATION TEST ═══\n');

  await test('GET /health returns 200 with stats', async () => {
    const r = await req('GET', '/health');
    if (r.status !== 200) throw new Error('status=' + r.status);
    if (r.body.status !== 'ok') throw new Error('status field not ok');
    if (typeof r.body.missions !== 'number') throw new Error('no missions count');
    if (!r.body.node) throw new Error('no node version');
  });

  let missionId;
  await test('POST /missions creates and returns mission', async () => {
    const r = await req('POST', '/missions', {
      origin_lat: 32.0853, origin_lng: 34.7818,
      dest_lat: 32.0892, dest_lng: 34.7878,
      metadata: { source: 'integration-test', user: 'tester' }
    });
    if (r.status !== 201) throw new Error('status=' + r.status + ' body=' + JSON.stringify(r.body));
    if (!r.body.id) throw new Error('no id');
    if (r.body.status !== 'created') throw new Error('wrong status');
    if (r.body.origin.lat !== 32.0853) throw new Error('lat mismatch');
    missionId = r.body.id;
  });

  await test('POST /missions rejects invalid origin coordinates', async () => {
    const r = await req('POST', '/missions', { origin_lat: 999, origin_lng: 0 });
    if (r.status !== 400) throw new Error('expected 400 got ' + r.status);
    if (r.body.error !== 'invalid_origin') throw new Error('wrong error: ' + r.body.error);
  });

  await test('POST /missions rejects missing body', async () => {
    const r = await req('POST', '/missions');
    if (r.status !== 400) throw new Error('expected 400 got ' + r.status);
  });

  await test('GET /missions/:id retrieves the created mission', async () => {
    const r = await req('GET', '/missions/' + missionId);
    if (r.status !== 200) throw new Error('status=' + r.status);
    if (r.body.mission.id !== missionId) throw new Error('id mismatch');
    if (r.body.mission.metadata.user !== 'tester') throw new Error('metadata not preserved');
    if (r.body.eventCount !== 0) throw new Error('events should be 0');
  });

  await test('POST /missions/:id/events appends event', async () => {
    const r = await req('POST', `/missions/${missionId}/events`, {
      type: 'MODE_TRANSITION',
      payload: { from: 'FULL', to: 'DEGRADED', reason: 'lost satellites' }
    });
    if (r.status !== 201) throw new Error('status=' + r.status);
    if (r.body.type !== 'MODE_TRANSITION') throw new Error('type mismatch');
    if (r.body.mission_id !== missionId) throw new Error('mission_id mismatch');
    if (!r.body.timestamp) throw new Error('no timestamp');
  });

  await test('GET /missions/:id now shows the event', async () => {
    const r = await req('GET', '/missions/' + missionId);
    if (r.body.eventCount !== 1) throw new Error('expected 1 event got ' + r.body.eventCount);
    if (r.body.events[0].payload.from !== 'FULL') throw new Error('payload not preserved');
  });

  await test('POST event to non-existent mission returns 404', async () => {
    const r = await req('POST', '/missions/999999/events', { type: 'X' });
    if (r.status !== 404) throw new Error('expected 404 got ' + r.status);
  });

  await test('GET /missions lists missions', async () => {
    const r = await req('GET', '/missions');
    if (r.status !== 200) throw new Error('status=' + r.status);
    if (r.body.count < 1) throw new Error('expected at least 1');
  });

  await test('GET /missions/:id with bad id returns 404', async () => {
    const r = await req('GET', '/missions/999999');
    if (r.status !== 404) throw new Error('expected 404 got ' + r.status);
  });

  await test('Data files actually exist on disk', async () => {
    const mFile = path.join(TEST_DIR, 'missions.jsonl');
    const eFile = path.join(TEST_DIR, 'events.jsonl');
    if (!fs.existsSync(mFile)) throw new Error('missions.jsonl not created');
    if (!fs.existsSync(eFile)) throw new Error('events.jsonl not created');
    const mSize = fs.statSync(mFile).size;
    if (mSize < 50) throw new Error('missions file too small: ' + mSize);
  });

  await test('Persistence: lines in jsonl match in-memory state', async () => {
    const mFile = path.join(TEST_DIR, 'missions.jsonl');
    const lines = fs.readFileSync(mFile, 'utf8').trim().split('\n');
    if (lines.length < 1) throw new Error('no missions in file');
    const parsed = JSON.parse(lines[0]);
    if (parsed.origin_lat !== 32.0853) throw new Error('persisted data mismatch');
  });

  await test('DELETE /missions/:id removes mission', async () => {
    const r = await req('DELETE', '/missions/' + missionId);
    if (r.status !== 200 || !r.body.deleted) throw new Error('delete failed');
    const r2 = await req('GET', '/missions/' + missionId);
    if (r2.status !== 404) throw new Error('still exists after delete');
  });

  await test('DELETE writes tombstone to jsonl (audit trail)', async () => {
    const mFile = path.join(TEST_DIR, 'missions.jsonl');
    const lines = fs.readFileSync(mFile, 'utf8').trim().split('\n');
    const tombstone = lines.find(l => l.includes('"_deleted":true'));
    if (!tombstone) throw new Error('no delete tombstone in log');
  });

  await test('Invalid JSON body returns 400', async () => {
    const data = '{not valid json';
    const r = await new Promise((resolve, reject) => {
      const req = http.request({
        hostname: 'localhost', port: 3099, path: '/missions', method: 'POST',
        headers: { 'Content-Type': 'application/json', 'Content-Length': Buffer.byteLength(data) }
      }, res => {
        let buf = '';
        res.on('data', c => buf += c);
        res.on('end', () => resolve({ status: res.statusCode, body: buf ? JSON.parse(buf) : null }));
      });
      req.on('error', reject);
      req.write(data);
      req.end();
    });
    if (r.status !== 400) throw new Error('expected 400 got ' + r.status);
    if (r.body.error !== 'invalid_json') throw new Error('wrong error: ' + r.body.error);
  });

  console.log('\n═══ ' + pass + '/' + (pass + fail) + ' tests passed ═══\n');

  // Cleanup
  if (fs.existsSync(TEST_DIR)) {
    fs.readdirSync(TEST_DIR).forEach(f => fs.unlinkSync(path.join(TEST_DIR, f)));
    fs.rmdirSync(TEST_DIR);
  }

  server.close(() => process.exit(fail > 0 ? 1 : 0));
}

main().catch(e => { console.error(e); process.exit(1); });
