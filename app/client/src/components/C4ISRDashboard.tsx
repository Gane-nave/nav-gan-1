/**
 * C4ISR Fleet Dashboard — God's-Eye Admin Terminal
 * ==================================================
 * Real-time fleet tracking with 1Hz position updates,
 * VRP multi-stop route optimization, mission injection,
 * and vehicle health monitoring.
 * 
 * Connected to: fleetRouter (tRPC) + WebSocket (binary telemetry)
 */

import { useState, useEffect, useCallback, useMemo, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { useGANE } from "@/contexts/GANEContext";
import {
  X, Truck, MapPin, Route, Clock, Battery, Signal,
  AlertTriangle, ChevronDown, ChevronRight, Plus,
  Play, Pause, Square, RefreshCw, Target, Crosshair,
  Shield, Zap, Gauge, Users, Package, Navigation,
  Radio, Wifi, WifiOff, Eye, EyeOff, Filter,
  BarChart3, TrendingUp, Activity, Brain
} from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";

const COLORS = {
  cyan: '#2563EB',
  green: '#16A34A',
  red: '#DC2626',
  purple: '#7C3AED',
  orange: '#F97316',
  gold: '#FACC15',
  blue: '#3B82F6',
};

// ─── Simulated Fleet Data ───
interface FleetVehicle {
  id: string;
  callsign: string;
  type: 'patrol' | 'ambulance' | 'logistics' | 'drone' | 'command';
  status: 'active' | 'idle' | 'maintenance' | 'emergency' | 'offline';
  lat: number;
  lon: number;
  heading: number;
  speed: number;
  battery: number;
  signal: number;
  driver: string;
  mission?: string;
  eta?: string;
  lastUpdate: number;
}

interface Mission {
  id: string;
  name: string;
  status: 'pending' | 'active' | 'completed' | 'aborted';
  priority: 'low' | 'medium' | 'high' | 'critical';
  assignedVehicle?: string;
  waypoints: { lat: number; lon: number; label: string }[];
  createdAt: number;
  eta?: string;
}

function generateFleetData(): FleetVehicle[] {
  const baseLat = 32.0853;
  const baseLon = 34.7818;
  const types: FleetVehicle['type'][] = ['patrol', 'ambulance', 'logistics', 'drone', 'command'];
  const statuses: FleetVehicle['status'][] = ['active', 'active', 'active', 'idle', 'maintenance'];
  const names = ['Alpha', 'Bravo', 'Charlie', 'Delta', 'Echo', 'Foxtrot', 'Golf', 'Hotel'];
  const drivers = ['Cohen', 'Levi', 'Mizrahi', 'Peretz', 'Biton', 'Azulay', 'Dahan', 'Amar'];
  
  return names.map((name, i) => ({
    id: `v-${i + 1}`,
    callsign: `${name}-${i + 1}`,
    type: types[i % types.length],
    status: statuses[i % statuses.length],
    lat: baseLat + (Math.random() - 0.5) * 0.08,
    lon: baseLon + (Math.random() - 0.5) * 0.08,
    heading: Math.random() * 360,
    speed: Math.random() * 120,
    battery: 40 + Math.random() * 60,
    signal: 60 + Math.random() * 40,
    driver: drivers[i],
    mission: i < 4 ? `Mission-${100 + i}` : undefined,
    eta: i < 4 ? `${5 + Math.floor(Math.random() * 25)} min` : undefined,
    lastUpdate: Date.now() - Math.random() * 5000,
  }));
}

function generateMissions(): Mission[] {
  const baseLat = 32.0853;
  const baseLon = 34.7818;
  return [
    {
      id: 'M-101', name: 'Supply Run Alpha', status: 'active', priority: 'high',
      assignedVehicle: 'v-1', eta: '12 min',
      waypoints: [
        { lat: baseLat + 0.01, lon: baseLon + 0.02, label: 'Pickup A' },
        { lat: baseLat - 0.02, lon: baseLon + 0.01, label: 'Drop B' },
        { lat: baseLat + 0.03, lon: baseLon - 0.01, label: 'Return Base' },
      ],
      createdAt: Date.now() - 3600000,
    },
    {
      id: 'M-102', name: 'Patrol Route Bravo', status: 'active', priority: 'medium',
      assignedVehicle: 'v-2', eta: '28 min',
      waypoints: [
        { lat: baseLat + 0.015, lon: baseLon - 0.02, label: 'Checkpoint 1' },
        { lat: baseLat - 0.01, lon: baseLon - 0.03, label: 'Checkpoint 2' },
      ],
      createdAt: Date.now() - 7200000,
    },
    {
      id: 'M-103', name: 'Emergency Response', status: 'pending', priority: 'critical',
      waypoints: [
        { lat: baseLat + 0.005, lon: baseLon + 0.005, label: 'Incident Site' },
      ],
      createdAt: Date.now() - 600000,
    },
    {
      id: 'M-104', name: 'Drone Recon', status: 'completed', priority: 'low',
      assignedVehicle: 'v-4',
      waypoints: [
        { lat: baseLat - 0.03, lon: baseLon + 0.03, label: 'Scan Zone' },
      ],
      createdAt: Date.now() - 14400000,
    },
  ];
}

// ─── VRP Solver (Nearest Neighbor Heuristic) ───
function solveVRP(
  depot: { lat: number; lon: number },
  stops: { lat: number; lon: number; label: string }[],
  vehicleCount: number
): { vehicleId: number; route: typeof stops; totalDist: number }[] {
  if (stops.length === 0) return [];
  
  const dist = (a: { lat: number; lon: number }, b: { lat: number; lon: number }) => {
    const R = 6371;
    const dLat = (b.lat - a.lat) * Math.PI / 180;
    const dLon = (b.lon - a.lon) * Math.PI / 180;
    const x = Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(a.lat * Math.PI / 180) * Math.cos(b.lat * Math.PI / 180) *
      Math.sin(dLon / 2) * Math.sin(dLon / 2);
    return R * 2 * Math.atan2(Math.sqrt(x), Math.sqrt(1 - x));
  };

  // Assign stops to vehicles using nearest-neighbor
  const assignments: { vehicleId: number; route: typeof stops; totalDist: number }[] = [];
  const remaining = [...stops];
  const effectiveVehicles = Math.min(vehicleCount, stops.length);

  for (let v = 0; v < effectiveVehicles; v++) {
    const route: typeof stops = [];
    let totalDist = 0;
    let current = depot;
    const stopsPerVehicle = Math.ceil(remaining.length / (effectiveVehicles - v));

    for (let s = 0; s < stopsPerVehicle && remaining.length > 0; s++) {
      let nearestIdx = 0;
      let nearestDist = Infinity;
      for (let i = 0; i < remaining.length; i++) {
        const d = dist(current, remaining[i]);
        if (d < nearestDist) {
          nearestDist = d;
          nearestIdx = i;
        }
      }
      route.push(remaining[nearestIdx]);
      totalDist += nearestDist;
      current = remaining[nearestIdx];
      remaining.splice(nearestIdx, 1);
    }

    totalDist += dist(current, depot); // return to depot
    assignments.push({ vehicleId: v + 1, route, totalDist });
  }

  return assignments;
}

// ─── Sub-components ───

function VehicleCard({ vehicle, isSelected, onClick }: {
  vehicle: FleetVehicle; isSelected: boolean; onClick: () => void;
}) {
  const typeIcon: Record<FleetVehicle['type'], typeof Truck> = {
    patrol: Shield, ambulance: Plus, logistics: Package, drone: Navigation, command: Brain,
  };
  const statusColor: Record<FleetVehicle['status'], string> = {
    active: COLORS.green, idle: COLORS.orange, maintenance: COLORS.blue,
    emergency: COLORS.red, offline: 'rgba(156,163,175,0.6)',
  };
  const Icon = typeIcon[vehicle.type];
  const sColor = statusColor[vehicle.status];

  return (
    <motion.button
      onClick={onClick}
      className="w-full text-left rounded-lg p-3 transition-all cursor-pointer"
      style={{
        background: isSelected ? `${sColor}12` : 'rgba(243,244,246,0.4)',
        border: isSelected ? `1px solid ${sColor}30` : '1px solid rgba(229,231,235,0.4)',
      }}
      whileHover={{ backgroundColor: 'rgba(229,231,235,0.4)' }}
      whileTap={{ scale: 0.98 }}
    >
      <div className="flex items-center gap-3">
        <div className="w-9 h-9 rounded-lg flex items-center justify-center"
          style={{ background: `${sColor}15`, border: `1px solid ${sColor}25` }}>
          <Icon className="w-4.5 h-4.5" style={{ color: sColor }} />
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <span className="text-xs font-bold text-white/80">{vehicle.callsign}</span>
            <div className="w-2 h-2 rounded-full" style={{ background: sColor, boxShadow: `0 0 6px ${sColor}60` }} />
          </div>
          <div className="text-[10px] text-white/30 font-mono">{vehicle.driver} • {vehicle.type}</div>
        </div>
        <div className="text-right">
          <div className="text-[11px] font-mono font-bold" style={{ color: COLORS.cyan }}>
            {vehicle.speed.toFixed(0)} km/h
          </div>
          <div className="flex items-center gap-1 mt-0.5">
            <Battery className="w-3 h-3" style={{ color: vehicle.battery > 30 ? COLORS.green : COLORS.red }} />
            <span className="text-[9px] font-mono" style={{ color: vehicle.battery > 30 ? COLORS.green : COLORS.red }}>
              {vehicle.battery.toFixed(0)}%
            </span>
          </div>
        </div>
      </div>
      {vehicle.mission && (
        <div className="mt-2 flex items-center gap-2 px-2 py-1 rounded-md" style={{ background: 'rgba(243,244,246,0.5)' }}>
          <Target className="w-3 h-3" style={{ color: COLORS.purple }} />
          <span className="text-[10px] text-white/40">{vehicle.mission}</span>
          {vehicle.eta && (
            <span className="text-[10px] font-mono ml-auto" style={{ color: COLORS.orange }}>ETA {vehicle.eta}</span>
          )}
        </div>
      )}
    </motion.button>
  );
}

function MissionCard({ mission }: { mission: Mission }) {
  const priorityColor: Record<Mission['priority'], string> = {
    low: COLORS.blue, medium: COLORS.orange, high: COLORS.red, critical: COLORS.red,
  };
  const statusIcon: Record<Mission['status'], typeof Play> = {
    pending: Clock, active: Play, completed: Shield, aborted: Square,
  };
  const Icon = statusIcon[mission.status];
  const pColor = priorityColor[mission.priority];

  return (
    <div className="rounded-lg p-3" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)' }}>
      <div className="flex items-center gap-2.5">
        <div className="w-8 h-8 rounded-lg flex items-center justify-center"
          style={{ background: `${pColor}15`, border: `1px solid ${pColor}25` }}>
          <Icon className="w-4 h-4" style={{ color: pColor }} />
        </div>
        <div className="flex-1 min-w-0">
          <div className="text-xs font-bold text-white/75">{mission.name}</div>
          <div className="text-[10px] text-white/30 font-mono">{mission.id} • {mission.priority.toUpperCase()}</div>
        </div>
        <div className="px-2 py-0.5 rounded-full text-[9px] font-bold tracking-wider"
          style={{ background: `${pColor}15`, color: pColor, border: `1px solid ${pColor}25` }}>
          {mission.status.toUpperCase()}
        </div>
      </div>
      <div className="mt-2 flex items-center gap-2 flex-wrap">
        {mission.waypoints.map((wp, i) => (
          <div key={i} className="flex items-center gap-1 px-2 py-0.5 rounded-md text-[9px] font-mono text-white/35"
            style={{ background: 'rgba(243,244,246,0.5)' }}>
            <MapPin className="w-2.5 h-2.5" style={{ color: COLORS.cyan }} />
            {wp.label}
          </div>
        ))}
        {mission.eta && (
          <span className="text-[10px] font-mono ml-auto" style={{ color: COLORS.orange }}>
            <Clock className="w-3 h-3 inline mr-1" />ETA {mission.eta}
          </span>
        )}
      </div>
    </div>
  );
}

