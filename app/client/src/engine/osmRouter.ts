/**
 * G.A.N.E — OpenStreetMap Routing Graph Engine
 * =============================================
 * Builds a real routing graph from OpenStreetMap data via Overpass API.
 * Implements:
 *   - Overpass API query builder for road network extraction
 *   - Graph construction from OSM ways and nodes
 *   - A* pathfinding with real road distances
 *   - Turn-by-turn instruction generation
 *   - Offline graph caching via IndexedDB
 *   - Speed profile estimation from OSM highway tags
 *
 * This replaces the placeholder InternalAdapter with real OSM data.
 */

// ─── Types ───

export interface OSMNode {
  id: number;
  lat: number;
  lon: number;
}

export interface OSMWay {
  id: number;
  nodes: number[];
  tags: Record<string, string>;
}

export interface GraphNode {
  id: number;
  lat: number;
  lon: number;
  edges: GraphEdge[];
}

export interface GraphEdge {
  targetId: number;
  wayId: number;
  distanceM: number;
  durationS: number;
  speedKmh: number;
  roadName: string;
  roadType: string;
  oneway: boolean;
}

export interface OSMRouteResult {
  nodes: GraphNode[];
  totalDistanceM: number;
  totalDurationS: number;
  geometry: { lat: number; lon: number }[];
  instructions: RouteInstruction[];
}

export interface RouteInstruction {
  text: string;
  distanceM: number;
  durationS: number;
  maneuver: string;
  roadName: string;
  geometry: { lat: number; lon: number }[];
}

// ─── Speed Profiles (km/h) by OSM highway tag ───

const SPEED_PROFILES: Record<string, Record<string, number>> = {
  driving: {
    motorway: 110, trunk: 90, primary: 70, secondary: 60,
    tertiary: 50, residential: 30, service: 20, unclassified: 40,
    motorway_link: 60, trunk_link: 50, primary_link: 40,
    secondary_link: 35, tertiary_link: 30, living_street: 15,
    track: 15, road: 40,
  },
  walking: {
    motorway: 0, trunk: 0, // Pedestrians can't use motorways
    primary: 5, secondary: 5, tertiary: 5, residential: 5,
    service: 5, unclassified: 5, footway: 5, path: 4,
    pedestrian: 5, steps: 3, living_street: 5, track: 4, road: 5,
  },
  cycling: {
    motorway: 0, trunk: 0,
    primary: 20, secondary: 20, tertiary: 20, residential: 18,
    service: 15, unclassified: 18, cycleway: 20, path: 12,
    living_street: 15, track: 12, road: 18,
  },
};

// ─── Overpass API Query Builder ───

const OVERPASS_API = "https://overpass-api.de/api/interpreter";

function buildOverpassQuery(
  lat: number,
  lon: number,
  radiusM: number,
  profile: string
): string {
  // Filter highway types based on profile
  const highwayTypes =
    profile === "walking"
      ? '["highway"~"primary|secondary|tertiary|residential|service|unclassified|footway|path|pedestrian|steps|living_street|track"]'
      : profile === "cycling"
        ? '["highway"~"primary|secondary|tertiary|residential|service|unclassified|cycleway|path|living_street|track"]'
        : '["highway"~"motorway|trunk|primary|secondary|tertiary|residential|service|unclassified|motorway_link|trunk_link|primary_link|secondary_link|tertiary_link|living_street|road"]';

  return `
    [out:json][timeout:30];
    (
      way${highwayTypes}(around:${radiusM},${lat},${lon});
    );
    out body;
    >;
    out skel qt;
  `.trim();
}

// ─── Haversine Distance ───

function haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const R = 6371000; // Earth radius in meters
  const dLat = ((lat2 - lat1) * Math.PI) / 180;
  const dLon = ((lon2 - lon1) * Math.PI) / 180;
  const a =
    Math.sin(dLat / 2) ** 2 +
    Math.cos((lat1 * Math.PI) / 180) *
    Math.cos((lat2 * Math.PI) / 180) *
    Math.sin(dLon / 2) ** 2;
  return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

// ─── Bearing Calculation ───

