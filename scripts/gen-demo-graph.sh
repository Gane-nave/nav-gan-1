#!/usr/bin/env bash
# Regenerate the demo road graph served at /engine/demo-graph.json
set -euo pipefail
cd "$(dirname "$0")/.."
TMP=$(mktemp)
cat > "$TMP" <<'JSON'
{"version":0.6,"elements":[
{"type":"node","id":1,"lat":32.0800,"lon":34.7800},
{"type":"node","id":2,"lat":32.0900,"lon":34.7800},
{"type":"node","id":3,"lat":32.0850,"lon":34.7900},
{"type":"way","id":100,"nodes":[1,2],"tags":{"highway":"primary","maxspeed":"80","maxheight":"4","bridge":"yes"}},
{"type":"way","id":101,"nodes":[1,3],"tags":{"highway":"secondary","maxspeed":"50"}},
{"type":"way","id":102,"nodes":[3,2],"tags":{"highway":"secondary","maxspeed":"50"}}]}
JSON
cargo run -q -p gane-osm-import -- import demo "$TMP" app/client/public/engine/demo-graph.json
rm -f "$TMP"
