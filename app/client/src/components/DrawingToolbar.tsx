/**
 * DrawingToolbar — Collaborative Drawing Tools for Map
 * =====================================================
 * Provides freehand, polygon, and polyline drawing tools
 * that sync via the collaboration annotation system.
 * 
 * Architecture:
 * - Uses Google Maps Drawing/Overlay API for rendering
 * - Stores drawings as annotations via useCollaboration
 * - Remote drawings rendered from annotation query data
 * - Real-time sync via SSE invalidation
 */
import { useState, useCallback, useRef, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Pencil, Pentagon, Minus, Trash2, X, Check,
  Palette, Undo2, MousePointer2, Spline
} from "lucide-react";

export type DrawingTool = "select" | "freehand" | "polygon" | "polyline" | null;

type DrawingPoint = { lat: number; lng: number };

type DrawingShape = {
  id: string;
  type: "freehand" | "polygon" | "polyline";
  points: DrawingPoint[];
  color: string;
  strokeWidth: number;
  userId?: number;
  annotationId?: string;
};

const DRAWING_COLORS = [
  "#00e5ff", "#00ff88", "#ff3355", "#aa66ff",
  "#ff9900", "#ffd700", "#4488ff", "#ff44aa",
];

interface DrawingToolbarProps {
  map: google.maps.Map | null;
  isCollaborating: boolean;
  onAddAnnotation?: (
    type: "text" | "route" | "area" | "measurement" | "arrow",
    data: Record<string, unknown>,
    color?: string
  ) => Promise<unknown>;
  onDeleteAnnotation?: (annotationId: string) => Promise<unknown>;
  remoteAnnotations?: Array<{
    annotationId: string;
    type: string;
    data: unknown;
    color: string | null;
    userId: number;
  }>;
}