function bearing(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const dLon = ((lon2 - lon1) * Math.PI) / 180;
  const y = Math.sin(dLon) * Math.cos((lat2 * Math.PI) / 180);
  const x =
    Math.cos((lat1 * Math.PI) / 180) * Math.sin((lat2 * Math.PI) / 180) -
    Math.sin((lat1 * Math.PI) / 180) * Math.cos((lat2 * Math.PI) / 180) * Math.cos(dLon);
  return ((Math.atan2(y, x) * 180) / Math.PI + 360) % 360;
}

function turnDirection(fromBearing: number, toBearing: number): string {
  let diff = ((toBearing - fromBearing) + 360) % 360;
  if (diff > 180) diff -= 360;
  if (Math.abs(diff) < 20) return "straight";
  if (diff > 0 && diff < 60) return "slight-right";
  if (diff >= 60 && diff < 120) return "right";
  if (diff >= 120) return "sharp-right";
  if (diff < 0 && diff > -60) return "slight-left";
  if (diff <= -60 && diff > -120) return "left";
  return "sharp-left";
}

// ─── OSM Routing Graph ───

export class OSMRoutingGraph {
  private nodes: Map<number, GraphNode> = new Map();
  private ways: Map<number, OSMWay> = new Map();
  private loaded = false;
  private loading = false;
  private lastCenter: { lat: number; lon: number } | null = null;
  private lastRadius = 0;

  // ─── IndexedDB Cache ───

  private async cacheGet(key: string): Promise<unknown | null> {
    try {
      if (typeof indexedDB === "undefined") return null;
      return new Promise((resolve) => {
        const req = indexedDB.open("gane_osm_cache", 1);
        req.onupgradeneeded = () => {
          const db = req.result;
          if (!db.objectStoreNames.contains("tiles")) {
            db.createObjectStore("tiles");
          }
        };
        req.onsuccess = () => {
          const tx = req.result.transaction("tiles", "readonly");
          const store = tx.objectStore("tiles");
          const getReq = store.get(key);
          getReq.onsuccess = () => resolve(getReq.result ?? null);
          getReq.onerror = () => resolve(null);
        };
        req.onerror = () => resolve(null);
      });
    } catch {
      return null;
    }
  }

  private async cacheSet(key: string, data: unknown): Promise<void> {
    try {
      if (typeof indexedDB === "undefined") return;
      return new Promise((resolve) => {
        const req = indexedDB.open("gane_osm_cache", 1);
        req.onupgradeneeded = () => {
          const db = req.result;
          if (!db.objectStoreNames.contains("tiles")) {
            db.createObjectStore("tiles");
          }
        };
        req.onsuccess = () => {
          const tx = req.result.transaction("tiles", "readwrite");
          const store = tx.objectStore("tiles");
          store.put(data, key);
          tx.oncomplete = () => resolve();
          tx.onerror = () => resolve();
        };
        req.onerror = () => resolve();
      });
    } catch {
      // Silently fail cache writes
    }
  }

  // ─── Load OSM Data ───

