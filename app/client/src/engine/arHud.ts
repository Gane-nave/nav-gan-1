/**
 * G.A.N.E — AR HUD Engine
 * =========================
 * Augmented Reality Head-Up Display for route visualization.
 * Draws navigation cues on a canvas overlay:
 * - Route path projection
 * - Turn arrows
 * - Distance markers
 * - Speed/heading indicators
 * - Hazard warnings
 */

export interface ARHudConfig {
  width: number;
  height: number;
  fov: number;           // Field of view in degrees
  vanishingPointY: number; // 0-1, where horizon line sits
  primaryColor: string;
  accentColor: string;
  dangerColor: string;
  opacity: number;
}

export interface ARWaypoint {
  lat: number;
  lon: number;
  distance: number;     // meters from current position
  bearing: number;      // degrees from north
  instruction?: string;
  turnType?: 'left' | 'right' | 'straight' | 'u-turn' | 'arrive';
}

export interface ARHazard {
  lat: number;
  lon: number;
  distance: number;
  bearing: number;
  type: 'pothole' | 'accident' | 'construction' | 'speed_camera' | 'police' | 'weather';
  severity: number; // 1-5
}

const DEFAULT_CONFIG: ARHudConfig = {
  width: 800,
  height: 400,
  fov: 60,
  vanishingPointY: 0.35,
  primaryColor: '#00e5ff',
  accentColor: '#00ff88',
  dangerColor: '#ff3355',
  opacity: 0.85,
};

export class ARHudRenderer {
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private config: ARHudConfig;
  private currentHeading = 0;
  private currentSpeed = 0;
  private animFrame = 0;

