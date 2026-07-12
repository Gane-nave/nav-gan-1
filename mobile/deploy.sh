#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# G.A.N.E — ONE-SHOT DEPLOYMENT
# Run from folder containing all G.A.N.E files. Single command:
#   chmod +x deploy.sh && ./deploy.sh
# ═══════════════════════════════════════════════════════════════════════
set -e
PORT=${PORT:-8000}
REQUIRED=("gane-v6-integrated.html" "gane-core-bundle.js" "gane-sw.js" "manifest.json" "test-report.html")

cyan(){ printf '\033[1;36m%s\033[0m\n' "$1"; }
green(){ printf '\033[1;32m%s\033[0m\n' "$1"; }
red(){ printf '\033[1;31m%s\033[0m\n' "$1"; }
yellow(){ printf '\033[1;33m%s\033[0m\n' "$1"; }

cyan "╔══════════════════════════════════════════════╗"
cyan "║  G.A.N.E — ONE-SHOT DEPLOYMENT               ║"
cyan "╚══════════════════════════════════════════════╝"
echo ""

# 1. Verify files
cyan "[1/5] Verifying files..."
MISSING=0
for f in "${REQUIRED[@]}"; do
  if [[ ! -f "$f" ]]; then
    red "  ✗ Missing: $f"
    MISSING=$((MISSING+1))
  else
    green "  ✓ $f ($(du -h "$f" | cut -f1))"
  fi
done
if [[ $MISSING -gt 0 ]]; then
  red "FATAL: $MISSING required files missing. Place all G.A.N.E files in same folder as this script."
  exit 1
fi

# 2. Validate bundle syntax
cyan "[2/5] Validating bundle syntax..."
if command -v node &>/dev/null; then
  if node -e "new Function(require('fs').readFileSync('gane-core-bundle.js','utf8'));" 2>/dev/null; then
    green "  ✓ Bundle syntax valid"
  else
    red "  ✗ Bundle has syntax errors"
    exit 1
  fi
else
  yellow "  ⚠ Node not installed — skipping syntax check"
fi

# 3. Pick a server
cyan "[3/5] Selecting HTTP server..."
SERVER_CMD=""
if command -v python3 &>/dev/null; then
  SERVER_CMD="python3 -m http.server $PORT"
  green "  ✓ Using python3 http.server"
elif command -v python &>/dev/null; then
  SERVER_CMD="python -m SimpleHTTPServer $PORT"
  green "  ✓ Using python SimpleHTTPServer"
elif command -v php &>/dev/null; then
  SERVER_CMD="php -S localhost:$PORT"
  green "  ✓ Using php built-in server"
elif command -v npx &>/dev/null; then
  SERVER_CMD="npx serve -l $PORT ."
  green "  ✓ Using npx serve"
else
  red "  ✗ No HTTP server available. Install python3 or php."
  exit 1
fi

# 4. Port check
cyan "[4/5] Checking port $PORT..."
if command -v lsof &>/dev/null && lsof -i ":$PORT" &>/dev/null; then
  yellow "  ⚠ Port $PORT in use. Trying port 8001..."
  PORT=8001
  SERVER_CMD="${SERVER_CMD/$PORT/8001}"
fi
green "  ✓ Port $PORT available"

# 5. Launch
cyan "[5/5] Launching server..."
echo ""
green "═══════════════════════════════════════════════"
green "  ✓ Running at: http://localhost:$PORT/"
green ""
green "  URLs to open:"
green "    → http://localhost:$PORT/gane-v6-integrated.html  (MAIN APP)"
green "    → http://localhost:$PORT/test-report.html         (FULL TEST REPORT)"
green "═══════════════════════════════════════════════"
echo ""

# Auto-open browser
URL="http://localhost:$PORT/test-report.html"
if command -v open &>/dev/null; then
  (sleep 1 && open "$URL") &
elif command -v xdg-open &>/dev/null; then
  (sleep 1 && xdg-open "$URL") &
elif command -v start &>/dev/null; then
  (sleep 1 && start "$URL") &
fi

yellow "Press Ctrl+C to stop server"
echo ""
exec $SERVER_CMD
