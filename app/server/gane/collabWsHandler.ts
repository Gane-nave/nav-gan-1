/**
 * G.A.N.E — Collaboration WebSocket Handler
 * ============================================
 * Bidirectional real-time communication for collaborative map editing.
 *
 * Authentication: JWT token verification on WebSocket upgrade handshake.
 * - Reads session cookie (app_session_id) from the upgrade request headers
 * - Verifies JWT using jose (same secret as tRPC context)
 * - Resolves user from database by openId
 * - Rejects unauthenticated connections with 4001 close code
 *
 * Protocol (client → server):
 *   { type: "subscribe",   sessionId: string }
 *   { type: "unsubscribe", sessionId: string }
 *   { type: "cursor",      sessionId: string, payload: { lat, lon } }
 *   { type: "heartbeat" }
 *
 * Protocol (server → client):
 *   { type: "connected",     connectionId, userId, userName, timestamp }
 *   { type: "subscribed",    sessionId }
 *   { type: "cursor_moved",  sessionId, userId, userName, userColor, payload, timestamp }
 *   { type: "marker_added",  ... }
 *   { type: "user_joined",   ... }
 *   { type: "error",         message, code? }
 *
 * Features:
 * - JWT cookie-based authentication on upgrade (no query params)
 * - Per-user and per-session connection limits
 * - Cursor updates sent via WS (no HTTP mutation needed)
 * - Server-side cursor throttling (5Hz)
 * - Ping/pong heartbeat (30s)
 * - Auto-cleanup on disconnect
 * - Prometheus metrics integration
 */
import type { Server as HttpServer, IncomingMessage } from "http";
import { WebSocketServer, WebSocket } from "ws";
import { parse as parseCookieHeader } from "cookie";
import { jwtVerify } from "jose";
import { pubsub, sessionChannel, type PubSubHandler } from "./redisPubSub";
import { notificationChannel } from "./notificationRouter";
import {
  canOpenSSEConnection,
  registerSSEConnection,
  unregisterSSEConnection,
  updateSSEActivity,
  shouldThrottleCursor,
} from "./rateLimiter";
import {
  wsConnectionsTotal,
  wsDisconnectsTotal,
  wsMessagesReceived,
  wsMessagesSent,
  collabCursorUpdatesTotal,
} from "./metricsCollector";
import type { CollabEvent } from "./collaborationRouter";
import { COOKIE_NAME } from "@shared/const";
import { ENV } from "../_core/env";
import * as db from "../db";

// ─── Types ───
interface AuthenticatedRequest extends IncomingMessage {
  __wsUser?: { userId: string; userDbId: number; userName: string };
}

interface CollabWSConnection {
  ws: WebSocket;
  connectionId: string;
  userId: string;
  userDbId: number;
  userName: string;
  subscribedSessions: Map<string, PubSubHandler>; // sessionId → handler
  notificationHandler: PubSubHandler | null; // Handler for user notification channel
  lastActivity: number;
}

// ─── Active Connections ───
const activeConnections = new Map<string, CollabWSConnection>();

// ─── JWT Verification for WebSocket Upgrade ───
async function authenticateWSUpgrade(
  req: IncomingMessage
): Promise<{ userId: string; userDbId: number; userName: string } | null> {
  try {
    // 1. Parse cookies from upgrade request headers
    const cookieHeader = req.headers.cookie;
    if (!cookieHeader) {
      console.warn("[CollabWS] No cookie header in upgrade request");
      return null;
    }

    const cookies = parseCookieHeader(cookieHeader);
    const sessionToken = cookies[COOKIE_NAME];

    if (!sessionToken) {
      console.warn("[CollabWS] No session cookie found in upgrade request");
      return null;
    }

    // 2. Verify JWT token (same secret as tRPC context)
    const secretKey = new TextEncoder().encode(ENV.cookieSecret);
    const { payload } = await jwtVerify(sessionToken, secretKey, {
      algorithms: ["HS256"],
    });

    const { openId, name } = payload as Record<string, unknown>;

    if (typeof openId !== "string" || !openId) {
      console.warn("[CollabWS] JWT payload missing openId");
      return null;
    }

    // 3. Resolve user from database
    const user = await db.getUserByOpenId(openId);
    if (!user) {
      console.warn(`[CollabWS] User not found for openId: ${openId}`);
      return null;
    }

    return {
      userId: openId,
      userDbId: user.id,
      userName: (name as string) || user.name || "User",
    };
  } catch (err) {
    console.warn("[CollabWS] JWT verification failed:", (err as Error).message);
    return null;
  }
}

