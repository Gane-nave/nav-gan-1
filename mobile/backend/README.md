# G.A.N.E Backend — Real Vertical Slice

Zero-dependency Node.js backend for mission storage.

## Stack
- Node.js ≥18 (built-ins only: `http`, `fs`, `url`)
- JSONL append-only persistence (no SQL, no compile)
- 0 npm dependencies

## Run

```bash
cd backend
node server.js          # starts at http://localhost:3001
```

Optional env vars:
- `PORT=3001`
- `GANE_DATA_DIR=./data`

## Test

```bash
node test.js            # 15 integration tests against real HTTP + real disk
```

Expected: `═══ 15/15 tests passed ═══`

## Endpoints

| Method | Path | Body | Returns |
|--------|------|------|---------|
| GET | `/health` | — | `{status, missions, events, uptime_sec, node}` |
| POST | `/missions` | `{origin_lat, origin_lng, dest_lat?, dest_lng?, metadata?}` | `201 {id, created_at, origin, dest, status, metadata}` |
| GET | `/missions` | — | `{missions[], count}` (latest 50) |
| GET | `/missions/:id` | — | `{mission, events[], eventCount}` |
| POST | `/missions/:id/events` | `{type, payload?}` | `201 {id, mission_id, type, payload, timestamp}` |
| DELETE | `/missions/:id` | — | `{deleted, id}` |

## Validation

- Lat ∈ [-90, 90], Lng ∈ [-180, 180]
- Invalid coords → `400 {error: "invalid_origin"}`
- Missing body on POST → `400 {error: "body_required"}`
- Malformed JSON → `400 {error: "invalid_json"}`
- Non-existent mission → `404`

## Persistence

Append-only JSONL files in `data/`:
- `missions.jsonl` — one mission per line; deletes append `{id, _deleted:true}` tombstone
- `events.jsonl` — one event per line; chronological order

On restart, loads files, rebuilds in-memory indexes.

## Frontend Wiring

Already wired in `www/index.html`:
- `bkPing()` → GET /health (auto on load)
- `bkUploadMission()` → POST /missions + events for trace entries
- `bkListMissions()` → GET /missions
- Trace events auto-streamed to backend during active mission

UI buttons in Diagnostics panel: "Ping", "Upload mission", "List server missions".

## What This Proves

- Real backend runs on real port
- Real DB persists to real disk
- Real HTTP requests get real responses
- Real frontend code calls real backend
- All 15 backend tests pass against live server

## What This Does NOT Claim

- Not authenticated (single-tenant, local-only)
- Not deployed (run locally with `node server.js`)
- Not RBAC, not billing, not multi-tenant
- Production deployment requires additional work