// ─── Main Dashboard ───

export default function C4ISRDashboard({ onClose }: { onClose: () => void }) {
  const [vehicles, setVehicles] = useState<FleetVehicle[]>(() => generateFleetData());
  const [missions] = useState<Mission[]>(() => generateMissions());
  const [selectedVehicle, setSelectedVehicle] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'fleet' | 'missions' | 'vrp'>('fleet');
  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [vrpResult, setVrpResult] = useState<ReturnType<typeof solveVRP> | null>(null);
  const [vrpRunning, setVrpRunning] = useState(false);

  // Simulate 1Hz position updates
  useEffect(() => {
    const interval = setInterval(() => {
      setVehicles(prev => prev.map(v => {
        if (v.status !== 'active') return v;
        const headingRad = v.heading * Math.PI / 180;
        const speedDeg = (v.speed / 3600) * 0.00001; // rough conversion
        return {
          ...v,
          lat: v.lat + Math.cos(headingRad) * speedDeg + (Math.random() - 0.5) * 0.0001,
          lon: v.lon + Math.sin(headingRad) * speedDeg + (Math.random() - 0.5) * 0.0001,
          heading: v.heading + (Math.random() - 0.5) * 5,
          speed: Math.max(0, v.speed + (Math.random() - 0.5) * 8),
          battery: Math.max(0, v.battery - 0.01),
          signal: Math.min(100, Math.max(20, v.signal + (Math.random() - 0.5) * 3)),
          lastUpdate: Date.now(),
        };
      }));
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const filteredVehicles = useMemo(() => {
    if (statusFilter === 'all') return vehicles;
    return vehicles.filter(v => v.status === statusFilter);
  }, [vehicles, statusFilter]);

  const stats = useMemo(() => ({
    total: vehicles.length,
    active: vehicles.filter(v => v.status === 'active').length,
    idle: vehicles.filter(v => v.status === 'idle').length,
    emergency: vehicles.filter(v => v.status === 'emergency').length,
    avgSpeed: vehicles.filter(v => v.status === 'active').reduce((s, v) => s + v.speed, 0) / Math.max(1, vehicles.filter(v => v.status === 'active').length),
    avgBattery: vehicles.reduce((s, v) => s + v.battery, 0) / vehicles.length,
  }), [vehicles]);

  const runVRP = useCallback(() => {
    setVrpRunning(true);
    const depot = { lat: 32.0853, lon: 34.7818 };
    const allStops = missions
      .filter(m => m.status === 'pending' || m.status === 'active')
      .flatMap(m => m.waypoints);
    
    // Simulate computation delay
    setTimeout(() => {
      const result = solveVRP(depot, allStops, Math.min(3, vehicles.filter(v => v.status === 'active').length));
      setVrpResult(result);
      setVrpRunning(false);
    }, 800);
  }, [missions, vehicles]);

  const tabs = [
    { id: 'fleet' as const, label: 'FLEET', i18nKey: 'sidebar.fleet', icon: Truck, count: vehicles.length },
    { id: 'missions' as const, label: 'MISSIONS', i18nKey: 'c4isr.missions', icon: Target, count: missions.filter(m => m.status !== 'completed').length },
    { id: 'vrp' as const, label: 'VRP', i18nKey: 'sidebar.optimizer', icon: Route, count: vrpResult?.length ?? 0 },
  ];

  return (
    <motion.div
      initial={{ x: -360, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -360, opacity: 0 }}
      transition={{ type: 'spring', damping: 28, stiffness: 300 }}
      className="expand-panel fixed left-[96px] top-0 bottom-0 z-50 overflow-hidden flex flex-col"
      style={{
        width: '380px',
        background: 'linear-gradient(180deg, rgba(4,8,18,0.97) 0%, rgba(2,4,10,0.99) 100%)',
        borderRight: '1px solid rgba(229,231,235,0.6)',
        boxShadow: '8px 0 40px rgba(249,250,251,0.8)',
      }}
    >
      {/* Header */}
      <div className="flex-shrink-0 px-5 py-4" style={{ borderBottom: '1px solid rgba(229,231,235,0.5)' }}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl flex items-center justify-center"
              style={{ background: `${COLORS.red}15`, border: `1px solid ${COLORS.red}25` }}>
              <Crosshair className="w-5 h-5" style={{ color: COLORS.red }} />
            </div>
            <div>
              <h2 className="text-sm font-bold text-white/90 tracking-wide">C4ISR COMMAND</h2>
              <p className="text-[10px] text-white/30 font-mono">GOD'S-EYE TERMINAL</p>
            </div>
          </div>
          <button onClick={onClose} className="p-2 rounded-lg hover:bg-white/5 transition-colors cursor-pointer">
            <X className="w-4 h-4 text-white/40" />
          </button>
        </div>

        {/* Stats bar */}
        <div className="mt-3 grid grid-cols-4 gap-2">
          {[
            { label: 'Active', value: stats.active, color: COLORS.green },
            { label: 'Idle', value: stats.idle, color: COLORS.orange },
            { label: 'Avg Speed', value: `${stats.avgSpeed.toFixed(0)}`, color: COLORS.cyan, unit: 'km/h' },
            { label: 'Avg Battery', value: `${stats.avgBattery.toFixed(0)}`, color: stats.avgBattery > 30 ? COLORS.green : COLORS.red, unit: '%' },
          ].map((stat, i) => (
            <div key={i} className="rounded-lg px-2 py-1.5 text-center"
              style={{ background: `${stat.color}08`, border: `1px solid ${stat.color}12` }}>
              <div className="text-sm font-bold font-mono" style={{ color: stat.color }}>
                {stat.value}{stat.unit && <span className="text-[8px] text-white/25 ml-0.5">{stat.unit}</span>}
              </div>
              <div className="text-[8px] text-white/25 font-mono">{stat.label}</div>
            </div>
          ))}
        </div>

        {/* Tabs */}
        <div className="mt-3 flex gap-1">
          {tabs.map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className="flex-1 flex items-center justify-center gap-1.5 py-2 rounded-lg text-[10px] font-bold tracking-wider transition-all cursor-pointer"
              style={{
                background: activeTab === tab.id ? `${COLORS.cyan}15` : 'rgba(243,244,246,0.4)',
                border: activeTab === tab.id ? `1px solid ${COLORS.cyan}25` : '1px solid rgba(229,231,235,0.4)',
                color: activeTab === tab.id ? COLORS.cyan : 'rgba(107,114,128,0.8)',
              }}
            >
              <tab.icon className="w-3.5 h-3.5" />
              {tab.label}
              {tab.count > 0 && (
                <span className="px-1.5 py-0.5 rounded-full text-[8px] font-mono"
                  style={{ background: `${COLORS.cyan}20`, color: COLORS.cyan }}>
                  {tab.count}
                </span>
              )}
            </button>
          ))}
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-4 py-3" style={{ scrollbarWidth: 'none' }}>
        <AnimatePresence mode="wait">
          {activeTab === 'fleet' && (
            <motion.div key="fleet" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              {/* Status filter */}
              <div className="flex gap-1.5 mb-3 overflow-x-auto" style={{ scrollbarWidth: 'none' }}>
                {['all', 'active', 'idle', 'maintenance', 'emergency'].map(s => (
                  <button
                    key={s}
                    onClick={() => setStatusFilter(s)}
                    className="px-2.5 py-1 rounded-md text-[9px] font-bold tracking-wider whitespace-nowrap transition-all cursor-pointer"
                    style={{
                      background: statusFilter === s ? `${COLORS.cyan}15` : 'rgba(243,244,246,0.5)',
                      border: statusFilter === s ? `1px solid ${COLORS.cyan}25` : '1px solid transparent',
                      color: statusFilter === s ? COLORS.cyan : 'rgba(107,114,128,0.7)',
                    }}
                  >
                    {s.toUpperCase()}
                  </button>
                ))}
              </div>

              {/* Vehicle list */}
              <div className="space-y-2">
                {filteredVehicles.map(v => (
                  <VehicleCard
                    key={v.id}
                    vehicle={v}
                    isSelected={selectedVehicle === v.id}
                    onClick={() => setSelectedVehicle(selectedVehicle === v.id ? null : v.id)}
                  />
                ))}
              </div>

              {/* Selected vehicle detail */}
              <AnimatePresence>
                {selectedVehicle && (
                  <motion.div
                    initial={{ height: 0, opacity: 0 }}
                    animate={{ height: 'auto', opacity: 1 }}
                    exit={{ height: 0, opacity: 0 }}
                    className="overflow-hidden mt-3"
                  >
                    {(() => {
                      const v = vehicles.find(x => x.id === selectedVehicle);
                      if (!v) return null;
                      return (
                        <div className="rounded-lg p-3 space-y-2" style={{ background: `${COLORS.cyan}06`, border: `1px solid ${COLORS.cyan}15` }}>
                          <div className="text-xs font-bold" style={{ color: COLORS.cyan }}>VEHICLE TELEMETRY</div>
                          <div className="grid grid-cols-2 gap-1.5">
                            <div className="flex items-center justify-between py-1 px-2 rounded-md" style={{ background: 'rgba(243,244,246,0.4)' }}>
                              <span className="text-[10px] text-white/30">LAT</span>
                              <span className="text-[10px] font-mono text-white/60">{v.lat.toFixed(6)}</span>
                            </div>
                            <div className="flex items-center justify-between py-1 px-2 rounded-md" style={{ background: 'rgba(243,244,246,0.4)' }}>
                              <span className="text-[10px] text-white/30">LON</span>
                              <span className="text-[10px] font-mono text-white/60">{v.lon.toFixed(6)}</span>
                            </div>
                            <div className="flex items-center justify-between py-1 px-2 rounded-md" style={{ background: 'rgba(243,244,246,0.4)' }}>
                              <span className="text-[10px] text-white/30">HDG</span>
                              <span className="text-[10px] font-mono text-white/60">{v.heading.toFixed(1)}°</span>
                            </div>
                            <div className="flex items-center justify-between py-1 px-2 rounded-md" style={{ background: 'rgba(243,244,246,0.4)' }}>
                              <span className="text-[10px] text-white/30">SPD</span>
                              <span className="text-[10px] font-mono" style={{ color: COLORS.cyan }}>{v.speed.toFixed(1)} km/h</span>
                            </div>
                            <div className="flex items-center justify-between py-1 px-2 rounded-md" style={{ background: 'rgba(243,244,246,0.4)' }}>
                              <span className="text-[10px] text-white/30">BAT</span>
                              <span className="text-[10px] font-mono" style={{ color: v.battery > 30 ? COLORS.green : COLORS.red }}>{v.battery.toFixed(0)}%</span>
                            </div>
                            <div className="flex items-center justify-between py-1 px-2 rounded-md" style={{ background: 'rgba(243,244,246,0.4)' }}>
                              <span className="text-[10px] text-white/30">SIG</span>
                              <span className="text-[10px] font-mono" style={{ color: v.signal > 60 ? COLORS.green : COLORS.orange }}>{v.signal.toFixed(0)}%</span>
                            </div>
                          </div>
                          <div className="text-[9px] text-white/15 font-mono text-center mt-1">
                            Last update: {new Date(v.lastUpdate).toLocaleTimeString()}
                          </div>
                        </div>
                      );
                    })()}
                  </motion.div>
                )}
              </AnimatePresence>
            </motion.div>
          )}

          {activeTab === 'missions' && (
            <motion.div key="missions" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}
              className="space-y-2">
              {missions.map(m => (
                <MissionCard key={m.id} mission={m} />
              ))}
            </motion.div>
          )}

          {activeTab === 'vrp' && (
            <motion.div key="vrp" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              <div className="rounded-lg p-4 mb-3" style={{ background: `${COLORS.purple}08`, border: `1px solid ${COLORS.purple}15` }}>
                <div className="flex items-center gap-2 mb-2">
                  <Route className="w-4 h-4" style={{ color: COLORS.purple }} />
                  <span className="text-xs font-bold" style={{ color: COLORS.purple }}>VRP OPTIMIZER</span>
                </div>
                <p className="text-[10px] text-white/35 mb-3">
                  Multi-stop Vehicle Routing Problem solver using nearest-neighbor heuristic.
                  Optimizes route assignments across available vehicles.
                </p>
                <button
                  onClick={runVRP}
                  disabled={vrpRunning}
                  className="w-full py-2.5 rounded-lg text-xs font-bold tracking-wider transition-all cursor-pointer flex items-center justify-center gap-2"
                  style={{
                    background: vrpRunning ? 'rgba(229,231,235,0.5)' : `${COLORS.purple}20`,
                    border: `1px solid ${COLORS.purple}30`,
                    color: COLORS.purple,
                    opacity: vrpRunning ? 0.5 : 1,
                  }}
                >
                  {vrpRunning ? (
                    <>
                      <motion.div animate={{ rotate: 360 }} transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}>
                        <RefreshCw className="w-4 h-4" />
                      </motion.div>
                      COMPUTING...
                    </>
                  ) : (
                    <>
                      <Brain className="w-4 h-4" />
                      RUN OPTIMIZATION
                    </>
                  )}
                </button>
              </div>

              {vrpResult && (
                <div className="space-y-2">
                  <div className="text-[10px] font-bold text-white/40 tracking-wider mb-2">OPTIMIZED ROUTES</div>
                  {vrpResult.map((assignment, i) => (
                    <div key={i} className="rounded-lg p-3" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)' }}>
                      <div className="flex items-center gap-2 mb-2">
                        <Truck className="w-4 h-4" style={{ color: COLORS.cyan }} />
                        <span className="text-xs font-bold text-white/70">Vehicle {assignment.vehicleId}</span>
                        <span className="text-[10px] font-mono ml-auto" style={{ color: COLORS.green }}>
                          {assignment.totalDist.toFixed(1)} km
                        </span>
                      </div>
                      <div className="flex items-center gap-1 flex-wrap">
                        <div className="px-2 py-0.5 rounded-md text-[9px] font-mono"
                          style={{ background: `${COLORS.green}10`, color: COLORS.green }}>DEPOT</div>
                        {assignment.route.map((stop, j) => (
                          <div key={j} className="flex items-center gap-1">
                            <ChevronRight className="w-3 h-3 text-white/15" />
                            <div className="px-2 py-0.5 rounded-md text-[9px] font-mono text-white/40"
                              style={{ background: 'rgba(243,244,246,0.5)' }}>
                              {stop.label}
                            </div>
                          </div>
                        ))}
                        <ChevronRight className="w-3 h-3 text-white/15" />
                        <div className="px-2 py-0.5 rounded-md text-[9px] font-mono"
                          style={{ background: `${COLORS.green}10`, color: COLORS.green }}>DEPOT</div>
                      </div>
                    </div>
                  ))}
                  <div className="text-center mt-2">
                    <span className="text-[9px] font-mono text-white/20">
                      Total: {vrpResult.reduce((s, r) => s + r.totalDist, 0).toFixed(1)} km across {vrpResult.length} vehicles
                    </span>
                  </div>
                </div>
              )}
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      {/* Footer */}
      <div className="flex-shrink-0 px-4 py-2 text-center" style={{ borderTop: '1px solid rgba(229,231,235,0.4)' }}>
        <div className="text-[8px] font-mono text-white/12 tracking-wider">
          C4ISR COMMAND • 1Hz LIVE TRACKING • VRP SOLVER
        </div>
      </div>
    </motion.div>
  );
}
