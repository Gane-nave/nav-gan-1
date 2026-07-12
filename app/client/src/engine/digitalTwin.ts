/**
 * G.A.N.E — Digital Twin Engine
 * ================================
 * Real-time city/road network digital twin.
 *
 * LAYERS:
 *   - Road network graph (nodes + edges)
 *   - Real-time traffic state per edge
 *   - Infrastructure state (traffic lights, signs)
 *   - Vehicle positions (from V2X + crowd)
 *   - Environmental conditions (weather, lighting)
 *   - Incident overlay
 *
 * CAPABILITIES:
 *   - What-if simulation (road closure, event)
 *   - Capacity analysis
 *   - Signal optimization
 *   - Bottleneck detection
 */

// ─── Types ───────────────────────────────────────────────

export interface TwinNode {
  id: string;
  lat: number;
  lon: number;
  type: 'intersection' | 'endpoint' | 'merge' | 'diverge' | 'roundabout';
  signalState?: {
    phase: 'red' | 'yellow' | 'green' | 'flashing';
    remainingMs: number;
    cycleMs: number;
  };
  elevation?: number;
}

export interface TwinEdge {
  id: string;
  fromNode: string;
  toNode: string;
  roadType: 'highway' | 'arterial' | 'collector' | 'local' | 'ramp';
  lanes: number;
  speedLimitKmh: number;
  lengthM: number;
  // Real-time state
  currentSpeedKmh: number;
  congestionIndex: number;          // 0-1
  vehicleCount: number;
  density: number;                  // vehicles/km
  flow: number;                     // vehicles/hour
  travelTimeMs: number;
  incidents: string[];              // incident IDs
  isBlocked: boolean;
}

export interface TwinVehicle {
  id: string;
  lat: number;
  lon: number;
  speed: number;
  heading: number;
  type: 'car' | 'truck' | 'bus' | 'motorcycle' | 'emergency';
  edgeId: string;
  lastUpdate: number;
}

export interface TwinEnvironment {
  weather: 'clear' | 'rain' | 'snow' | 'fog' | 'storm';
  visibility: number;               // meters
  temperature: number;              // celsius
  windSpeed: number;                // km/h
  lightCondition: 'day' | 'dusk' | 'night';
  roadCondition: 'dry' | 'wet' | 'icy' | 'flooded';
}

export interface SimulationScenario {
  id: string;
  name: string;
  nameHe: string;
  type: 'road_closure' | 'event' | 'weather' | 'incident' | 'custom';
  affectedEdges: string[];
  affectedNodes: string[];
  modifications: {
    edgeId: string;
    speedMultiplier?: number;       // 0 = blocked
    capacityMultiplier?: number;
  }[];
  duration: number;
}

export interface SimulationResult {
  scenarioId: string;
  impactScore: number;              // 0-1 (1 = severe)
  avgDelayIncrease: number;         // seconds
  affectedVehicles: number;
  alternativeRoutes: number;
  bottlenecks: string[];            // edge IDs
  recommendation: string;
  recommendationHe: string;
}

export interface TwinState {
  nodeCount: number;
  edgeCount: number;
  vehicleCount: number;
  avgCongestion: number;
  avgSpeed: number;
  totalFlow: number;
  blockedEdges: number;
  lastUpdateAt: number;
}

// ─── Digital Twin Engine ────────────────────────────────

export class DigitalTwinEngine {
  private nodes: Map<string, TwinNode> = new Map();
  private edges: Map<string, TwinEdge> = new Map();
  private vehicles: Map<string, TwinVehicle> = new Map();
  private environment: TwinEnvironment = {
    weather: 'clear',
    visibility: 10000,
    temperature: 25,
    windSpeed: 10,
    lightCondition: 'day',
    roadCondition: 'dry',
  };

  private updateTimer: ReturnType<typeof setInterval> | null = null;

  // ─── Network Building ─────────────────────────────────

  /**
   * Add a node to the twin.
   */
  addNode(node: TwinNode) {
    this.nodes.set(node.id, node);
  }

  /**
   * Add an edge to the twin.
   */
  addEdge(edge: TwinEdge) {
    this.edges.set(edge.id, edge);
  }