// ─── WebSocket Handler Factory ───
export function createCollabWSHandler(server: HttpServer): WebSocketServer {
  const wss = new WebSocketServer({
    server,
    path: "/ws/collab",
    maxPayload: 64 * 1024, // 64KB max message size
    // Don't auto-accept — we verify JWT first in the upgrade handler
    verifyClient: async (info, callback) => {
      const user = await authenticateWSUpgrade(info.req);
      if (!user) {
        callback(false, 4001, "Authentication required");
        return;
      }
      // Attach user info to the request for use in connection handler
      (info.req as AuthenticatedRequest).__wsUser = user;
      callback(true);
    },
  });

  console.log("[CollabWS] Collaboration WebSocket handler initialized on /ws/collab (JWT auth)");

  wss.on("connection", (ws: WebSocket, req: IncomingMessage) => {
    // User was verified in verifyClient — retrieve from request
    const user = (req as AuthenticatedRequest).__wsUser;

    if (!user) {
      // Should not happen — verifyClient should have rejected
      ws.close(4001, "Authentication required");
      return;
    }

    // Generate unique connection ID
    const connectionId = `ws_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;

    // Check connection limits (reuse SSE limiter — same pool)
    const check = canOpenSSEConnection(user.userId, "__global__");
    if (!check.allowed) {
      ws.close(4029, JSON.stringify({
        type: "error",
        message: "Connection limit reached",
        reason: check.reason,
      }));
      return;
    }

    // Register connection
    registerSSEConnection(connectionId, user.userId, "__global__");
    wsConnectionsTotal.inc({ type: "collab" });

    const conn: CollabWSConnection = {
      ws,
      connectionId,
      userId: user.userId,
      userDbId: user.userDbId,
      userName: user.userName,
      subscribedSessions: new Map(),
      notificationHandler: null,
      lastActivity: Date.now(),
    };
    activeConnections.set(connectionId, conn);

    // Auto-subscribe to user's notification channel
    const notifHandler: PubSubHandler = (data: unknown) => {
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify(data));
        wsMessagesSent.inc({ type: "notification" });
      }
    };
    pubsub.subscribe(notificationChannel(user.userDbId), notifHandler);
    conn.notificationHandler = notifHandler;

    // Send connected confirmation with authenticated user info
    sendJSON(ws, {
      type: "connected",
      connectionId,
      userId: user.userId,
      userName: user.userName,
      timestamp: Date.now(),
    });

    // ─── Ping/Pong Heartbeat ───
    const pingInterval = setInterval(() => {
      if (ws.readyState === WebSocket.OPEN) {
        ws.ping();
      }
    }, 30_000);

    ws.on("pong", () => {
      conn.lastActivity = Date.now();
      updateSSEActivity(connectionId);
    });

    // ─── Message Handler ───
    ws.on("message", (raw: Buffer) => {
      conn.lastActivity = Date.now();
      updateSSEActivity(connectionId);

      try {
        const msg = JSON.parse(raw.toString());
        wsMessagesReceived.inc({ type: "collab", message_type: msg.type || "unknown" });

        switch (msg.type) {
          case "subscribe":
            handleSubscribe(conn, msg.sessionId);
            break;

          case "unsubscribe":
            handleUnsubscribe(conn, msg.sessionId);
            break;

          case "cursor":
            handleCursor(conn, msg.sessionId, msg.payload);
            break;

          case "heartbeat":
            // Activity already updated above
            sendJSON(ws, { type: "heartbeat_ack", timestamp: Date.now() });
            break;

          default:
            sendJSON(ws, { type: "error", message: `Unknown message type: ${msg.type}` });
        }
      } catch (err) {
        sendJSON(ws, { type: "error", message: "Invalid message format" });
      }
    });

    // ─── Cleanup on Close ───
    ws.on("close", (code, reason) => {
      clearInterval(pingInterval);
      wsDisconnectsTotal.inc({ type: "collab", reason: String(code) });
      cleanupConnection(conn);
    });

    ws.on("error", () => {
      clearInterval(pingInterval);
      wsDisconnectsTotal.inc({ type: "collab", reason: "error" });
      cleanupConnection(conn);
    });
  });

  return wss;
}

// ─── Subscribe to Session Events ───
function handleSubscribe(conn: CollabWSConnection, sessionId: string): void {
  if (!sessionId) {
    sendJSON(conn.ws, { type: "error", message: "sessionId required" });
    return;
  }

  // Already subscribed?
  if (conn.subscribedSessions.has(sessionId)) {
    sendJSON(conn.ws, { type: "already_subscribed", sessionId });
    return;
  }

  // Check session connection limit
  const check = canOpenSSEConnection(conn.userId, sessionId);
  if (!check.allowed) {
    sendJSON(conn.ws, {
      type: "error",
      message: check.reason || "Session connection limit reached",
      sessionId,
    });
    return;
  }

  // Create handler that forwards events to this WebSocket
  const handler: PubSubHandler = (data: unknown) => {
    if (conn.ws.readyState === WebSocket.OPEN) {
      conn.ws.send(JSON.stringify(data));
      wsMessagesSent.inc({ type: "collab" });
    }
  };

  // Subscribe via Redis Pub/Sub
  pubsub.subscribe(sessionChannel(sessionId), handler);
  conn.subscribedSessions.set(sessionId, handler);

  sendJSON(conn.ws, { type: "subscribed", sessionId, timestamp: Date.now() });
}

// ─── Unsubscribe from Session ───
function handleUnsubscribe(conn: CollabWSConnection, sessionId: string): void {
  const handler = conn.subscribedSessions.get(sessionId);
  if (handler) {
    pubsub.unsubscribe(sessionChannel(sessionId), handler);
    conn.subscribedSessions.delete(sessionId);
  }
  sendJSON(conn.ws, { type: "unsubscribed", sessionId });
}

// ─── Handle Cursor Update via WebSocket ───
function handleCursor(
  conn: CollabWSConnection,
  sessionId: string,
  payload: { lat?: number; lon?: number }
): void {
  if (!sessionId || payload?.lat == null || payload?.lon == null) {
    return; // Silently drop malformed cursor updates
  }

  // Server-side throttling (5Hz max)
  if (conn.userDbId && shouldThrottleCursor(conn.userDbId, sessionId)) {
    return; // Throttled — drop silently
  }

  collabCursorUpdatesTotal.inc();

  // Broadcast cursor update to all session subscribers
  const event: CollabEvent = {
    type: "cursor_moved",
    sessionId,
    userId: conn.userDbId || 0,
    userName: conn.userName,
    userColor: undefined,
    payload: { lat: payload.lat, lon: payload.lon },
    timestamp: Date.now(),
  };

  pubsub.publish(sessionChannel(sessionId), event);
}

// ─── Cleanup Connection ───
function cleanupConnection(conn: CollabWSConnection): void {
  // Unsubscribe from all sessions
  for (const [sessionId, handler] of Array.from(conn.subscribedSessions.entries())) {
    pubsub.unsubscribe(sessionChannel(sessionId), handler);
  }
  conn.subscribedSessions.clear();

  // Unsubscribe from notification channel
  if (conn.notificationHandler) {
    pubsub.unsubscribe(notificationChannel(conn.userDbId), conn.notificationHandler);
    conn.notificationHandler = null;
  }

  // Unregister from connection limiter
  unregisterSSEConnection(conn.connectionId);

  // Remove from active connections
  activeConnections.delete(conn.connectionId);
}

// ─── Helper: Send JSON to WebSocket ───
function sendJSON(ws: WebSocket, data: unknown): void {
  if (ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(data));
    wsMessagesSent.inc({ type: "collab" });
  }
}

// ─── Stats for Monitoring ───
export function getCollabWSStats(): {
  totalConnections: number;
  totalSubscriptions: number;
  connectionsByUser: Record<string, number>;
} {
  let totalSubscriptions = 0;
  const connectionsByUser: Record<string, number> = {};

  for (const conn of Array.from(activeConnections.values())) {
    totalSubscriptions += conn.subscribedSessions.size;
    connectionsByUser[conn.userId] = (connectionsByUser[conn.userId] || 0) + 1;
  }

  return {
    totalConnections: activeConnections.size,
    totalSubscriptions,
    connectionsByUser,
  };
}
