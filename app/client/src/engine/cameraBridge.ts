/**
 * G.A.N.E — Camera Bridge
 * ========================
 * Connects browser MediaDevices API (getUserMedia) to:
 *   1. Visual Odometry engine — extracts pixel data from video frames
 *   2. AR HUD engine — provides live camera stream for AR overlay
 *
 * Uses requestAnimationFrame to capture frames at a controlled rate,
 * extracts grayscale pixel data via OffscreenCanvas, and feeds them
 * into the VO pipeline. The raw MediaStream is also exposed for
 * AR HUD canvas compositing.
 */

import type { VisualOdometryEngine } from './visualOdometry';

// ─── Types ───

export interface CameraBridgeConfig {
  /** Target frame capture rate (fps) for VO processing */
  targetFPS: number;
  /** Preferred camera resolution width */
  preferredWidth: number;
  /** Preferred camera resolution height */
  preferredHeight: number;
  /** Use rear camera if available (for mobile devices) */
  preferRearCamera: boolean;
  /** Enable frame capture for VO (can be disabled to save CPU) */
  enableVOCapture: boolean;
}

export interface CameraState {
  isActive: boolean;
  hasPermission: boolean | null; // null = not yet requested
  resolution: { width: number; height: number } | null;
  fps: number;
  facingMode: string;
  framesCapured: number;
  lastFrameTime: number;
  error: string | null;
}

const DEFAULT_CONFIG: CameraBridgeConfig = {
  targetFPS: 15,
  preferredWidth: 640,
  preferredHeight: 480,
  preferRearCamera: true,
  enableVOCapture: true,
};

// ─── Camera Bridge ───

export class CameraBridge {
  private config: CameraBridgeConfig;
  private stream: MediaStream | null = null;
  private videoElement: HTMLVideoElement | null = null;
  private captureCanvas: OffscreenCanvas | null = null;
  private captureCtx: OffscreenCanvasRenderingContext2D | null = null;
  private rafId: number | null = null;
  private lastCaptureTime = 0;
  private frameInterval: number;
  private voEngine: VisualOdometryEngine | null = null;
  private arCanvas: HTMLCanvasElement | null = null;
  private arCtx: CanvasRenderingContext2D | null = null;
  private destroyed = false;

  state: CameraState = {
    isActive: false,
    hasPermission: null,
    resolution: null,
    fps: 0,
    facingMode: 'environment',
    framesCapured: 0,
    lastFrameTime: 0,
    error: null,
  };

  constructor(config: Partial<CameraBridgeConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.frameInterval = 1000 / this.config.targetFPS;
  }

  // ─── Permission Check ───