  async loadArea(
    centerLat: number,
    centerLon: number,
    radiusM: number = 5000,
    profile: string = "driving"
  ): Promise<{ nodeCount: number; edgeCount: number }> {
    if (this.loading) return { nodeCount: this.nodes.size, edgeCount: 0 };

    // Skip if already loaded for this area
    if (this.loaded && this.lastCenter) {
      const dist = haversine(centerLat, centerLon, this.lastCenter.lat, this.lastCenter.lon);
      if (dist < radiusM * 0.3 && this.lastRadius >= radiusM) {
        return { nodeCount: this.nodes.size, edgeCount: this.countEdges() };
      }
    }

    this.loading = true;

    try {
      const cacheKey = `osm_${centerLat.toFixed(3)}_${centerLon.toFixed(3)}_${radiusM}_${profile}`;

      // Try cache first
      const cached = await this.cacheGet(cacheKey) as { nodes: OSMNode[]; ways: OSMWay[] } | null;
      if (cached) {
        this.buildGraph(cached.nodes, cached.ways, profile);
        this.lastCenter = { lat: centerLat, lon: centerLon };
        this.lastRadius = radiusM;
        this.loaded = true;
        this.loading = false;
        return { nodeCount: this.nodes.size, edgeCount: this.countEdges() };
      }

      // Fetch from Overpass API
      const query = buildOverpassQuery(centerLat, centerLon, radiusM, profile);
      const response = await fetch(OVERPASS_API, {
        method: "POST",
        headers: { "Content-Type": "application/x-www-form-urlencoded" },
        body: `data=${encodeURIComponent(query)}`,
      });

      if (!response.ok) {
        throw new Error(`Overpass API error: ${response.status}`);
      }

      const data = await response.json();
      const osmNodes: OSMNode[] = [];
      const osmWays: OSMWay[] = [];

      for (const element of data.elements) {
        if (element.type === "node") {
          osmNodes.push({ id: element.id, lat: element.lat, lon: element.lon });
        } else if (element.type === "way") {
          osmWays.push({ id: element.id, nodes: element.nodes, tags: element.tags || {} });
        }
      }

      // Cache the raw data
      await this.cacheSet(cacheKey, { nodes: osmNodes, ways: osmWays });

      // Build graph
      this.buildGraph(osmNodes, osmWays, profile);
      this.lastCenter = { lat: centerLat, lon: centerLon };
      this.lastRadius = radiusM;
      this.loaded = true;
      this.loading = false;

      return { nodeCount: this.nodes.size, edgeCount: this.countEdges() };
    } catch (err) {
      this.loading = false;
      console.warn("[OSM] Failed to load area:", err);
      return { nodeCount: this.nodes.size, edgeCount: this.countEdges() };
    }
  }

  // ─── Build Graph from OSM Data ───

  private buildGraph(osmNodes: OSMNode[], osmWays: OSMWay[], profile: string): void {
    this.nodes.clear();
    this.ways.clear();

    // Index nodes
    const nodeMap = new Map<number, OSMNode>();
    for (const node of osmNodes) {
      nodeMap.set(node.id, node);
    }

    // Create graph nodes for all nodes referenced by ways
    for (const way of osmWays) {
      this.ways.set(way.id, way);
      for (const nodeId of way.nodes) {
        const osmNode = nodeMap.get(nodeId);
        if (osmNode && !this.nodes.has(nodeId)) {
          this.nodes.set(nodeId, {
            id: nodeId,
            lat: osmNode.lat,
            lon: osmNode.lon,
            edges: [],
          });
        }
      }
    }

    // Build edges from ways
    const speeds = SPEED_PROFILES[profile] || SPEED_PROFILES.driving;

    for (const way of osmWays) {
      const highway = way.tags.highway || "road";
      const speed = speeds[highway] || 30;
      if (speed === 0) continue; // Can't use this road type

      const isOneway = way.tags.oneway === "yes" || way.tags.oneway === "1" ||
        highway === "motorway" || highway === "motorway_link";
      const roadName = way.tags.name || way.tags.ref || highway;

      for (let i = 0; i < way.nodes.length - 1; i++) {
        const fromId = way.nodes[i];
        const toId = way.nodes[i + 1];
        const fromNode = this.nodes.get(fromId);
        const toNode = this.nodes.get(toId);
        if (!fromNode || !toNode) continue;

        const dist = haversine(fromNode.lat, fromNode.lon, toNode.lat, toNode.lon);
        const duration = dist / (speed / 3.6); // speed in m/s

        // Forward edge
        fromNode.edges.push({
          targetId: toId,
          wayId: way.id,
          distanceM: dist,
          durationS: duration,
          speedKmh: speed,
          roadName,
          roadType: highway,
          oneway: isOneway,
        });

        // Reverse edge (if not oneway)
        if (!isOneway) {
          toNode.edges.push({
            targetId: fromId,
            wayId: way.id,
            distanceM: dist,
            durationS: duration,
            speedKmh: speed,
            roadName,
            roadType: highway,
            oneway: false,
          });
        }
      }
    }
  }

  // ─── A* Pathfinding ───

