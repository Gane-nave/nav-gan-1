/**
 * CollaboratorCursors — Renders remote user cursors on the Google Map
 * ====================================================================
 * Uses Google Maps OverlayView to position colored cursor markers
 * with user names at each collaborator's lat/lon position.
 * Smooth CSS transitions for cursor movement.
 */
import { useEffect, useRef, useCallback } from "react";
import { useNavigation } from "@/contexts/NavigationContext";

type CursorData = { lat: number; lon: number; color: string; name?: string };

interface CollaboratorCursorsProps {
  cursors: Map<number, CursorData>;
  currentUserId?: number;
}

// Cursor overlay element factory
function createCursorElement(color: string, name: string): HTMLDivElement {
  const container = document.createElement("div");
  container.style.cssText = `
    position: absolute;
    transform: translate(-50%, -100%);
    transition: left 300ms ease-out, top 300ms ease-out;
    pointer-events: none;
    z-index: 999;
    filter: drop-shadow(0 2px 6px rgba(0,0,0,0.4));
  `;

  // Cursor arrow SVG
  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.setAttribute("width", "24");
  svg.setAttribute("height", "28");
  svg.setAttribute("viewBox", "0 0 24 28");
  svg.style.display = "block";

  const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
  path.setAttribute("d", "M5.65 2.2 L21 12 L13.5 13.5 L10 21.5 Z");
  path.setAttribute("fill", color);
  path.setAttribute("stroke", "rgba(255,255,255,0.9)");
  path.setAttribute("stroke-width", "1.5");
  path.setAttribute("stroke-linejoin", "round");
  svg.appendChild(path);
  container.appendChild(svg);

  // Name label
  const label = document.createElement("div");
  label.textContent = name;
  label.style.cssText = `
    background: ${color};
    color: #fff;
    font-size: 10px;
    font-weight: 600;
    font-family: system-ui, -apple-system, sans-serif;
    padding: 2px 6px;
    border-radius: 4px;
    margin-top: 2px;
    white-space: nowrap;
    letter-spacing: 0.3px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.3);
  `;
  container.appendChild(label);

  // Pulse ring
  const pulse = document.createElement("div");
  pulse.style.cssText = `
    position: absolute;
    top: 20px;
    left: 50%;
    transform: translateX(-50%);
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: ${color};
    opacity: 0.4;
    animation: cursorPulse 2s ease-out infinite;
  `;
  container.appendChild(pulse);

  return container;
}

// Inject pulse animation CSS once
let styleInjected = false;
function injectCursorStyles() {
  if (styleInjected) return;
  styleInjected = true;
  const style = document.createElement("style");
  style.textContent = `
    @keyframes cursorPulse {
      0% { transform: translateX(-50%) scale(1); opacity: 0.4; }
      70% { transform: translateX(-50%) scale(2.5); opacity: 0; }
      100% { transform: translateX(-50%) scale(2.5); opacity: 0; }
    }
  `;
  document.head.appendChild(style);
}

// Custom OverlayView class for a single cursor
class CursorOverlay {
  private overlay: google.maps.OverlayView;
  private position: google.maps.LatLng;
  private element: HTMLDivElement;
  private pane: Element | null = null;

  constructor(map: google.maps.Map, lat: number, lon: number, color: string, name: string) {
    this.position = new google.maps.LatLng(lat, lon);
    this.element = createCursorElement(color, name);
    this.overlay = new google.maps.OverlayView();

    this.overlay.onAdd = () => {
      this.pane = this.overlay.getPanes()?.overlayMouseTarget ?? null;
      if (this.pane) {
        this.pane.appendChild(this.element);
      }
    };

    this.overlay.draw = () => {
      const projection = this.overlay.getProjection();
      if (!projection) return;
      const point = projection.fromLatLngToDivPixel(this.position);
      if (point) {
        this.element.style.left = point.x + "px";
        this.element.style.top = point.y + "px";
      }
    };

    this.overlay.onRemove = () => {
      if (this.element.parentNode) {
        this.element.parentNode.removeChild(this.element);
      }
    };

    this.overlay.setMap(map);
  }

  updatePosition(lat: number, lon: number) {
    this.position = new google.maps.LatLng(lat, lon);
    const projection = this.overlay.getProjection();
    if (projection) {
      const point = projection.fromLatLngToDivPixel(this.position);
      if (point) {
        // CSS transition handles smooth movement
        this.element.style.left = point.x + "px";
        this.element.style.top = point.y + "px";
      }
    }
  }

  remove() {
    this.overlay.setMap(null);
  }
}

export default function CollaboratorCursors({ cursors, currentUserId }: CollaboratorCursorsProps) {
  const { mapRef } = useNavigation();
  const overlaysRef = useRef<Map<number, CursorOverlay>>(new Map());
  const staleTimersRef = useRef<Map<number, ReturnType<typeof setTimeout>>>(new Map());

  useEffect(() => {
    injectCursorStyles();
  }, []);

  // Sync overlays with cursor data
  useEffect(() => {
    const map = mapRef.current;
    if (!map || !window.google) return;

    const currentOverlays = overlaysRef.current;
    const activeCursorIds = new Set<number>();

    cursors.forEach((cursor, userId) => {
      // Skip own cursor
      if (userId === currentUserId) return;
      activeCursorIds.add(userId);

      const existing = currentOverlays.get(userId);
      if (existing) {
        // Update position with smooth transition
        existing.updatePosition(cursor.lat, cursor.lon);
      } else {
        // Create new overlay
        const overlay = new CursorOverlay(
          map,
          cursor.lat,
          cursor.lon,
          cursor.color,
          cursor.name || `User ${userId}`
        );
        currentOverlays.set(userId, overlay);
      }

      // Reset stale timer
      const existingTimer = staleTimersRef.current.get(userId);
      if (existingTimer) clearTimeout(existingTimer);
      staleTimersRef.current.set(userId, setTimeout(() => {
        // Auto-hide after 30s of no updates
        const overlay = currentOverlays.get(userId);
        if (overlay) {
          overlay.remove();
          currentOverlays.delete(userId);
        }
        staleTimersRef.current.delete(userId);
      }, 30000));
    });

    // Remove overlays for users no longer in the cursor map
    currentOverlays.forEach((overlay, userId) => {
      if (!activeCursorIds.has(userId)) {
        overlay.remove();
        currentOverlays.delete(userId);
        const timer = staleTimersRef.current.get(userId);
        if (timer) {
          clearTimeout(timer);
          staleTimersRef.current.delete(userId);
        }
      }
    });
  }, [cursors, currentUserId, mapRef]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      overlaysRef.current.forEach((overlay) => overlay.remove());
      overlaysRef.current.clear();
      staleTimersRef.current.forEach((timer) => clearTimeout(timer));
      staleTimersRef.current.clear();
    };
  }, []);

  // This component renders nothing directly — it manages Google Maps overlays imperatively
  return null;
}