  async checkPermission(): Promise<boolean> {
    try {
      if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
        this.state.hasPermission = false;
        this.state.error = 'Camera API not available (requires HTTPS)';
        return false;
      }
      // Query permission state without prompting
      if (navigator.permissions) {
        const result = await navigator.permissions.query({ name: 'camera' as PermissionName });
        if (result.state === 'granted') {
          this.state.hasPermission = true;
          return true;
        }
        if (result.state === 'denied') {
          this.state.hasPermission = false;
          this.state.error = 'Camera permission denied by user';
          return false;
        }
      }
      // Permission state is 'prompt' — we haven't asked yet
      this.state.hasPermission = null;
      return true; // Can still request
    } catch {
      this.state.hasPermission = null;
      return true; // Assume we can try
    }
  }

  // ─── Start Camera ───

  async start(): Promise<boolean> {
    if (this.destroyed) return false;
    if (this.state.isActive) return true;

    try {
      // Request camera access
      const constraints: MediaStreamConstraints = {
        video: {
          width: { ideal: this.config.preferredWidth },
          height: { ideal: this.config.preferredHeight },
          facingMode: this.config.preferRearCamera ? 'environment' : 'user',
          frameRate: { ideal: this.config.targetFPS, max: 30 },
        },
        audio: false,
      };

      this.stream = await navigator.mediaDevices.getUserMedia(constraints);
      this.state.hasPermission = true;

      // Get actual resolution from track settings
      const videoTrack = this.stream.getVideoTracks()[0];
      const settings = videoTrack.getSettings();
      this.state.resolution = {
        width: settings.width || this.config.preferredWidth,
        height: settings.height || this.config.preferredHeight,
      };
      this.state.facingMode = settings.facingMode || 'environment';

      // Create hidden video element for frame extraction
      this.videoElement = document.createElement('video');
      this.videoElement.srcObject = this.stream;
      this.videoElement.setAttribute('playsinline', 'true');
      this.videoElement.setAttribute('autoplay', 'true');
      this.videoElement.muted = true;
      await this.videoElement.play();

      // Create offscreen canvas for pixel extraction
      const w = this.state.resolution.width;
      const h = this.state.resolution.height;
      this.captureCanvas = new OffscreenCanvas(w, h);
      this.captureCtx = this.captureCanvas.getContext('2d') as OffscreenCanvasRenderingContext2D;

      this.state.isActive = true;
      this.state.error = null;

      // Start frame capture loop
      this.startCaptureLoop();

      return true;
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : 'Unknown camera error';
      this.state.error = msg;
      this.state.hasPermission = msg.includes('Permission') || msg.includes('NotAllowed') ? false : this.state.hasPermission;
      this.state.isActive = false;
      return false;
    }
  }

  // ─── Stop Camera ───

  stop(): void {
    if (this.rafId !== null) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }

    if (this.stream) {
      this.stream.getTracks().forEach(track => track.stop());
      this.stream = null;
    }

    if (this.videoElement) {
      this.videoElement.srcObject = null;
      this.videoElement = null;
    }

    this.captureCanvas = null;
    this.captureCtx = null;
    this.state.isActive = false;
  }

  // ─── Destroy (permanent cleanup) ───

  destroy(): void {
    this.destroyed = true;
    this.stop();
    this.voEngine = null;
    this.arCanvas = null;
    this.arCtx = null;
  }

  // ─── Connect Engines ───

  /** Connect Visual Odometry engine to receive frame data */
  connectVO(engine: VisualOdometryEngine): void {
    this.voEngine = engine;
  }

  /** Connect AR HUD canvas for camera background rendering */
  connectARCanvas(canvas: HTMLCanvasElement): void {
    this.arCanvas = canvas;
    this.arCtx = canvas.getContext('2d');
  }

  /** Get the raw MediaStream for direct video element binding */
  getStream(): MediaStream | null {
    return this.stream;
  }

  // ─── Frame Capture Loop ───

  private startCaptureLoop(): void {
    const loop = (now: number) => {
      if (this.destroyed || !this.state.isActive) return;

      const elapsed = now - this.lastCaptureTime;
      if (elapsed >= this.frameInterval) {
        this.lastCaptureTime = now;
        this.captureFrame(now);
      }

      this.rafId = requestAnimationFrame(loop);
    };
    this.rafId = requestAnimationFrame(loop);
  }

  private captureFrame(timestamp: number): void {
    if (!this.videoElement || !this.captureCtx || !this.captureCanvas) return;
    if (this.videoElement.readyState < 2) return; // HAVE_CURRENT_DATA

    const w = this.captureCanvas.width;
    const h = this.captureCanvas.height;

    // Draw current video frame to offscreen canvas
    this.captureCtx.drawImage(this.videoElement, 0, 0, w, h);

    // Feed VO engine with grayscale pixel data
    if (this.config.enableVOCapture && this.voEngine) {
      const imageData = this.captureCtx.getImageData(0, 0, w, h);
      const grayscale = this.rgbaToGrayscale(imageData.data, w, h);
      this.voEngine.processFrame(grayscale, timestamp);
    }

    // Draw camera feed onto AR HUD canvas (background layer)
    if (this.arCanvas && this.arCtx && this.videoElement) {
      this.arCtx.drawImage(
        this.videoElement,
        0, 0,
        this.arCanvas.width,
        this.arCanvas.height
      );
    }

    this.state.framesCapured++;
    this.state.lastFrameTime = timestamp;

    // Calculate actual FPS
    if (this.state.framesCapured % 30 === 0) {
      const dt = timestamp - (this.state.lastFrameTime || timestamp);
      this.state.fps = dt > 0 ? Math.round(1000 / (dt / 30)) : this.config.targetFPS;
    }
  }

  /** Convert RGBA pixel data to grayscale Uint8Array */
  private rgbaToGrayscale(rgba: Uint8ClampedArray, width: number, height: number): Uint8Array {
    const gray = new Uint8Array(width * height);
    for (let i = 0; i < width * height; i++) {
      const ri = i * 4;
      // ITU-R BT.601 luma coefficients
      gray[i] = Math.round(0.299 * rgba[ri] + 0.587 * rgba[ri + 1] + 0.114 * rgba[ri + 2]);
    }
    return gray;
  }
}