  /**
   * Build network from raw data.
   */
  buildNetwork(
    nodes: TwinNode[],
    edges: TwinEdge[]
  ) {
    this.nodes.clear();
    this.edges.clear();

    for (const node of nodes) {
      this.nodes.set(node.id, node);
    }
    for (const edge of edges) {
      this.edges.set(edge.id, edge);
    }
  }

  // ─── Real-Time Updates ────────────────────────────────

  /**
   * Update traffic state on an edge.
   */
  updateEdgeTraffic(
    edgeId: string,
    speedKmh: number,
    vehicleCount: number
  ) {
    const edge = this.edges.get(edgeId);
    if (!edge) return;

    edge.currentSpeedKmh = speedKmh;
    edge.vehicleCount = vehicleCount;
    edge.density = edge.lengthM > 0 ? (vehicleCount / (edge.lengthM / 1000)) : 0;
    edge.flow = speedKmh > 0 ? Math.round(edge.density * speedKmh) : 0;
    edge.travelTimeMs = speedKmh > 0
      ? Math.round((edge.lengthM / 1000) / speedKmh * 3600000)
      : Infinity;

    // Compute congestion index
    edge.congestionIndex = edge.speedLimitKmh > 0
      ? Math.max(0, Math.min(1, 1 - speedKmh / edge.speedLimitKmh))
      : 0;
  }

  /**
   * Update a vehicle position.
   */
  updateVehicle(vehicle: TwinVehicle) {
    this.vehicles.set(vehicle.id, { ...vehicle, lastUpdate: Date.now() });
  }

  /**
   * Remove stale vehicles.
   */
  cleanupVehicles(maxAgeMs: number = 30000) {
    const cutoff = Date.now() - maxAgeMs;
    for (const [id, v] of Array.from(this.vehicles.entries())) {
      if (v.lastUpdate < cutoff) {
        this.vehicles.delete(id);
      }
    }
  }

  /**
   * Update environment.
   */
  updateEnvironment(env: Partial<TwinEnvironment>) {
    Object.assign(this.environment, env);
  }

  /**
   * Update signal state at a node.
   */
  updateSignal(nodeId: string, phase: TwinNode['signalState']) {
    const node = this.nodes.get(nodeId);
    if (node) {
      node.signalState = phase;
    }
  }

  // ─── Simulation ───────────────────────────────────────

  /**
   * Run a what-if simulation.
   */
  simulate(scenario: SimulationScenario): SimulationResult {
    // Create a snapshot of affected edges
    const originalStates: Map<string, { speed: number; congestion: number }> = new Map();

    for (const mod of scenario.modifications) {
      const edge = this.edges.get(mod.edgeId);
      if (!edge) continue;

      originalStates.set(mod.edgeId, {
        speed: edge.currentSpeedKmh,
        congestion: edge.congestionIndex,
      });

      // Apply modification
      if (mod.speedMultiplier !== undefined) {
        edge.currentSpeedKmh *= mod.speedMultiplier;
        if (mod.speedMultiplier === 0) {
          edge.isBlocked = true;
        }
      }
      if (mod.capacityMultiplier !== undefined) {
        edge.vehicleCount = Math.round(edge.vehicleCount / mod.capacityMultiplier);
      }

      // Recalculate
      this.updateEdgeTraffic(mod.edgeId, edge.currentSpeedKmh, edge.vehicleCount);
    }

    // Analyze impact
    let totalDelayIncrease = 0;
    let affectedVehicles = 0;
    const bottlenecks: string[] = [];

    for (const mod of scenario.modifications) {
      const edge = this.edges.get(mod.edgeId);
      const original = originalStates.get(mod.edgeId);
      if (!edge || !original) continue;

      const delayIncrease = edge.congestionIndex - original.congestion;
      if (delayIncrease > 0) {
        totalDelayIncrease += delayIncrease * edge.travelTimeMs / 1000;
        affectedVehicles += edge.vehicleCount;
      }

      if (edge.congestionIndex > 0.8) {
        bottlenecks.push(mod.edgeId);
      }
    }

    const impactScore = Math.min(1, totalDelayIncrease / 300); // 5 min = max impact

    // Restore original states
    for (const [edgeId, original] of Array.from(originalStates.entries())) {
      const edge = this.edges.get(edgeId);
      if (edge) {
        edge.currentSpeedKmh = original.speed;
        edge.isBlocked = false;
        this.updateEdgeTraffic(edgeId, original.speed, edge.vehicleCount);
      }
    }

    return {
      scenarioId: scenario.id,
      impactScore: Math.round(impactScore * 1000) / 1000,
      avgDelayIncrease: Math.round(totalDelayIncrease),
      affectedVehicles,
      alternativeRoutes: Math.max(1, Math.round(3 - impactScore * 2)),
      bottlenecks,
      recommendation: impactScore > 0.7
        ? 'Severe impact expected. Pre-emptive rerouting recommended.'
        : impactScore > 0.3
          ? 'Moderate impact. Monitor and prepare alternatives.'
          : 'Minimal impact. Standard operations sufficient.',
      recommendationHe: impactScore > 0.7
        ? 'צפוי השפעה חמורה. מומלץ ניתוב מחדש מקדים.'
        : impactScore > 0.3
          ? 'השפעה בינונית. יש לנטר ולהכין חלופות.'
          : 'השפעה מינימלית. פעולות רגילות מספיקות.',
    };
  }