  constructor(config: Partial<ARHudConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /** Attach to a canvas element */
  attach(canvas: HTMLCanvasElement): void {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    if (this.ctx) {
      const dpr = Math.min(window.devicePixelRatio, 2);
      canvas.width = this.config.width * dpr;
      canvas.height = this.config.height * dpr;
      canvas.style.width = this.config.width + 'px';
      canvas.style.height = this.config.height + 'px';
      this.ctx.scale(dpr, dpr);
    }
  }

  /** Detach from canvas */
  detach(): void {
    this.canvas = null;
    this.ctx = null;
  }

  /** Update heading and speed */
  updateVehicleState(heading: number, speed: number): void {
    this.currentHeading = heading;
    this.currentSpeed = speed;
  }

  /** Render a full AR HUD frame */
  render(waypoints: ARWaypoint[], hazards: ARHazard[] = []): void {
    if (!this.ctx) return;
    const { width, height } = this.config;
    this.animFrame++;

    // Clear
    this.ctx.clearRect(0, 0, width, height);

    // Draw layers
    this.drawHorizonLine();
    this.drawRouteProjection(waypoints);
    this.drawTurnArrows(waypoints);
    this.drawDistanceMarkers(waypoints);
    this.drawHazardWarnings(hazards);
    this.drawSpeedIndicator();
    this.drawCompass();
    this.drawHUDFrame();
  }

  // ─── Drawing Methods ───

  private drawHorizonLine(): void {
    if (!this.ctx) return;
    const { width, vanishingPointY, primaryColor } = this.config;
    const y = this.config.height * vanishingPointY;

    this.ctx.beginPath();
    this.ctx.moveTo(0, y);
    this.ctx.lineTo(width, y);
    this.ctx.strokeStyle = primaryColor + '30';
    this.ctx.lineWidth = 1;
    this.ctx.setLineDash([10, 10]);
    this.ctx.stroke();
    this.ctx.setLineDash([]);

    // Horizon glow
    const gradient = this.ctx.createLinearGradient(0, y - 20, 0, y + 20);
    gradient.addColorStop(0, 'transparent');
    gradient.addColorStop(0.5, primaryColor + '10');
    gradient.addColorStop(1, 'transparent');
    this.ctx.fillStyle = gradient;
    this.ctx.fillRect(0, y - 20, width, 40);
  }

  private drawRouteProjection(waypoints: ARWaypoint[]): void {
    if (!this.ctx || waypoints.length === 0) return;
    const { width, height, primaryColor, vanishingPointY, fov } = this.config;
    const vpY = height * vanishingPointY;
    const vpX = width / 2;

    // Project waypoints onto the HUD
    this.ctx.beginPath();
    this.ctx.moveTo(vpX, height); // Start from bottom center

    const projected = waypoints
      .filter(wp => wp.distance < 2000) // Only show within 2km
      .map(wp => this.projectToScreen(wp.bearing, wp.distance, vpX, vpY, width, height, fov));

    // Draw route path
    if (projected.length > 0) {
      // Left edge of road
      this.ctx.beginPath();
      this.ctx.moveTo(vpX - 80, height);
      projected.forEach(p => {
        this.ctx!.lineTo(p.x - p.roadWidth / 2, p.y);
      });
      this.ctx.strokeStyle = primaryColor + '60';
      this.ctx.lineWidth = 2;
      this.ctx.stroke();

      // Right edge of road
      this.ctx.beginPath();
      this.ctx.moveTo(vpX + 80, height);
      projected.forEach(p => {
        this.ctx!.lineTo(p.x + p.roadWidth / 2, p.y);
      });
      this.ctx.strokeStyle = primaryColor + '60';
      this.ctx.lineWidth = 2;
      this.ctx.stroke();

      // Center line (dashed)
      this.ctx.beginPath();
      this.ctx.moveTo(vpX, height);
      projected.forEach(p => {
        this.ctx!.lineTo(p.x, p.y);
      });
      this.ctx.strokeStyle = primaryColor + '40';
      this.ctx.lineWidth = 1;
      this.ctx.setLineDash([15, 15]);
      this.ctx.stroke();
      this.ctx.setLineDash([]);

      // Route glow
      this.ctx.beginPath();
      this.ctx.moveTo(vpX - 80, height);
      projected.forEach(p => {
        this.ctx!.lineTo(p.x - p.roadWidth / 2, p.y);
      });
      projected.reverse().forEach(p => {
        this.ctx!.lineTo(p.x + p.roadWidth / 2, p.y);
      });
      this.ctx.closePath();
      const gradient = this.ctx.createLinearGradient(0, height, 0, vpY);
      gradient.addColorStop(0, primaryColor + '15');
      gradient.addColorStop(1, 'transparent');
      this.ctx.fillStyle = gradient;
      this.ctx.fill();
    }
  }

  private drawTurnArrows(waypoints: ARWaypoint[]): void {
    if (!this.ctx) return;
    const { width, accentColor } = this.config;

    // Find next turn
    const nextTurn = waypoints.find(wp => wp.turnType && wp.turnType !== 'straight' && wp.distance < 500);
    if (!nextTurn) return;

    const arrowX = nextTurn.turnType === 'left' ? width * 0.25 : width * 0.75;
    const arrowY = this.config.height * 0.45;
    const size = 40;

    // Pulsing effect
    const pulse = 0.8 + Math.sin(this.animFrame * 0.1) * 0.2;

    this.ctx.save();
    this.ctx.translate(arrowX, arrowY);
    this.ctx.scale(pulse, pulse);

    // Arrow
    this.ctx.beginPath();
    if (nextTurn.turnType === 'left') {
      this.ctx.moveTo(size, -size / 2);
      this.ctx.lineTo(-size / 2, 0);
      this.ctx.lineTo(size, size / 2);
    } else if (nextTurn.turnType === 'right') {
      this.ctx.moveTo(-size, -size / 2);
      this.ctx.lineTo(size / 2, 0);
      this.ctx.lineTo(-size, size / 2);
    } else if (nextTurn.turnType === 'u-turn') {
      this.ctx.arc(0, 0, size / 2, Math.PI, 0, false);
      this.ctx.lineTo(size / 2 + 10, -10);
    }
    this.ctx.strokeStyle = accentColor;
    this.ctx.lineWidth = 3;
    this.ctx.shadowColor = accentColor;
    this.ctx.shadowBlur = 15;
    this.ctx.stroke();
    this.ctx.shadowBlur = 0;

    // Distance text
    this.ctx.fillStyle = accentColor;
    this.ctx.font = 'bold 14px monospace';
    this.ctx.textAlign = 'center';
    this.ctx.fillText(`${Math.round(nextTurn.distance)}m`, 0, size + 10);

    // Instruction text
    if (nextTurn.instruction) {
      this.ctx.fillStyle = 'rgba(255,255,255,0.7)';
      this.ctx.font = '12px monospace';
      this.ctx.fillText(nextTurn.instruction, 0, size + 28);
    }

    this.ctx.restore();
  }

  private drawDistanceMarkers(waypoints: ARWaypoint[]): void {
    if (!this.ctx) return;
    const { primaryColor } = this.config;

    // Draw distance markers at 100m, 500m, 1km
    const markers = [100, 500, 1000];
    markers.forEach(dist => {
      const wp = waypoints.find(w => Math.abs(w.distance - dist) < 50);
      if (!wp) return;

      const projected = this.projectToScreen(
        wp.bearing, wp.distance,
        this.config.width / 2,
        this.config.height * this.config.vanishingPointY,
        this.config.width,
        this.config.height,
        this.config.fov
      );

      // Marker line
      this.ctx!.beginPath();
      this.ctx!.moveTo(projected.x - projected.roadWidth, projected.y);
      this.ctx!.lineTo(projected.x + projected.roadWidth, projected.y);
      this.ctx!.strokeStyle = primaryColor + '40';
      this.ctx!.lineWidth = 1;
      this.ctx!.stroke();

      // Distance label
      this.ctx!.fillStyle = primaryColor + '80';
      this.ctx!.font = '10px monospace';
      this.ctx!.textAlign = 'right';
      const label = dist >= 1000 ? `${dist / 1000}km` : `${dist}m`;
      this.ctx!.fillText(label, projected.x + projected.roadWidth + 30, projected.y + 4);
    });
  }

  private drawHazardWarnings(hazards: ARHazard[]): void {
    if (!this.ctx || hazards.length === 0) return;
    const { dangerColor, width, height, vanishingPointY, fov } = this.config;
    const vpX = width / 2;
    const vpY = height * vanishingPointY;

    hazards
      .filter(h => h.distance < 1000)
      .forEach(hazard => {
        const p = this.projectToScreen(hazard.bearing, hazard.distance, vpX, vpY, width, height, fov);

        // Warning triangle
        const size = 12 + (1 - hazard.distance / 1000) * 8;
        const pulse = 0.7 + Math.sin(this.animFrame * 0.15) * 0.3;

        this.ctx!.save();
        this.ctx!.translate(p.x, p.y);
        this.ctx!.scale(pulse, pulse);

        // Triangle
        this.ctx!.beginPath();
        this.ctx!.moveTo(0, -size);
        this.ctx!.lineTo(-size * 0.866, size * 0.5);
        this.ctx!.lineTo(size * 0.866, size * 0.5);
        this.ctx!.closePath();
        this.ctx!.fillStyle = dangerColor + '30';
        this.ctx!.fill();
        this.ctx!.strokeStyle = dangerColor;
        this.ctx!.lineWidth = 2;
        this.ctx!.shadowColor = dangerColor;
        this.ctx!.shadowBlur = 10;
        this.ctx!.stroke();
        this.ctx!.shadowBlur = 0;

        // Exclamation mark
        this.ctx!.fillStyle = dangerColor;
        this.ctx!.font = `bold ${size}px monospace`;
        this.ctx!.textAlign = 'center';
        this.ctx!.fillText('!', 0, size * 0.3);

        this.ctx!.restore();
      });
  }

  private drawSpeedIndicator(): void {
    if (!this.ctx) return;
    const { primaryColor, height } = this.config;
    const x = 60;
    const y = height - 50;

    // Speed value
    this.ctx.fillStyle = primaryColor;
    this.ctx.font = 'bold 28px monospace';
    this.ctx.textAlign = 'center';
    this.ctx.fillText(Math.round(this.currentSpeed).toString(), x, y);

    // Unit
    this.ctx.fillStyle = 'rgba(255,255,255,0.4)';
    this.ctx.font = '10px monospace';
    this.ctx.fillText('km/h', x, y + 16);
  }

  private drawCompass(): void {
    if (!this.ctx) return;
    const { width, primaryColor } = this.config;
    const x = width - 50;
    const y = 40;
    const radius = 25;

    // Compass circle
    this.ctx.beginPath();
    this.ctx.arc(x, y, radius, 0, Math.PI * 2);
    this.ctx.strokeStyle = primaryColor + '30';
    this.ctx.lineWidth = 1;
    this.ctx.stroke();

    // North indicator
    const headingRad = -this.currentHeading * Math.PI / 180;
    const nx = x + Math.sin(headingRad) * (radius - 5);
    const ny = y - Math.cos(headingRad) * (radius - 5);

    this.ctx.beginPath();
    this.ctx.arc(nx, ny, 3, 0, Math.PI * 2);
    this.ctx.fillStyle = '#ff3355';
    this.ctx.fill();

    // N label
    this.ctx.fillStyle = '#ff3355';
    this.ctx.font = 'bold 8px monospace';
    this.ctx.textAlign = 'center';
    const nlx = x + Math.sin(headingRad) * (radius + 8);
    const nly = y - Math.cos(headingRad) * (radius + 8);
    this.ctx.fillText('N', nlx, nly + 3);

    // Heading value
    this.ctx.fillStyle = primaryColor + '80';
    this.ctx.font = '10px monospace';
    this.ctx.fillText(`${Math.round(this.currentHeading)}°`, x, y + 4);
  }

  private drawHUDFrame(): void {
    if (!this.ctx) return;
    const { width, height, primaryColor } = this.config;
    const cornerSize = 20;

    this.ctx.strokeStyle = primaryColor + '25';
    this.ctx.lineWidth = 1;

    // Top-left corner
    this.ctx.beginPath();
    this.ctx.moveTo(0, cornerSize);
    this.ctx.lineTo(0, 0);
    this.ctx.lineTo(cornerSize, 0);
    this.ctx.stroke();

    // Top-right corner
    this.ctx.beginPath();
    this.ctx.moveTo(width - cornerSize, 0);
    this.ctx.lineTo(width, 0);
    this.ctx.lineTo(width, cornerSize);
    this.ctx.stroke();

    // Bottom-left corner
    this.ctx.beginPath();
    this.ctx.moveTo(0, height - cornerSize);
    this.ctx.lineTo(0, height);
    this.ctx.lineTo(cornerSize, height);
    this.ctx.stroke();

    // Bottom-right corner
    this.ctx.beginPath();
    this.ctx.moveTo(width - cornerSize, height);
    this.ctx.lineTo(width, height);
    this.ctx.lineTo(width, height - cornerSize);
    this.ctx.stroke();
  }

  // ─── Projection ───

  private projectToScreen(
    bearing: number, distance: number,
    vpX: number, vpY: number,
    width: number, height: number,
    fov: number
  ): { x: number; y: number; roadWidth: number } {
    // Relative bearing to heading
    let relBearing = bearing - this.currentHeading;
    if (relBearing > 180) relBearing -= 360;
    if (relBearing < -180) relBearing += 360;

    // Perspective projection
    const maxDist = 2000;
    const t = Math.min(distance / maxDist, 1); // 0=close, 1=far
    const perspective = 1 - t;

    // X position based on bearing
    const xOffset = (relBearing / (fov / 2)) * (width / 2);
    const x = vpX + xOffset * perspective;

    // Y position (perspective: far=horizon, close=bottom)
    const y = vpY + (height - vpY) * perspective;

    // Road width (narrows with distance)
    const roadWidth = 160 * perspective + 4;

    return { x, y, roadWidth };
  }
}
