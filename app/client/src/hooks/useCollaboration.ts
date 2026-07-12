/**
 * useCollaboration — Real-time collaboration hook
 * =================================================
 * WebSocket-first with SSE fallback.
 * Manages real-time connection, presence, markers, annotations,
 * and activity feed for collaborative map editing sessions.
 *
 * Features:
 * - WebSocket bidirectional communication (preferred)
 * - SSE fallback when WebSocket unavailable
 * - Auto-reconnection with exponential backoff (1s → 30s)
 * - Cursor updates sent via WebSocket (no HTTP mutation)
 * - Heartbeat to keep connection alive
 */
import { useState, useEffect, useCallback, useRef } from "react";
import { trpc } from "@/lib/trpc";
import { useAuth } from "@/_core/hooks/useAuth";
import { dispatchNotificationEvent } from "@/hooks/useNotifications";

export type CollabEvent = {
  type: string;
  sessionId: string;
  userId: number;
  userName?: string;
  userColor?: string;
  payload: Record<string, unknown>;
  timestamp: number;
};

export type Participant = {
  userId: number;
  displayName: string | null;
  color: string;
  cursorLat: number | null;
  cursorLon: number | null;
  isOnline: boolean;
};

export type SharedMarker = {
  markerId: string;
  lat: number;
  lon: number;
  label: string | null;
  description: string | null;
  icon: string | null;
  color: string | null;
  userId: number;
};

// ─── Reconnection Config ───
const INITIAL_RECONNECT_DELAY = 1000;
const MAX_RECONNECT_DELAY = 30000;
const HEARTBEAT_INTERVAL = 20000;

type TransportMode = "websocket" | "sse" | "disconnected";