  findRoute(
    originLat: number,
    originLon: number,
    destLat: number,
    destLon: number
  ): OSMRouteResult | null {
    if (!this.loaded || this.nodes.size === 0) return null;

    // Find nearest graph nodes to origin and destination
    const startNode = this.findNearestNode(originLat, originLon);
    const endNode = this.findNearestNode(destLat, destLon);
    if (!startNode || !endNode) return null;
    if (startNode.id === endNode.id) return null;

    // A* search
    const openSet = new Map<number, { f: number; g: number; parent: number | null }>();
    const closedSet = new Set<number>();

    openSet.set(startNode.id, {
      f: haversine(startNode.lat, startNode.lon, endNode.lat, endNode.lon),
      g: 0,
      parent: null,
    });

    const cameFrom = new Map<number, { parent: number; edge: GraphEdge }>();
    const gScore = new Map<number, number>();
    gScore.set(startNode.id, 0);

    let iterations = 0;
    const maxIterations = 50000;

    while (openSet.size > 0 && iterations < maxIterations) {
      iterations++;

      // Find node with lowest f score
      let currentId = -1;
      let lowestF = Infinity;
      openSet.forEach((data, id) => {
        if (data.f < lowestF) {
          lowestF = data.f;
          currentId = id;
        }
      });

      if (currentId === endNode.id) {
        // Reconstruct path
        return this.reconstructPath(currentId, cameFrom, startNode, endNode);
      }

      openSet.delete(currentId);
      closedSet.add(currentId);

      const currentNode = this.nodes.get(currentId);
      if (!currentNode) continue;

      for (const edge of currentNode.edges) {
        if (closedSet.has(edge.targetId)) continue;

        const tentativeG = (gScore.get(currentId) || 0) + edge.durationS;
        const existingG = gScore.get(edge.targetId) ?? Infinity;

        if (tentativeG < existingG) {
          const targetNode = this.nodes.get(edge.targetId);
          if (!targetNode) continue;

          gScore.set(edge.targetId, tentativeG);
          cameFrom.set(edge.targetId, { parent: currentId, edge });

          const h = haversine(targetNode.lat, targetNode.lon, endNode.lat, endNode.lon) / 30; // Heuristic: ~30 m/s
          openSet.set(edge.targetId, {
            f: tentativeG + h,
            g: tentativeG,
            parent: currentId,
          });
        }
      }
    }

    return null; // No route found
  }

  // ─── Path Reconstruction ───

  private reconstructPath(
    endId: number,
    cameFrom: Map<number, { parent: number; edge: GraphEdge }>,
    startNode: GraphNode,
    endNode: GraphNode
  ): OSMRouteResult {
    const path: number[] = [endId];
    const edges: GraphEdge[] = [];
    let current = endId;

    while (cameFrom.has(current)) {
      const { parent, edge } = cameFrom.get(current)!;
      path.unshift(parent);
      edges.unshift(edge);
      current = parent;
    }

    // Build geometry
    const geometry: { lat: number; lon: number }[] = [];
    let totalDistance = 0;
    let totalDuration = 0;

    for (const nodeId of path) {
      const node = this.nodes.get(nodeId);
      if (node) {
        geometry.push({ lat: node.lat, lon: node.lon });
      }
    }

    for (const edge of edges) {
      totalDistance += edge.distanceM;
      totalDuration += edge.durationS;
    }

    // Generate turn-by-turn instructions
    const instructions = this.generateInstructions(path, edges);

    // Collect nodes
    const routeNodes = path.map((id) => this.nodes.get(id)!).filter(Boolean);

    return {
      nodes: routeNodes,
      totalDistanceM: totalDistance,
      totalDurationS: totalDuration,
      geometry,
      instructions,
    };
  }

  // ─── Turn-by-Turn Instructions ───