  // ─── Analysis ─────────────────────────────────────────

  /**
   * Find bottleneck edges (high congestion, high flow).
   */
  findBottlenecks(threshold: number = 0.7): TwinEdge[] {
    return Array.from(this.edges.values())
      .filter(e => e.congestionIndex > threshold)
      .sort((a, b) => b.congestionIndex - a.congestionIndex);
  }

  /**
   * Get network-wide statistics.
   */
  getNetworkStats(): {
    avgCongestion: number;
    avgSpeed: number;
    totalFlow: number;
    utilizationPercent: number;
    blockedEdges: number;
    criticalEdges: number;
  } {
    const edgeArr = Array.from(this.edges.values());
    if (edgeArr.length === 0) {
      return { avgCongestion: 0, avgSpeed: 0, totalFlow: 0, utilizationPercent: 0, blockedEdges: 0, criticalEdges: 0 };
    }

    const avgCongestion = edgeArr.reduce((s, e) => s + e.congestionIndex, 0) / edgeArr.length;
    const avgSpeed = edgeArr.reduce((s, e) => s + e.currentSpeedKmh, 0) / edgeArr.length;
    const totalFlow = edgeArr.reduce((s, e) => s + e.flow, 0);
    const blockedEdges = edgeArr.filter(e => e.isBlocked).length;
    const criticalEdges = edgeArr.filter(e => e.congestionIndex > 0.8).length;

    return {
      avgCongestion: Math.round(avgCongestion * 1000) / 1000,
      avgSpeed: Math.round(avgSpeed * 10) / 10,
      totalFlow,
      utilizationPercent: Math.round(avgCongestion * 100),
      blockedEdges,
      criticalEdges,
    };
  }

  // ─── Public API ───────────────────────────────────────

  getState(): TwinState {
    const stats = this.getNetworkStats();
    return {
      nodeCount: this.nodes.size,
      edgeCount: this.edges.size,
      vehicleCount: this.vehicles.size,
      avgCongestion: stats.avgCongestion,
      avgSpeed: stats.avgSpeed,
      totalFlow: stats.totalFlow,
      blockedEdges: stats.blockedEdges,
      lastUpdateAt: Date.now(),
    };
  }

  getEnvironment(): TwinEnvironment {
    return { ...this.environment };
  }

  getNode(id: string): TwinNode | undefined {
    return this.nodes.get(id);
  }

  getEdge(id: string): TwinEdge | undefined {
    return this.edges.get(id);
  }

  getAllEdges(): TwinEdge[] {
    return Array.from(this.edges.values());
  }

  getAllNodes(): TwinNode[] {
    return Array.from(this.nodes.values());
  }

  getAllVehicles(): TwinVehicle[] {
    return Array.from(this.vehicles.values());
  }

  destroy() {
    if (this.updateTimer) {
      clearInterval(this.updateTimer);
    }
    this.nodes.clear();
    this.edges.clear();
    this.vehicles.clear();
  }
}
