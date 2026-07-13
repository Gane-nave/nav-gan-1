#!/usr/bin/env bash
# G.A.N.E — Full stack verification (backend + E2E)
set -e
cd "$(dirname "$0")"

echo "═══ Cleaning previous state ═══"
rm -rf data test-data ../tests-e2e/e2e-data
pkill -f "node server.js" 2>/dev/null || true
sleep 1

echo ""
echo "═══ [1/3] Backend unit tests (15 tests) ═══"
node test.js

echo ""
echo "═══ [2/3] E2E mission lifecycle tests (13 tests) ═══"
cd ..
node tests-e2e/e2e-mission-lifecycle.js
cd backend

echo ""
echo "═══ [3/3] Live server + curl + disk persistence ═══"
node server.js > /tmp/gane-server.log 2>&1 &
SERVER_PID=$!
trap "kill $SERVER_PID 2>/dev/null || true" EXIT
sleep 1

if ! ps -p $SERVER_PID > /dev/null; then
  echo "✗ server failed to start"; cat /tmp/gane-server.log; exit 1
fi
echo "✓ server PID $SERVER_PID"

echo ""
echo "--- GET /health ---"
curl -sS http://localhost:3001/health
echo ""

echo ""
echo "--- POST /missions (real HTTP) ---"
curl -sS -X POST http://localhost:3001/missions \
  -H 'Content-Type: application/json' \
  -d '{"origin_lat":32.0853,"origin_lng":34.7818,"metadata":{"src":"VERIFY.sh"}}'
echo ""

echo ""
echo "--- Disk persistence ---"
ls -l data/
echo "missions.jsonl content:"
cat data/missions.jsonl

kill $SERVER_PID
wait $SERVER_PID 2>/dev/null || true

echo ""
echo "═══ VERIFIED — 28/28 tests, live server OK, disk OK ═══"
