/**
 * G.A.N.E Backend — Zero-dependency HTTP server
 * Pure Node built-ins. Routes registered in routes/missions.js.
 */
const http = require('http');
const url = require('url');
const db = require('./db');
const missionsHandler = require('./routes/missions');

const PORT = process.env.PORT || 3001;

function sendJSON(res, status, body) {
  const data = JSON.stringify(body);
  res.writeHead(status, {
    'Content-Type': 'application/json',
    'Content-Length': Buffer.byteLength(data),
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'GET,POST,PUT,DELETE,OPTIONS',
    'Access-Control-Allow-Headers': 'Content-Type, Authorization'
  });
  res.end(data);
}

function readBody(req) {
  return new Promise((resolve, reject) => {
    let data = '';
    req.on('data', chunk => {
      data += chunk;
      if (data.length > 10 * 1024 * 1024) { reject(new Error('payload_too_large')); req.destroy(); }
    });
    req.on('end', () => {
      if (!data) return resolve(null);
      try { resolve(JSON.parse(data)); }
      catch (e) { reject(new Error('invalid_json')); }
    });
    req.on('error', reject);
  });
}

const server = http.createServer(async (req, res) => {
  // CORS preflight
  if (req.method === 'OPTIONS') {
    res.writeHead(204, {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET,POST,PUT,DELETE,OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type, Authorization'
    });
    return res.end();
  }

  const parsed = url.parse(req.url, true);
  const pathname = parsed.pathname;

  try {
    // Health check
    if (req.method === 'GET' && pathname === '/health') {
      const stats = db.stats();
      return sendJSON(res, 200, {
        status: 'ok',
        ...stats,
        uptime_sec: Math.round(process.uptime()),
        node: process.version
      });
    }

    // Parse body for POST/PUT/DELETE
    let body = null;
    if (['POST', 'PUT', 'DELETE'].includes(req.method)) {
      try { body = await readBody(req); }
      catch (e) { return sendJSON(res, 400, { error: e.message }); }
    }

    // Route to missions handler
    if (pathname === '/missions' || pathname.startsWith('/missions/')) {
      const result = missionsHandler({ method: req.method, pathname, query: parsed.query, body });
      return sendJSON(res, result.status, result.body);
    }

    // 404
    sendJSON(res, 404, { error: 'not_found', path: pathname });
  } catch (err) {
    console.error('[error]', err);
    sendJSON(res, 500, { error: 'internal', detail: err.message });
  }
});

server.listen(PORT, () => {
  console.log(`[gane-backend] listening on http://localhost:${PORT}`);
  console.log(`[gane-backend] data: ${db.DATA_DIR}`);
  console.log(`[gane-backend] try: curl http://localhost:${PORT}/health`);
});

process.on('SIGTERM', () => { console.log('[gane-backend] SIGTERM'); server.close(()=>process.exit(0)); });
process.on('SIGINT', () => { console.log('[gane-backend] SIGINT'); server.close(()=>process.exit(0)); });

module.exports = server;
