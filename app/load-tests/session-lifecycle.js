/**
 * G.A.N.E — Session Lifecycle Load Test
 * ========================================
 * Tests the full collaboration session lifecycle under load:
 *   1. Create session
 *   2. Join session
 *   3. Add markers
 *   4. Update cursor positions
 *   5. Send heartbeats
 *   6. Leave session
 *
 * Run:
 *   k6 run --env TARGET_HOST=localhost:3000 load-tests/session-lifecycle.js
 */
import http from "k6/http";
import { check, sleep } from "k6";
import { Counter, Trend, Rate } from "k6/metrics";

// ─── Custom Metrics ───
const lifecycleLatency = new Trend("lifecycle_step_latency", true);
const lifecycleErrors = new Counter("lifecycle_errors");
const lifecycleSuccess = new Rate("lifecycle_success_rate");
const sessionsCreated = new Counter("sessions_created");
const markersAdded = new Counter("markers_added");

// ─── Test Configuration ───
export const options = {
  scenarios: {
    lifecycle: {
      executor: "ramping-vus",
      startVUs: 0,
      stages: [
        { duration: "15s", target: 10 },   // Ramp to 10
        { duration: "1m", target: 25 },    // Ramp to 25
        { duration: "2m", target: 25 },    // Sustain 25
        { duration: "15s", target: 0 },    // Ramp down
      ],
    },
  },
  thresholds: {
    lifecycle_step_latency: ["p(95)<2000"],  // 95% steps under 2s
    lifecycle_success_rate: ["rate>0.80"],    // 80% success (rate limits expected)
    lifecycle_errors: ["count<50"],           // Less than 50 errors
  },
};

const TARGET_HOST = __ENV.TARGET_HOST || "localhost:3000";
const BASE_URL = `http://${TARGET_HOST}`;

function trpcMutation(procedure, input) {
  const url = `${BASE_URL}/api/trpc/${procedure}`;
  const payload = JSON.stringify({ json: input });
  const params = {
    headers: { "Content-Type": "application/json" },
    timeout: "15s",
  };

  const start = Date.now();
  const res = http.post(url, payload, params);
  lifecycleLatency.add(Date.now() - start);

  if (res.status >= 200 && res.status < 300) {
    lifecycleSuccess.add(1);
  } else if (res.status === 429) {
    lifecycleSuccess.add(0); // Rate limited — expected under load
  } else {
    lifecycleErrors.add(1);
    lifecycleSuccess.add(0);
  }

  return res;
}

export default function () {
  const vuId = `vu_${__VU}_${__ITER}`;

  // Step 1: Create session
  const createRes = trpcMutation("collaboration.createSession", {
    name: `Load Test Session ${vuId}`,
    centerLat: 32.0853 + Math.random() * 0.1,
    centerLon: 34.7818 + Math.random() * 0.1,
    zoomLevel: 12,
  });

  let sessionId = null;
  if (createRes.status === 200) {
    try {
      const body = JSON.parse(createRes.body);
      sessionId = body.result?.data?.json?.sessionId;
      if (sessionId) sessionsCreated.add(1);
    } catch {}
  }

  if (!sessionId) {
    sleep(2);
    return; // Rate limited or error — skip this iteration
  }

  check(createRes, {
    "session created": (r) => r.status === 200,
  });

  sleep(0.5);

  // Step 2: Join session
  const joinRes = trpcMutation("collaboration.joinSession", {
    sessionId: sessionId,
  });
  check(joinRes, {
    "joined session": (r) => r.status === 200 || r.status === 429,
  });

  sleep(0.3);

  // Step 3: Add markers
  for (let i = 0; i < 3; i++) {
    const markerRes = trpcMutation("collaboration.addMarker", {
      sessionId: sessionId,
      lat: 32.0 + Math.random() * 0.2,
      lon: 34.7 + Math.random() * 0.2,
      label: `Marker ${i + 1}`,
      description: `Load test marker ${vuId}`,
      color: "#FF5733",
    });

    if (markerRes.status === 200) {
      markersAdded.add(1);
    }

    sleep(0.2);
  }

  // Step 4: Cursor updates
  for (let i = 0; i < 5; i++) {
    trpcMutation("collaboration.updateCursor", {
      sessionId: sessionId,
      lat: 32.0 + Math.random() * 0.2,
      lon: 34.7 + Math.random() * 0.2,
    });
    sleep(0.2);
  }

  // Step 5: Heartbeat
  trpcMutation("collaboration.heartbeat", {
    sessionId: sessionId,
  });

  sleep(0.5);

  // Step 6: Leave session
  const leaveRes = trpcMutation("collaboration.leaveSession", {
    sessionId: sessionId,
  });
  check(leaveRes, {
    "left session": (r) => r.status === 200 || r.status === 429,
  });

  sleep(1);
}

// ─── Summary Handler ───
export function handleSummary(data) {
  const summary = {
    timestamp: new Date().toISOString(),
    test: "Session Lifecycle",
    target: TARGET_HOST,
    metrics: {
      step_latency_p50: data.metrics.lifecycle_step_latency
        ? data.metrics.lifecycle_step_latency.values["p(50)"]
        : null,
      step_latency_p95: data.metrics.lifecycle_step_latency
        ? data.metrics.lifecycle_step_latency.values["p(95)"]
        : null,
      success_rate: data.metrics.lifecycle_success_rate
        ? data.metrics.lifecycle_success_rate.values.rate
        : 0,
      sessions_created: data.metrics.sessions_created
        ? data.metrics.sessions_created.values.count
        : 0,
      markers_added: data.metrics.markers_added
        ? data.metrics.markers_added.values.count
        : 0,
      total_errors: data.metrics.lifecycle_errors
        ? data.metrics.lifecycle_errors.values.count
        : 0,
      total_requests: data.metrics.http_reqs
        ? data.metrics.http_reqs.values.count
        : 0,
    },
  };

  return {
    stdout: JSON.stringify(summary, null, 2) + "\n",
  };
}
