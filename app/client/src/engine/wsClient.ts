/**
 * G.A.N.E — WebSocket Client
 * ============================
 * Frontend WebSocket client for real-time communication with the G.A.N.E bridge.
 * Supports binary telemetry, JSON messages, and automatic reconnection.
 */

export type WSMessageHandler = (msg: { type: string; payload: Record<string, unknown> }) => void;

interface WSClientConfig {
  reconnectInterval: number;
  maxReconnectAttempts: number;
  heartbeatInterval: number;
}

const DEFAULT_CONFIG: WSClientConfig = {
  reconnectInterval: 3000,
  maxReconnectAttempts: 10,
  heartbeatInterval: 30000,
};

class GANEWSClient {
  private ws: WebSocket | null = null;
  private config: WSClientConfig;
  private reconnectAttempts = 0;
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private heartbeatTimer: ReturnType<typeof setInterval> | null = null;
  private handlers = new Map<string, Set<WSMessageHandler>>();
  private globalHandlers = new Set<WSMessageHandler>();
  private _clientId: string | null = null;
  private _connected = false;
  private _deviceId: string | null = null;

  constructor(config: Partial<WSClientConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  get connected(): boolean { return this._connected; }
  get clientId(): string | null { return this._clientId; }

  /** Connect to the G.A.N.E WebSocket bridge */
  connect(deviceId?: string): void {
    if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
      return;
    }

    this._deviceId = deviceId ?? `web_${Date.now().toString(36)}`;

    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const url = `${protocol}//${window.location.host}/ws/gane`;

    try {
      this.ws = new WebSocket(url);
      this.ws.binaryType = 'arraybuffer';

      this.ws.onopen = () => {
        console.log('[WS Client] Connected to G.A.N.E bridge');
        this._connected = true;
        this.reconnectAttempts = 0;

        // Register device
        this.send('register', { deviceId: this._deviceId, role: 'device' });

        // Start heartbeat
        this.startHeartbeat();

        // Notify handlers
        this.emit('connected', {});
      };

      this.ws.onmessage = (event) => {
        try {
          if (event.data instanceof ArrayBuffer) {
            // Binary telemetry from another device
            this.emit('binary_telemetry', { buffer: event.data });
          } else {
            const msg = JSON.parse(event.data);
            if (msg.type === 'welcome') {
              this._clientId = msg.payload.clientId;
            }
            this.emit(msg.type, msg.payload);
          }
        } catch (err) {
          console.error('[WS Client] Message parse error:', err);
        }
      };

      this.ws.onclose = () => {
        console.log('[WS Client] Disconnected');
        this._connected = false;
        this.stopHeartbeat();
        this.emit('disconnected', {});
        this.scheduleReconnect();
      };

      this.ws.onerror = (err) => {
        console.error('[WS Client] Error:', err);
        this._connected = false;
      };
    } catch (err) {
      console.error('[WS Client] Connection error:', err);
      this.scheduleReconnect();
    }
  }

  /** Disconnect */
  disconnect(): void {
    this.reconnectAttempts = this.config.maxReconnectAttempts; // prevent reconnect
    if (this.reconnectTimer) clearTimeout(this.reconnectTimer);
    this.stopHeartbeat();
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this._connected = false;
  }

  /** Send JSON message */
  send(type: string, payload: Record<string, unknown> = {}): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) return;
    this.ws.send(JSON.stringify({ type, payload }));
  }

  /** Send binary telemetry (32 bytes) */
  sendBinaryTelemetry(data: {
    lat: number; lon: number; alt: number; velocity: number;
    heading: number; confidence: number; satellites: number; flags: number;
  }): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) return;

    const buffer = new ArrayBuffer(32);
    const view = new DataView(buffer);
    view.setFloat32(0, data.lat, true);
    view.setFloat32(4, data.lon, true);
    view.setFloat32(8, data.alt, true);
    view.setFloat32(12, data.velocity, true);
    view.setFloat32(16, data.heading, true);
    view.setFloat32(20, data.confidence, true);
    view.setUint32(24, data.satellites, true);
    view.setUint32(28, data.flags, true);
    this.ws.send(buffer);
  }

  /** Subscribe to a specific message type */
  on(type: string, handler: WSMessageHandler): () => void {
    if (!this.handlers.has(type)) {
      this.handlers.set(type, new Set());
    }
    this.handlers.get(type)!.add(handler);
    return () => this.handlers.get(type)?.delete(handler);
  }

  /** Subscribe to all messages */
  onAny(handler: WSMessageHandler): () => void {
    this.globalHandlers.add(handler);
    return () => this.globalHandlers.delete(handler);
  }

  /** Report an anomaly via WebSocket */
  reportAnomaly(type: string, lat: number, lon: number, severity: number = 1, description?: string): void {
    this.send('anomaly_report', { type, lat, lon, severity, description });
  }

  /** Subscribe to a channel */
  subscribe(channel: string): void {
    this.send('subscribe', { channel });
  }

  /** Unsubscribe from a channel */
  unsubscribe(channel: string): void {
    this.send('unsubscribe', { channel });
  }

  // ─── Private ───

  private emit(type: string, payload: Record<string, unknown>): void {
    const msg = { type, payload };

    // Type-specific handlers
    const handlers = this.handlers.get(type);
    if (handlers) {
      handlers.forEach(h => { try { h(msg); } catch (e) { /* ignore */ } });
    }

    // Global handlers
    this.globalHandlers.forEach(h => { try { h(msg); } catch (e) { /* ignore */ } });
  }

  private scheduleReconnect(): void {
    if (this.reconnectAttempts >= this.config.maxReconnectAttempts) {
      console.log('[WS Client] Max reconnect attempts reached');
      return;
    }

    const delay = this.config.reconnectInterval * Math.pow(1.5, this.reconnectAttempts);
    this.reconnectAttempts++;

    console.log(`[WS Client] Reconnecting in ${Math.round(delay)}ms (attempt ${this.reconnectAttempts})`);
    this.reconnectTimer = setTimeout(() => {
      this.connect(this._deviceId ?? undefined);
    }, delay);
  }

  private startHeartbeat(): void {
    this.heartbeatTimer = setInterval(() => {
      this.send('ping', {});
    }, this.config.heartbeatInterval);
  }

  private stopHeartbeat(): void {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer);
      this.heartbeatTimer = null;
    }
  }
}

// Singleton
export const wsClient = new GANEWSClient();
