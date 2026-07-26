/**
 * Tests for shared/contracts/sharedArtifacts.ts
 * Schema validation + SharedArtifactStore upsert / remove / gc / list.
 */
import { describe, it, expect } from "vitest";
import {
  AnnotationSchema,
  SharedArtifactSchema,
  SharedArtifactStore,
  SharedRouteSchema,
  SharedWaypointSchema,
  type SharedArtifact,
} from "../../shared/contracts/sharedArtifacts";

const now = () => Date.now();

describe("schema validation", () => {
  it("accepts a well-formed waypoint", () => {
    const wp = {
      id: "wp-1",
      kind: "pin" as const,
      label: "home",
      position: { lat: 32.08, lon: 34.78 },
      createdBy: "alice",
      createdAt: now(),
      locked: false,
    };
    expect(SharedWaypointSchema.parse(wp).id).toBe("wp-1");
  });

  it("rejects a waypoint with out-of-range lat", () => {
    const bad = {
      id: "x",
      kind: "pin" as const,
      label: "",
      position: { lat: 999, lon: 0 },
      createdBy: "alice",
      createdAt: now(),
      locked: false,
    };
    expect(() => SharedWaypointSchema.parse(bad)).toThrow();
  });

  it("requires a route of at least 2 points", () => {
    const bad = {
      id: "r1",
      name: "too short",
      path: [{ lat: 0, lon: 0 }],
      createdBy: "a",
      createdAt: now(),
      version: 1,
    };
    expect(() => SharedRouteSchema.parse(bad)).toThrow();
  });

  it("accepts a simple annotation", () => {
    const ann = {
      id: "a1",
      kind: "note" as const,
      geometry: [{ lat: 0, lon: 0 }],
      body: "watch out — gravel",
      createdBy: "bob",
      createdAt: now(),
    };
    expect(AnnotationSchema.parse(ann).body).toMatch(/gravel/);
  });

  it("tags the discriminator on the union type", () => {
    const a: SharedArtifact = {
      type: "waypoint",
      data: {
        id: "w1",
        kind: "hazard",
        label: "",
        position: { lat: 0, lon: 0 },
        createdBy: "alice",
        createdAt: now(),
        locked: false,
      },
    };
    expect(SharedArtifactSchema.parse(a).type).toBe("waypoint");
  });
});

describe("SharedArtifactStore", () => {
  it("upserts and lists each artifact type", () => {
    const store = new SharedArtifactStore();
    store.upsert({
      type: "waypoint",
      data: {
        id: "w1", kind: "pin", label: "", position: { lat: 0, lon: 0 },
        createdBy: "a", createdAt: now(), locked: false,
      },
    });
    store.upsert({
      type: "route",
      data: {
        id: "r1", name: "short", path: [{ lat: 0, lon: 0 }, { lat: 1, lon: 1 }],
        createdBy: "a", createdAt: now(), version: 1,
      },
    });
    store.upsert({
      type: "annotation",
      data: {
        id: "n1", kind: "note", geometry: [{ lat: 0, lon: 0 }],
        body: "", createdBy: "a", createdAt: now(),
      },
    });
    const sizes = store.size();
    expect(sizes.waypoints).toBe(1);
    expect(sizes.routes).toBe(1);
    expect(sizes.annotations).toBe(1);
    expect(store.listWaypoints()[0].id).toBe("w1");
  });

  it("upsert replaces an existing id rather than duplicating", () => {
    const store = new SharedArtifactStore();
    const base = {
      id: "w1", kind: "pin" as const, label: "",
      position: { lat: 0, lon: 0 }, createdBy: "a",
      createdAt: now(), locked: false,
    };
    store.upsert({ type: "waypoint", data: base });
    store.upsert({ type: "waypoint", data: { ...base, label: "renamed" } });
    expect(store.size().waypoints).toBe(1);
    expect(store.listWaypoints()[0].label).toBe("renamed");
  });

  it("remove returns false for unknown ids", () => {
    const store = new SharedArtifactStore();
    expect(store.remove("waypoint", "nope")).toBe(false);
  });

  it("gc removes expired waypoints only", () => {
    const store = new SharedArtifactStore();
    const t = now();
    store.upsert({
      type: "waypoint",
      data: {
        id: "live", kind: "pin", label: "", position: { lat: 0, lon: 0 },
        createdBy: "a", createdAt: t, expiresAt: t + 60_000, locked: false,
      },
    });
    store.upsert({
      type: "waypoint",
      data: {
        id: "dead", kind: "pin", label: "", position: { lat: 0, lon: 0 },
        createdBy: "a", createdAt: t - 120_000, expiresAt: t - 60_000, locked: false,
      },
    });
    const removed = store.gc(t);
    expect(removed).toBe(1);
    expect(store.listWaypoints().map((w) => w.id)).toEqual(["live"]);
  });

  it("clear empties all three collections", () => {
    const store = new SharedArtifactStore();
    store.upsert({
      type: "annotation",
      data: {
        id: "a", kind: "note", geometry: [{ lat: 0, lon: 0 }],
        body: "", createdBy: "x", createdAt: now(),
      },
    });
    store.clear();
    expect(store.size()).toEqual({ waypoints: 0, routes: 0, annotations: 0 });
  });
});