export function useCollaboration(sessionId: string | null) {
  const [isConnected, setIsConnected] = useState(false);
  const [transportMode, setTransportMode] = useState<TransportMode>("disconnected");
  const [participants, setParticipants] = useState<Participant[]>([]);
  const [remoteCursors, setRemoteCursors] = useState<Map<number, { lat: number; lon: number; color: string }>>(new Map());
  const [events, setEvents] = useState<CollabEvent[]>([]);

  const wsRef = useRef<WebSocket | null>(null);
  const eventSourceRef = useRef<EventSource | null>(null);
  const heartbeatRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const reconnectDelayRef = useRef(INITIAL_RECONNECT_DELAY);
  const reconnectTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const intentionalCloseRef = useRef(false);

  const { user } = useAuth();
  const utils = trpc.useUtils();

  // Mutations (still needed for markers, annotations, join/leave)
  const joinMutation = trpc.collaboration.joinSession.useMutation();
  const leaveMutation = trpc.collaboration.leaveSession.useMutation();
  const cursorMutationFallback = trpc.collaboration.updateCursor.useMutation();
  const heartbeatMutation = trpc.collaboration.heartbeat.useMutation();
  const addMarkerMutation = trpc.collaboration.addMarker.useMutation();
  const moveMarkerMutation = trpc.collaboration.moveMarker.useMutation();
  const deleteMarkerMutation = trpc.collaboration.deleteMarker.useMutation();
  const addAnnotationMutation = trpc.collaboration.addAnnotation.useMutation();
  const deleteAnnotationMutation = trpc.collaboration.deleteAnnotation.useMutation();

  // Queries
  const markersQuery = trpc.collaboration.getMarkers.useQuery(
    { sessionId: sessionId! },
    { enabled: !!sessionId && isConnected }
  );
  const annotationsQuery = trpc.collaboration.getAnnotations.useQuery(
    { sessionId: sessionId! },
    { enabled: !!sessionId && isConnected }
  );
  const participantsQuery = trpc.collaboration.getParticipants.useQuery(
    { sessionId: sessionId! },
    { enabled: !!sessionId && isConnected, refetchInterval: 10000 }
  );

  // ─── Handle Incoming Event (shared by WS and SSE) ───
  const handleEvent = useCallback(
    (data: CollabEvent) => {
      if (!sessionId) return;

      // Handle cursor updates
      if (data.type === "cursor_moved") {
        setRemoteCursors((prev) => {
          const next = new Map(prev);
          next.set(data.userId, {
            lat: data.payload.lat as number,
            lon: data.payload.lon as number,
            color: data.userColor || "#00e5ff",
          });
          return next;
        });
        return;
      }

      // Handle user left — remove cursor
      if (data.type === "user_left") {
        setRemoteCursors((prev) => {
          const next = new Map(prev);
          next.delete(data.userId);
          return next;
        });
      }

      // Handle marker/annotation changes — invalidate queries
      if (["marker_added", "marker_moved", "marker_deleted"].includes(data.type)) {
        utils.collaboration.getMarkers.invalidate({ sessionId });
      }
      if (["annotation_added", "annotation_deleted"].includes(data.type)) {
        utils.collaboration.getAnnotations.invalidate({ sessionId });
      }
      if (["user_joined", "user_left"].includes(data.type)) {
        utils.collaboration.getParticipants.invalidate({ sessionId });
      }

      // Add to activity feed
      setEvents((prev) => [data, ...prev].slice(0, 100));
    },
    [sessionId, utils]
  );

  // ─── WebSocket Connection ───
  const connectWebSocket = useCallback(() => {
    if (!sessionId || !user) return false;

    try {
      const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
      // JWT auth via cookie — no query params needed (cookie sent automatically on upgrade)
      const wsUrl = `${protocol}//${window.location.host}/ws/collab`;

      const ws = new WebSocket(wsUrl);
      wsRef.current = ws;

      ws.onopen = () => {
        setIsConnected(true);
        setTransportMode("websocket");
        reconnectDelayRef.current = INITIAL_RECONNECT_DELAY;

        // Subscribe to session
        ws.send(JSON.stringify({ type: "subscribe", sessionId }));

        // Start heartbeat
        if (heartbeatRef.current) clearInterval(heartbeatRef.current);
        heartbeatRef.current = setInterval(() => {
          if (ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ type: "heartbeat" }));
          }
          // Also send tRPC heartbeat for presence tracking
          heartbeatMutation.mutate({ sessionId });
        }, HEARTBEAT_INTERVAL);
      };

      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);

          // Handle control messages
          if (data.type === "connected" || data.type === "subscribed" || data.type === "heartbeat_ack") {
            return;
          }

          // Handle errors
          if (data.type === "error") {
            console.warn("[Collab WS] Server error:", data.message);
            return;
          }

          // Forward notification events to the global notification bus
          if (data.type === "notification") {
            dispatchNotificationEvent(data);
            return;
          }

          // Handle collaboration events
          handleEvent(data as CollabEvent);
        } catch {
          // Ignore parse errors
        }
      };

      ws.onclose = (event) => {
        setIsConnected(false);
        setTransportMode("disconnected");
        wsRef.current = null;

        if (heartbeatRef.current) {
          clearInterval(heartbeatRef.current);
          heartbeatRef.current = null;
        }

        // Auto-reconnect unless intentionally closed
        if (!intentionalCloseRef.current && event.code !== 1000) {
          scheduleReconnect();
        }
      };

      ws.onerror = () => {
        // onclose will fire after onerror — reconnection handled there
      };

      return true;
    } catch {
      return false;
    }
  }, [sessionId, user, handleEvent, heartbeatMutation]);

  // ─── SSE Fallback Connection ───
  const connectSSE = useCallback(() => {
    if (!sessionId) return;

    const es = new EventSource(`/api/collab/stream?sessionId=${sessionId}`);
    eventSourceRef.current = es;

    es.onopen = () => {
      setIsConnected(true);
      setTransportMode("sse");
      reconnectDelayRef.current = INITIAL_RECONNECT_DELAY;
    };

    es.onmessage = (event) => {
      try {
        const data: CollabEvent = JSON.parse(event.data);

        if (data.type === "connected") {
          setIsConnected(true);
          return;
        }

        handleEvent(data);
      } catch {
        // Ignore parse errors (heartbeats, etc.)
      }
    };

    es.onerror = () => {
      setIsConnected(false);
      setTransportMode("disconnected");
      es.close();
      eventSourceRef.current = null;

      if (!intentionalCloseRef.current) {
        scheduleReconnect();
      }
    };

    // Heartbeat via tRPC (SSE is unidirectional)
    if (heartbeatRef.current) clearInterval(heartbeatRef.current);
    heartbeatRef.current = setInterval(() => {
      heartbeatMutation.mutate({ sessionId });
    }, HEARTBEAT_INTERVAL);
  }, [sessionId, handleEvent, heartbeatMutation]);

  // ─── Reconnection with Exponential Backoff ───
  const scheduleReconnect = useCallback(() => {
    if (intentionalCloseRef.current) return;

    if (reconnectTimerRef.current) {
      clearTimeout(reconnectTimerRef.current);
    }

    reconnectTimerRef.current = setTimeout(() => {
      reconnectTimerRef.current = null;

      // Try WebSocket first, fall back to SSE
      const wsConnected = connectWebSocket();
      if (!wsConnected) {
        connectSSE();
      }

      // Exponential backoff
      reconnectDelayRef.current = Math.min(
        reconnectDelayRef.current * 2,
        MAX_RECONNECT_DELAY
      );
    }, reconnectDelayRef.current);
  }, [connectWebSocket, connectSSE]);

  // ─── Main Connection Effect ───
  useEffect(() => {
    if (!sessionId) return;

    intentionalCloseRef.current = false;

    // Try WebSocket first
    const wsConnected = connectWebSocket();
    if (!wsConnected) {
      // Fall back to SSE
      connectSSE();
    }

    return () => {
      intentionalCloseRef.current = true;

      // Close WebSocket
      if (wsRef.current) {
        wsRef.current.close(1000, "Component unmounting");
        wsRef.current = null;
      }

      // Close SSE
      if (eventSourceRef.current) {
        eventSourceRef.current.close();
        eventSourceRef.current = null;
      }

      // Clear heartbeat
      if (heartbeatRef.current) {
        clearInterval(heartbeatRef.current);
        heartbeatRef.current = null;
      }

      // Clear reconnect timer
      if (reconnectTimerRef.current) {
        clearTimeout(reconnectTimerRef.current);
        reconnectTimerRef.current = null;
      }

      setIsConnected(false);
      setTransportMode("disconnected");
    };
  }, [sessionId, connectWebSocket, connectSSE]);

  // ─── Actions ───
  const joinSession = useCallback(
    async (sid: string) => {
      return joinMutation.mutateAsync({ sessionId: sid });
    },
    [joinMutation]
  );

  const leaveSession = useCallback(async () => {
    if (!sessionId) return;
    await leaveMutation.mutateAsync({ sessionId });
  }, [sessionId, leaveMutation]);

  // Cursor updates: prefer WebSocket, fall back to tRPC mutation
  const updateCursor = useCallback(
    (lat: number, lon: number) => {
      if (!sessionId) return;

      if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
        // Send via WebSocket — no HTTP overhead
        wsRef.current.send(
          JSON.stringify({
            type: "cursor",
            sessionId,
            payload: { lat, lon },
          })
        );
      } else {
        // Fallback to tRPC mutation
        cursorMutationFallback.mutate({ sessionId, lat, lon });
      }
    },
    [sessionId, cursorMutationFallback]
  );

  const addMarker = useCallback(
    async (lat: number, lon: number, label?: string, description?: string, color?: string) => {
      if (!sessionId) return;
      return addMarkerMutation.mutateAsync({
        sessionId,
        lat,
        lon,
        label,
        description,
        color,
      });
    },
    [sessionId, addMarkerMutation]
  );

  const moveMarker = useCallback(
    async (markerId: string, lat: number, lon: number) => {
      if (!sessionId) return;
      return moveMarkerMutation.mutateAsync({ markerId, sessionId, lat, lon });
    },
    [sessionId, moveMarkerMutation]
  );

  const deleteMarker = useCallback(
    async (markerId: string) => {
      if (!sessionId) return;
      return deleteMarkerMutation.mutateAsync({ markerId, sessionId });
    },
    [sessionId, deleteMarkerMutation]
  );

  const addAnnotation = useCallback(
    async (type: "text" | "route" | "area" | "measurement" | "arrow", data: Record<string, unknown>, color?: string) => {
      if (!sessionId) return;
      return addAnnotationMutation.mutateAsync({ sessionId, type, data, color });
    },
    [sessionId, addAnnotationMutation]
  );

  const deleteAnnotation = useCallback(
    async (annotationId: string) => {
      if (!sessionId) return;
      return deleteAnnotationMutation.mutateAsync({ annotationId, sessionId });
    },
    [sessionId, deleteAnnotationMutation]
  );

  return {
    isConnected,
    transportMode,
    participants: participantsQuery.data ?? participants,
    remoteCursors,
    markers: (markersQuery.data && 'markers' in markersQuery.data) ? markersQuery.data.markers : (markersQuery.data as unknown as Array<unknown>) ?? [],
    annotations: (annotationsQuery.data && 'annotations' in annotationsQuery.data) ? annotationsQuery.data.annotations : (annotationsQuery.data as unknown as Array<unknown>) ?? [],
    events,
    joinSession,
    leaveSession,
    updateCursor,
    addMarker,
    moveMarker,
    deleteMarker,
    addAnnotation,
    deleteAnnotation,
  };
}
