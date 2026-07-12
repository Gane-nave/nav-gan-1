/**
 * G.A.N.E — WebSocket Load Test
 * ================================
 * Simulates 1,000 concurrent WebSocket connections to the collaboration handler.
 * Each virtual user:
 *   1. Connects to /ws/collab
 *   2. Subscribes to a session
 *   3. Sends cursor updates at 2Hz for 3 minutes
 *   4. Gracefully disconnects
 *
 * Run:
 *   k6 run --env TARGET_HOST=localhost:3000 --env SESSION_ID=test-session load-tests/ws-connections.js
 *
 * With JSON output:
 *   k6 run --out json=load-tests/results/ws-results.json load-tests/ws-connections.js
 */
import ws from "k6/ws";
import { check, sleep } from "k6";
import { Counter, Trend, Rate } from "k6/metrics";

// ─── Custom Metrics ───
const wsConnectTime = new Trend("ws_connect_time", true);
const wsMessageLatency = new Trend("ws_message_latency", true);
const wsErrors = new Counter("ws_errors");
const wsConnectSuccess = new Rate("ws_connect_success");
const cursorUpdatesSent = new Counter("cursor_updates_sent");
const eventsReceived = new Counter("events_received");

// ─── Test Configuration ───
export const options = {
  scenarios: {
    websocket_load: {
      executor: "ramping-vus",
      startVUs: 0,
      stages: [
        { duration: "30s", target: 100 },   // Ramp to 100
        { duration: "30s", target: 500 },   // Ramp to 500
        { duration: "30s", target: 1000 },  // Ramp to 1,000
        { duration: "2m", target: 1000 },   // Sustain 1,000
        { duration: "30s", target: 0 },     // Ramp down
      ],
    },
  },
  thresholds: {
    ws_connect_time: ["p(95)<2000"],       // 95% connect under 2s
    ws_message_latency: ["p(95)<200"],     // 95% messages under 200ms
    ws_errors: ["count<50"],               // Less than 50 errors total
    ws_connect_success: ["rate>0.95"],     // 95% successful connections
  },
};

const TARGET_HOST = __ENV.TARGET_HOST || "localhost:3000";
const SESSION_ID = __ENV.SESSION_ID || "load-test-session";

export default function () {
  const userId = `load_user_${__VU}_${__ITER}`;
  const userName = `LoadUser${__VU}`;
  const url = `ws://${TARGET_HOST}/ws/collab?userId=${userId}&userName=${userName}`;

  const connectStart = Date.now();

  const res = ws.connect(url, {}, function (socket) {
    const connectTime = Date.now() - connectStart;
    wsConnectTime.add(connectTime);
    wsConnectSuccess.add(1);

    socket.on("open", () => {
      // Subscribe to session
      socket.send(
        JSON.stringify({
          type: "subscribe",
          sessionId: SESSION_ID,
        })
      );
    });

    socket.on("message", (msg) => {
      eventsReceived.add(1);

      try {
        const data = JSON.parse(msg);

        // Measure latency for timestamped events
        if (data.timestamp) {
          const latency = Date.now() - data.timestamp;
          if (latency >= 0 && latency < 60000) {
            wsMessageLatency.add(latency);
          }
        }
      } catch (e) {
        // Ignore parse errors
      }
    });

    socket.on("error", (e) => {
      wsErrors.add(1);
    });

    // Simulate cursor updates at 2Hz (every 500ms)
    const baseLat = 32.0 + Math.random() * 0.2;
    const baseLon = 34.7 + Math.random() * 0.2;

    socket.setInterval(() => {
      const lat = baseLat + (Math.random() - 0.5) * 0.01;
      const lon = baseLon + (Math.random() - 0.5) * 0.01;

      socket.send(
        JSON.stringify({
          type: "cursor",
          sessionId: SESSION_ID,
          payload: { lat, lon },
        })
      );
      cursorUpdatesSent.add(1);
    }, 500);

    // Send heartbeat every 20s
    socket.setInterval(() => {
      socket.send(JSON.stringify({ type: "heartbeat" }));
    }, 20000);

    // Stay connected for 3 minutes then close
    socket.setTimeout(() => {
      socket.close(1000);
    }, 180000);
  });

  // If connection failed
  if (!res || res.status !== 101) {
    wsConnectSuccess.add(0);
    wsErrors.add(1);
  }

  // Small pause between iterations
  sleep(1);
}

// ─── Summary Handler ───
export function handleSummary(data) {
  const summary = {
    timestamp: new Date().toISOString(),
    test: "WebSocket Connections",
    target: TARGET_HOST,
    sessionId: SESSION_ID,
    metrics: {
      connect_time_p95: data.metrics.ws_connect_time
        ? data.metrics.ws_connect_time.values["p(95)"]
        : null,
      message_latency_p95: data.metrics.ws_message_latency
        ? data.metrics.ws_message_latency.values["p(95)"]
        : null,
      total_errors: data.metrics.ws_errors
        ? data.metrics.ws_errors.values.count
        : 0,
      connect_success_rate: data.metrics.ws_connect_success
        ? data.metrics.ws_connect_success.values.rate
        : 0,
      cursor_updates_sent: data.metrics.cursor_updates_sent
        ? data.metrics.cursor_updates_sent.values.count
        : 0,
      events_received: data.metrics.events_received
        ? data.metrics.events_received.values.count
        : 0,
    },
  };

  return {
    stdout: JSON.stringify(summary, null, 2) + "\n",
  };
}