export default function DrawingToolbar({
  map,
  isCollaborating,
  onAddAnnotation,
  onDeleteAnnotation,
  remoteAnnotations = [],
}: DrawingToolbarProps) {
  const [activeTool, setActiveTool] = useState<DrawingTool>(null);
  const [selectedColor, setSelectedColor] = useState(DRAWING_COLORS[0]);
  const [strokeWidth, setStrokeWidth] = useState(3);
  const [showColorPicker, setShowColorPicker] = useState(false);
  const [isDrawing, setIsDrawing] = useState(false);
  const [currentPoints, setCurrentPoints] = useState<DrawingPoint[]>([]);
  const [localShapes, setLocalShapes] = useState<DrawingShape[]>([]);
  const [selectedShapeId, setSelectedShapeId] = useState<string | null>(null);

  // Google Maps overlay references
  const polylinesRef = useRef<Map<string, google.maps.Polyline>>(new Map());
  const polygonsRef = useRef<Map<string, google.maps.Polygon>>(new Map());
  const currentOverlayRef = useRef<google.maps.Polyline | null>(null);
  const clickListenerRef = useRef<google.maps.MapsEventListener | null>(null);
  const moveListenerRef = useRef<google.maps.MapsEventListener | null>(null);
  const dblClickListenerRef = useRef<google.maps.MapsEventListener | null>(null);

  // ─── Generate unique ID ───
  const genLocalId = () => `draw_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;

  // ─── Clean up current drawing ───
  const cleanupCurrentDrawing = useCallback(() => {
    if (currentOverlayRef.current) {
      currentOverlayRef.current.setMap(null);
      currentOverlayRef.current = null;
    }
    if (clickListenerRef.current) {
      google.maps.event.removeListener(clickListenerRef.current);
      clickListenerRef.current = null;
    }
    if (moveListenerRef.current) {
      google.maps.event.removeListener(moveListenerRef.current);
      moveListenerRef.current = null;
    }
    if (dblClickListenerRef.current) {
      google.maps.event.removeListener(dblClickListenerRef.current);
      dblClickListenerRef.current = null;
    }
    setIsDrawing(false);
    setCurrentPoints([]);
  }, []);

  // ─── Render a shape on the map ───
  const renderShape = useCallback(
    (shape: DrawingShape) => {
      if (!map) return;

      const path = shape.points.map((p) => ({ lat: p.lat, lng: p.lng }));

      if (shape.type === "polygon") {
        // Remove existing if updating
        const existing = polygonsRef.current.get(shape.id);
        if (existing) existing.setMap(null);

        const polygon = new google.maps.Polygon({
          paths: path,
          strokeColor: shape.color,
          strokeWeight: shape.strokeWidth,
          strokeOpacity: 0.9,
          fillColor: shape.color,
          fillOpacity: 0.15,
          map,
          clickable: true,
          zIndex: 100,
        });

        polygon.addListener("click", () => {
          setSelectedShapeId(shape.id);
        });

        polygonsRef.current.set(shape.id, polygon);
      } else {
        // Polyline or freehand
        const existing = polylinesRef.current.get(shape.id);
        if (existing) existing.setMap(null);

        const polyline = new google.maps.Polyline({
          path,
          strokeColor: shape.color,
          strokeWeight: shape.strokeWidth,
          strokeOpacity: 0.9,
          map,
          clickable: true,
          zIndex: 100,
          geodesic: shape.type === "polyline",
        });

        polyline.addListener("click", () => {
          setSelectedShapeId(shape.id);
        });

        polylinesRef.current.set(shape.id, polyline);
      }
    },
    [map]
  );

  // ─── Remove a shape from the map ───
  const removeShapeFromMap = useCallback((shapeId: string) => {
    const polyline = polylinesRef.current.get(shapeId);
    if (polyline) {
      polyline.setMap(null);
      polylinesRef.current.delete(shapeId);
    }
    const polygon = polygonsRef.current.get(shapeId);
    if (polygon) {
      polygon.setMap(null);
      polygonsRef.current.delete(shapeId);
    }
  }, []);

  // ─── Finalize drawing ───
  const finalizeDrawing = useCallback(
    async (points: DrawingPoint[], type: "freehand" | "polygon" | "polyline") => {
      if (points.length < 2) return;

      const shapeId = genLocalId();
      const shape: DrawingShape = {
        id: shapeId,
        type,
        points,
        color: selectedColor,
        strokeWidth,
      };

      // Add locally
      setLocalShapes((prev) => [...prev, shape]);
      renderShape(shape);

      // Sync to collaboration if active
      if (isCollaborating && onAddAnnotation) {
        try {
          // Map drawing type to annotation type
          const annotationType = type === "polygon" ? "area" : "route";
          const result = await onAddAnnotation(
            annotationType,
            {
              drawingType: type,
              points: points.map((p) => [p.lat, p.lng]),
              strokeWidth,
            },
            selectedColor
          );

          // Update local shape with annotation ID
          if (result && typeof result === "object" && "annotationId" in result) {
            setLocalShapes((prev) =>
              prev.map((s) =>
                s.id === shapeId
                  ? { ...s, annotationId: (result as { annotationId: string }).annotationId }
                  : s
              )
            );
          }
        } catch (err) {
          console.error("Failed to sync drawing:", err);
        }
      }

      cleanupCurrentDrawing();
    },
    [selectedColor, strokeWidth, isCollaborating, onAddAnnotation, renderShape, cleanupCurrentDrawing]
  );

  // ─── Start Freehand Drawing ───
  const startFreehand = useCallback(() => {
    if (!map) return;
    cleanupCurrentDrawing();
    setIsDrawing(true);

    const points: DrawingPoint[] = [];
    let isMouseDown = false;

    // Create a live polyline for visual feedback
    const livePolyline = new google.maps.Polyline({
      strokeColor: selectedColor,
      strokeWeight: strokeWidth,
      strokeOpacity: 0.8,
      map,
      zIndex: 200,
    });
    currentOverlayRef.current = livePolyline;

    // Disable map dragging during freehand
    map.setOptions({ draggable: false });

    const mouseDownListener = map.addListener("mousedown", (e: google.maps.MapMouseEvent) => {
      isMouseDown = true;
      if (e.latLng) {
        const point = { lat: e.latLng.lat(), lng: e.latLng.lng() };
        points.push(point);
        livePolyline.getPath().push(e.latLng);
      }
    });

    moveListenerRef.current = map.addListener("mousemove", (e: google.maps.MapMouseEvent) => {
      if (!isMouseDown || !e.latLng) return;
      const point = { lat: e.latLng.lat(), lng: e.latLng.lng() };
      points.push(point);
      livePolyline.getPath().push(e.latLng);
    });

    const mouseUpListener = map.addListener("mouseup", () => {
      isMouseDown = false;
      map.setOptions({ draggable: true });

      // Simplify path — keep every Nth point for performance
      const simplified = simplifyPath(points, 0.00005);
      if (simplified.length >= 2) {
        finalizeDrawing(simplified, "freehand");
      } else {
        cleanupCurrentDrawing();
      }

      // Clean up freehand-specific listeners
      google.maps.event.removeListener(mouseDownListener);
      google.maps.event.removeListener(mouseUpListener);
    });

    clickListenerRef.current = mouseDownListener;
  }, [map, selectedColor, strokeWidth, cleanupCurrentDrawing, finalizeDrawing]);

  // ─── Start Polygon/Polyline Drawing ───
  const startStructuredDrawing = useCallback(
    (type: "polygon" | "polyline") => {
      if (!map) return;
      cleanupCurrentDrawing();
      setIsDrawing(true);

      const points: DrawingPoint[] = [];

      const livePolyline = new google.maps.Polyline({
        strokeColor: selectedColor,
        strokeWeight: strokeWidth,
        strokeOpacity: 0.8,
        map,
        zIndex: 200,
      });
      currentOverlayRef.current = livePolyline;

      // Click to add points
      clickListenerRef.current = map.addListener("click", (e: google.maps.MapMouseEvent) => {
        if (!e.latLng) return;
        const point = { lat: e.latLng.lat(), lng: e.latLng.lng() };
        points.push(point);
        livePolyline.getPath().push(e.latLng);
        setCurrentPoints([...points]);
      });

      // Double-click to finish
      dblClickListenerRef.current = map.addListener("dblclick", (e: google.maps.MapMouseEvent) => {
        e.stop?.();
        if (points.length >= 2) {
          finalizeDrawing(points, type);
        } else {
          cleanupCurrentDrawing();
        }
      });
    },
    [map, selectedColor, strokeWidth, cleanupCurrentDrawing, finalizeDrawing]
  );

  // ─── Tool Selection ───
  const handleToolSelect = useCallback(
    (tool: DrawingTool) => {
      if (tool === activeTool) {
        // Deselect
        setActiveTool(null);
        cleanupCurrentDrawing();
        return;
      }

      setActiveTool(tool);
      cleanupCurrentDrawing();

      if (tool === "freehand") {
        startFreehand();
      } else if (tool === "polygon" || tool === "polyline") {
        startStructuredDrawing(tool);
      }
    },
    [activeTool, cleanupCurrentDrawing, startFreehand, startStructuredDrawing]
  );

  // ─── Delete Selected Shape ───
  const handleDeleteSelected = useCallback(async () => {
    if (!selectedShapeId) return;

    const shape = localShapes.find((s) => s.id === selectedShapeId);
    removeShapeFromMap(selectedShapeId);
    setLocalShapes((prev) => prev.filter((s) => s.id !== selectedShapeId));
    setSelectedShapeId(null);

    // Sync deletion to collaboration
    if (shape?.annotationId && onDeleteAnnotation) {
      try {
        await onDeleteAnnotation(shape.annotationId);
      } catch (err) {
        console.error("Failed to delete annotation:", err);
      }
    }
  }, [selectedShapeId, localShapes, removeShapeFromMap, onDeleteAnnotation]);

  // ─── Undo Last Shape ───
  const handleUndo = useCallback(async () => {
    const lastShape = localShapes[localShapes.length - 1];
    if (!lastShape) return;

    removeShapeFromMap(lastShape.id);
    setLocalShapes((prev) => prev.slice(0, -1));

    if (lastShape.annotationId && onDeleteAnnotation) {
      try {
        await onDeleteAnnotation(lastShape.annotationId);
      } catch (err) {
        console.error("Failed to undo annotation:", err);
      }
    }
  }, [localShapes, removeShapeFromMap, onDeleteAnnotation]);

  // ─── Render Remote Annotations as Drawings ───
  useEffect(() => {
    if (!map) return;

    // Track which remote annotation IDs we've already rendered
    const localAnnotationIds = new Set(
      localShapes.filter((s) => s.annotationId).map((s) => s.annotationId)
    );

    const remoteDrawings = remoteAnnotations.filter(
      (a) =>
        (a.type === "route" || a.type === "area") &&
        a.data &&
        typeof a.data === "object" &&
        "drawingType" in (a.data as Record<string, unknown>) &&
        !localAnnotationIds.has(a.annotationId)
    );

    // Render remote drawings
    for (const annotation of remoteDrawings) {
      const data = annotation.data as Record<string, unknown>;
      const drawingType = data.drawingType as "freehand" | "polygon" | "polyline";
      const points = (data.points as number[][]).map((p) => ({
        lat: p[0],
        lng: p[1],
      }));
      const sw = (data.strokeWidth as number) || 3;

      const remoteId = `remote_${annotation.annotationId}`;

      // Skip if already rendered
      if (polylinesRef.current.has(remoteId) || polygonsRef.current.has(remoteId)) continue;

      const shape: DrawingShape = {
        id: remoteId,
        type: drawingType,
        points,
        color: annotation.color || "#00e5ff",
        strokeWidth: sw,
        userId: annotation.userId,
        annotationId: annotation.annotationId,
      };

      renderShape(shape);
    }

    // Clean up remote drawings that no longer exist
    const activeRemoteIds = new Set(remoteDrawings.map((a) => `remote_${a.annotationId}`));
    Array.from(polylinesRef.current.keys()).forEach((id) => {
      if (id.startsWith("remote_") && !activeRemoteIds.has(id)) {
        removeShapeFromMap(id);
      }
    });
    Array.from(polygonsRef.current.keys()).forEach((id) => {
      if (id.startsWith("remote_") && !activeRemoteIds.has(id)) {
        removeShapeFromMap(id);
      }
    });
  }, [map, remoteAnnotations, localShapes, renderShape, removeShapeFromMap]);

  // ─── Cleanup on unmount ───
  useEffect(() => {
    return () => {
      cleanupCurrentDrawing();
      // Remove all rendered shapes
      Array.from(polylinesRef.current.values()).forEach((polyline) => {
        polyline.setMap(null);
      });
      Array.from(polygonsRef.current.values()).forEach((polygon) => {
        polygon.setMap(null);
      });
      polylinesRef.current.clear();
      polygonsRef.current.clear();
    };
  }, [cleanupCurrentDrawing]);

  // ─── Tools Config ───
  const tools: { id: DrawingTool; icon: typeof Pencil; label: string; shortLabel: string }[] = [
    { id: "select", icon: MousePointer2, label: "Select", shortLabel: "SEL" },
    { id: "freehand", icon: Pencil, label: "Freehand", shortLabel: "FREE" },
    { id: "polyline", icon: Spline, label: "Polyline", shortLabel: "LINE" },
    { id: "polygon", icon: Pentagon, label: "Polygon", shortLabel: "POLY" },
  ];

  return (
    <motion.div
      initial={{ y: 20, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      className="absolute bottom-28 left-1/2 -translate-x-1/2 z-[70] flex items-center gap-2"
    >
      {/* Main Toolbar */}
      <div
        className="flex items-center gap-1 px-2 py-1.5 rounded-2xl"
        style={{
          background: "rgba(2, 6, 23, 0.92)",
          border: "1px solid rgba(0, 229, 255, 0.2)",
          backdropFilter: "blur(20px)",
          boxShadow: "0 8px 32px rgba(0, 0, 0, 0.4)",
        }}
      >
        {tools.map((tool) => {
          const Icon = tool.icon;
          const isActive = activeTool === tool.id;
          return (
            <button
              key={tool.id}
              onClick={() => handleToolSelect(tool.id)}
              className="relative flex flex-col items-center gap-0.5 px-3 py-2 rounded-xl transition-all"
              style={{
                background: isActive ? "rgba(0, 229, 255, 0.15)" : "transparent",
                border: isActive ? "1px solid rgba(0, 229, 255, 0.3)" : "1px solid transparent",
              }}
              title={tool.label}
            >
              <Icon
                size={18}
                style={{ color: isActive ? "#00e5ff" : "rgba(255,255,255,0.5)" }}
              />
              <span
                className="text-[9px] font-bold tracking-wider"
                style={{ color: isActive ? "#00e5ff" : "rgba(255,255,255,0.3)" }}
              >
                {tool.shortLabel}
              </span>
              {isActive && (
                <motion.div
                  layoutId="drawing-tool-indicator"
                  className="absolute -bottom-0.5 left-1/2 -translate-x-1/2 w-4 h-0.5 rounded-full"
                  style={{ background: "#00e5ff" }}
                />
              )}
            </button>
          );
        })}

        {/* Separator */}
        <div className="w-px h-8 bg-white/10 mx-1" />

        {/* Color Picker */}
        <div className="relative">
          <button
            onClick={() => setShowColorPicker(!showColorPicker)}
            className="flex items-center gap-1.5 px-2 py-2 rounded-xl hover:bg-white/5 transition-all"
            title="Drawing color"
          >
            <div
              className="w-4 h-4 rounded-full border border-white/20"
              style={{ background: selectedColor }}
            />
            <Palette size={14} style={{ color: "rgba(255,255,255,0.4)" }} />
          </button>

          <AnimatePresence>
            {showColorPicker && (
              <motion.div
                initial={{ opacity: 0, y: 10, scale: 0.95 }}
                animate={{ opacity: 1, y: 0, scale: 1 }}
                exit={{ opacity: 0, y: 10, scale: 0.95 }}
                className="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 p-2 rounded-xl grid grid-cols-4 gap-1.5"
                style={{
                  background: "rgba(2, 6, 23, 0.95)",
                  border: "1px solid rgba(0, 229, 255, 0.2)",
                  backdropFilter: "blur(20px)",
                }}
              >
                {DRAWING_COLORS.map((color) => (
                  <button
                    key={color}
                    onClick={() => {
                      setSelectedColor(color);
                      setShowColorPicker(false);
                    }}
                    className="w-7 h-7 rounded-full transition-transform hover:scale-110"
                    style={{
                      background: color,
                      border: color === selectedColor ? "2px solid white" : "2px solid transparent",
                      boxShadow: color === selectedColor ? `0 0 8px ${color}` : "none",
                    }}
                  />
                ))}

                {/* Stroke Width */}
                <div className="col-span-4 flex items-center gap-2 mt-1 pt-1 border-t border-white/10">
                  <Minus size={10} style={{ color: "rgba(255,255,255,0.3)" }} />
                  <input
                    type="range"
                    min={1}
                    max={8}
                    value={strokeWidth}
                    onChange={(e) => setStrokeWidth(Number(e.target.value))}
                    className="flex-1 h-1 appearance-none rounded-full"
                    style={{ background: "rgba(255,255,255,0.2)" }}
                  />
                  <span className="text-[10px] text-white/40 w-4 text-center">{strokeWidth}</span>
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>

        {/* Separator */}
        <div className="w-px h-8 bg-white/10 mx-1" />

        {/* Undo */}
        <button
          onClick={handleUndo}
          disabled={localShapes.length === 0}
          className="p-2 rounded-xl hover:bg-white/5 transition-all disabled:opacity-20"
          title="Undo last drawing"
        >
          <Undo2 size={16} style={{ color: "rgba(255,255,255,0.5)" }} />
        </button>

        {/* Delete Selected */}
        <button
          onClick={handleDeleteSelected}
          disabled={!selectedShapeId}
          className="p-2 rounded-xl hover:bg-red-500/10 transition-all disabled:opacity-20"
          title="Delete selected"
        >
          <Trash2 size={16} style={{ color: selectedShapeId ? "#ff3355" : "rgba(255,255,255,0.3)" }} />
        </button>

        {/* Clear All */}
        <button
          onClick={() => {
            localShapes.forEach((s) => removeShapeFromMap(s.id));
            setLocalShapes([]);
            setSelectedShapeId(null);
          }}
          disabled={localShapes.length === 0}
          className="p-2 rounded-xl hover:bg-red-500/10 transition-all disabled:opacity-20"
          title="Clear all drawings"
        >
          <X size={16} style={{ color: localShapes.length > 0 ? "#ff3355" : "rgba(255,255,255,0.3)" }} />
        </button>
      </div>

      {/* Drawing Status */}
      <AnimatePresence>
        {isDrawing && (
          <motion.div
            initial={{ opacity: 0, x: -10 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -10 }}
            className="px-3 py-2 rounded-xl text-xs font-medium"
            style={{
              background: "rgba(0, 229, 255, 0.1)",
              border: "1px solid rgba(0, 229, 255, 0.2)",
              color: "#00e5ff",
            }}
          >
            {activeTool === "freehand" ? (
              "Draw on map..."
            ) : (
              <span>
                Click to add points ({currentPoints.length})
                <br />
                <span className="text-[10px] text-white/40">Double-click to finish</span>
              </span>
            )}
          </motion.div>
        )}
      </AnimatePresence>

      {/* Shape Count Badge */}
      {localShapes.length > 0 && !isDrawing && (
        <div
          className="px-2 py-1 rounded-lg text-[10px] font-bold"
          style={{
            background: "rgba(0, 229, 255, 0.1)",
            color: "#00e5ff",
            border: "1px solid rgba(0, 229, 255, 0.15)",
          }}
        >
          {localShapes.length} shape{localShapes.length !== 1 ? "s" : ""}
        </div>
      )}
    </motion.div>
  );
}

// ─── Path Simplification (Ramer-Douglas-Peucker) ───
function simplifyPath(points: DrawingPoint[], epsilon: number): DrawingPoint[] {
  if (points.length <= 2) return points;

  let maxDist = 0;
  let maxIdx = 0;

  const start = points[0];
  const end = points[points.length - 1];

  for (let i = 1; i < points.length - 1; i++) {
    const dist = perpendicularDistance(points[i], start, end);
    if (dist > maxDist) {
      maxDist = dist;
      maxIdx = i;
    }
  }

  if (maxDist > epsilon) {
    const left = simplifyPath(points.slice(0, maxIdx + 1), epsilon);
    const right = simplifyPath(points.slice(maxIdx), epsilon);
    return [...left.slice(0, -1), ...right];
  }

  return [start, end];
}

function perpendicularDistance(
  point: DrawingPoint,
  lineStart: DrawingPoint,
  lineEnd: DrawingPoint
): number {
  const dx = lineEnd.lng - lineStart.lng;
  const dy = lineEnd.lat - lineStart.lat;
  const norm = Math.sqrt(dx * dx + dy * dy);
  if (norm === 0) return Math.sqrt(
    (point.lng - lineStart.lng) ** 2 + (point.lat - lineStart.lat) ** 2
  );
  return Math.abs(
    dy * point.lng - dx * point.lat + lineEnd.lng * lineStart.lat - lineEnd.lat * lineStart.lng
  ) / norm;
}
