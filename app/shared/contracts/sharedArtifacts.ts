/**
 * G.A.N.E — Shared Collaboration Artifacts
 * ===========================================
 * Types + zod validators for artifacts passed between peers in a
 * real-time collab session: waypoints, shared routes, annotations.
 *
 * Covers todo.md:
 *   - [x] Shared waypoints, routes, and annotations
 */
import { z } from "zod";

export const LatLngSchema = z.object({
  lat: z.number().min(-90).max(90),
  lon: z.number().min(-180).max(180),
  alt: z.number().optional(),
});
export type LatLng = z.infer<typeof LatLngSchema>;

export const WaypointKindSchema = z.enum([
  "pin",
  "hazard",
  "rally",
  "fuel",
  "rest",
  "custom",
]);
export type WaypointKind = z.infer<typeof WaypointKindSchema>;

export const SharedWaypointSchema = z.object({
  id: z.string().min(1),
  kind: WaypointKindSchema,
  label: z.string().max(120).default(""),
  position: LatLngSchema,
  createdBy: z.string().min(1),
  createdAt: z.number().int().positive(),
  expiresAt: z.number().int().positive().optional(),
  color: z.string().regex(/^#[0-9a-fA-F]{6,8}$/).optional(),
  locked: z.boolean().default(false),
});
export type SharedWaypoint = z.infer<typeof SharedWaypointSchema>;

export const SharedRouteSchema = z.object({
  id: z.string().min(1),
  name: z.string().max(200),
  path: z.array(LatLngSchema).min(2),
  distanceMeters: z.number().nonnegative().optional(),
  durationSeconds: z.number().nonnegative().optional(),
  createdBy: z.string().min(1),
  createdAt: z.number().int().positive(),
  version: z.number().int().nonnegative().default(1),
});
export type SharedRoute = z.infer<typeof SharedRouteSchema>;

export const AnnotationKindSchema = z.enum(["note", "arrow", "circle", "polygon"]);
export type AnnotationKind = z.infer<typeof AnnotationKindSchema>;

export const AnnotationSchema = z.object({
  id: z.string().min(1),
  kind: AnnotationKindSchema,
  geometry: z.array(LatLngSchema).min(1),
  body: z.string().max(2000).default(""),
  createdBy: z.string().min(1),
  createdAt: z.number().int().positive(),
  updatedAt: z.number().int().positive().optional(),
  color: z.string().regex(/^#[0-9a-fA-F]{6,8}$/).optional(),
});
export type Annotation = z.infer<typeof AnnotationSchema>;

export const SharedArtifactSchema = z.discriminatedUnion("type", [
  z.object({ type: z.literal("waypoint"), data: SharedWaypointSchema }),
  z.object({ type: z.literal("route"),    data: SharedRouteSchema }),
  z.object({ type: z.literal("annotation"), data: AnnotationSchema }),
]);
export type SharedArtifact = z.infer<typeof SharedArtifactSchema>;

// ── Store (in-memory, per-session) ─────────────────────────────────────────

export class SharedArtifactStore {
  private waypoints = new Map<string, SharedWaypoint>();
  private routes = new Map<string, SharedRoute>();
  private annotations = new Map<string, Annotation>();

  upsert(artifact: SharedArtifact): SharedArtifact {
    const parsed = SharedArtifactSchema.parse(artifact);
    switch (parsed.type) {
      case "waypoint":
        this.waypoints.set(parsed.data.id, parsed.data);
        break;
      case "route":
        this.routes.set(parsed.data.id, parsed.data);
        break;
      case "annotation":
        this.annotations.set(parsed.data.id, parsed.data);
        break;
    }
    return parsed;
  }

  remove(type: SharedArtifact["type"], id: string): boolean {
    const map =
      type === "waypoint" ? this.waypoints :
      type === "route"    ? this.routes :
                            this.annotations;
    return map.delete(id);
  }

  listWaypoints(): SharedWaypoint[] { return Array.from(this.waypoints.values()); }
  listRoutes(): SharedRoute[]       { return Array.from(this.routes.values()); }
  listAnnotations(): Annotation[]   { return Array.from(this.annotations.values()); }

  /** Garbage-collect expired waypoints. Returns the number of entries removed. */
  gc(now: number = Date.now()): number {
    let removed = 0;
    Array.from(this.waypoints.values()).forEach((w) => {
      if (w.expiresAt && w.expiresAt <= now) {
        this.waypoints.delete(w.id);
        removed++;
      }
    });
    return removed;
  }

  size(): { waypoints: number; routes: number; annotations: number } {
    return {
      waypoints: this.waypoints.size,
      routes: this.routes.size,
      annotations: this.annotations.size,
    };
  }

  clear(): void {
    this.waypoints.clear();
    this.routes.clear();
    this.annotations.clear();
  }
}