  private generateInstructions(
    path: number[],
    edges: GraphEdge[]
  ): RouteInstruction[] {
    const instructions: RouteInstruction[] = [];
    if (edges.length === 0) return instructions;

    let currentRoad = edges[0].roadName;
    let segmentDistance = 0;
    let segmentDuration = 0;
    let segmentGeometry: { lat: number; lon: number }[] = [];
    const startNode = this.nodes.get(path[0]);
    if (startNode) segmentGeometry.push({ lat: startNode.lat, lon: startNode.lon });

    // Depart instruction
    instructions.push({
      text: `Depart on ${currentRoad}`,
      distanceM: 0,
      durationS: 0,
      maneuver: "depart",
      roadName: currentRoad,
      geometry: startNode ? [{ lat: startNode.lat, lon: startNode.lon }] : [],
    });

    for (let i = 0; i < edges.length; i++) {
      const edge = edges[i];
      const targetNode = this.nodes.get(edge.targetId);
      if (targetNode) segmentGeometry.push({ lat: targetNode.lat, lon: targetNode.lon });

      segmentDistance += edge.distanceM;
      segmentDuration += edge.durationS;

      // Check if road changes
      if (i < edges.length - 1 && edge.roadName !== edges[i + 1].roadName) {
        // Calculate turn direction
        const fromNode = this.nodes.get(path[i]);
        const midNode = this.nodes.get(path[i + 1]);
        const toNode = this.nodes.get(path[i + 2]);

        let maneuver = "continue";
        if (fromNode && midNode && toNode) {
          const fromBearing = bearing(fromNode.lat, fromNode.lon, midNode.lat, midNode.lon);
          const toBearing = bearing(midNode.lat, midNode.lon, toNode.lat, toNode.lon);
          maneuver = turnDirection(fromBearing, toBearing);
        }

        const nextRoad = edges[i + 1].roadName;
        const turnText = maneuver === "straight"
          ? `Continue onto ${nextRoad}`
          : `Turn ${maneuver.replace("-", " ")} onto ${nextRoad}`;

        instructions.push({
          text: turnText,
          distanceM: segmentDistance,
          durationS: segmentDuration,
          maneuver,
          roadName: currentRoad,
          geometry: [...segmentGeometry],
        });

        currentRoad = nextRoad;
        segmentDistance = 0;
        segmentDuration = 0;
        segmentGeometry = targetNode ? [{ lat: targetNode.lat, lon: targetNode.lon }] : [];
      }
    }

    // Final segment
    if (segmentDistance > 0) {
      instructions.push({
        text: `Continue on ${currentRoad}`,
        distanceM: segmentDistance,
        durationS: segmentDuration,
        maneuver: "continue",
        roadName: currentRoad,
        geometry: segmentGeometry,
      });
    }

    // Arrive instruction
    const lastNode = this.nodes.get(path[path.length - 1]);
    instructions.push({
      text: "Arrive at destination",
      distanceM: 0,
      durationS: 0,
      maneuver: "arrive",
      roadName: currentRoad,
      geometry: lastNode ? [{ lat: lastNode.lat, lon: lastNode.lon }] : [],
    });

    return instructions;
  }

  // ─── Find Nearest Node ───

  private findNearestNode(lat: number, lon: number): GraphNode | null {
    let nearest: GraphNode | null = null;
    let minDist = Infinity;

    this.nodes.forEach((node) => {
      if (node.edges.length === 0) return; // Skip isolated nodes
      const dist = haversine(lat, lon, node.lat, node.lon);
      if (dist < minDist) {
        minDist = dist;
        nearest = node;
      }
    });

    return nearest;
  }

  // ─── Stats ───

  private countEdges(): number {
    let count = 0;
    this.nodes.forEach((node) => {
      count += node.edges.length;
    });
    return count;
  }

  getStats(): { nodeCount: number; edgeCount: number; loaded: boolean; loading: boolean } {
    return {
      nodeCount: this.nodes.size,
      edgeCount: this.countEdges(),
      loaded: this.loaded,
      loading: this.loading,
    };
  }

  isLoaded(): boolean {
    return this.loaded;
  }
}

// ─── Singleton ───

let _osmRouter: OSMRoutingGraph | null = null;

export function getOSMRouter(): OSMRoutingGraph {
  if (!_osmRouter) {
    _osmRouter = new OSMRoutingGraph();
  }
  return _osmRouter;
}
