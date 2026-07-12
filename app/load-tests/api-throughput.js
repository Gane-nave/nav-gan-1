/**
 * G.A.N.E — API Throughput Load Test
 * =====================================
 * Tests tRPC API endpoints under load to validate:
 * - Rate limiting enforcement
 * - Pagination performance
 * - Response times under concurrent load
 *
 * Run:
 *   k6 run --env TARGET_HOST=localhost:3000 load-tests/api-throughput.js
 */
import http from "k6/http";
import { check, sleep } from "k6";
import { Counter, Trend, Rate } from "k6/metrics";

// ─── Custom Metrics ───
const apiLatency = new Trend("api_latency", true);
const rateLimited = new Counter("rate_limited_responses");
const apiErrors = new Counter("api_errors");
const apiSuccess = new Rate("api_success_rate");

// ─── Test Configuration ───
export const options = {
  scenarios: {
    api_load: {
      executor: "ramping-vus",
      startVUs: 0,
      stages: [
        { duration: "15s", target: 50 },   // Ramp to 50
        { duration: "1m", target: 50 },    // Sustain 50
        { duration: "15s", target: 100 },  // Ramp to 100
        { duration: "1m", target: 100 },   // Sustain 100
        { duration: "15s", target: 0 },    // Ramp down
      ],
    },
  },
  thresholds: {
    api_latency: ["p(95)<1000"],         // 95% under 1s
    api_success_rate: ["rate>0.90"],      // 90% success (some 429s expected)
    api_errors: ["count<20"],            // Less than 20 server errors
  },
};

const TARGET_HOST = __ENV.TARGET_HOST || "localhost:3000";
const BASE_URL = `http://${TARGET_HOST}`;

// tRPC batch endpoint
function trpcCall(procedure, input) {
  const url = `${BASE_URL}/api/trpc/${procedure}`;
  const payload = JSON.stringify({ json: input || {} });
  const params = {
    headers: {
      "Content-Type": "application/json",
    },
    timeout: "10s",
  };

  const start = Date.now();
  const res = http.post(url, payload, params);
  const latency = Date.now() - start;

  apiLatency.add(latency);

  if (res.status === 429) {
    rateLimited.add(1);
    apiSuccess.add(0);
  } else if (res.status >= 200 && res.status < 300) {
    apiSuccess.add(1);
  } else {
    apiErrors.add(1);
    apiSuccess.add(0);
  }

  return res;
}

// ─── Test Scenarios ───
const scenarios = [
  // List sessions (paginated)
  () => {
    const res = trpcCall("collaboration.listSessions", { limit: 20 });
    check(res, {
      "listSessions status ok": (r) => r.status === 200 || r.status === 429,
    });
  },

  // Get system stats
  () => {
    const res = trpcCall("collaboration.getSystemStats", {});
    check(res, {
      "getSystemStats status ok": (r) => r.status === 200 || r.status === 429,
    });
  },
];

export default function () {
  // Pick a random scenario
  const scenario = scenarios[Math.floor(Math.random() * scenarios.length)];
  scenario();
  sleep(0.1 + Math.random() * 0.2); // 100-300ms between requests
}

// ─── Summary Handler ───
export function handleSummary(data) {
  const summary = {
    timestamp: new Date().toISOString(),
    test: "API Throughput",
    target: TARGET_HOST,
    metrics: {
      latency_p50: data.metrics.api_latency
        ? data.metrics.api_latency.values["p(50)"]
        : null,
      latency_p95: data.metrics.api_latency
        ? data.metrics.api_latency.values["p(95)"]
        : null,
      latency_p99: data.metrics.api_latency
        ? data.metrics.api_latency.values["p(99)"]
        : null,
      success_rate: data.metrics.api_success_rate
        ? data.metrics.api_success_rate.values.rate
        : 0,
      rate_limited: data.metrics.rate_limited_responses
        ? data.metrics.rate_limited_responses.values.count
        : 0,
      total_errors: data.metrics.api_errors
        ? data.metrics.api_errors.values.count
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
