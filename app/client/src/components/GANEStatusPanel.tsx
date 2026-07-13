/**
 * G.A.N.E Engine Status Panel
 * =============================
 * Real-time dashboard showing all engine states:
 * - FSM state + transitions
 * - Sensor health
 * - WebSocket connection
 * - Position/heading/speed
 * - Nearby anomalies
 * - Active UI profile
 */

import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { useGANE } from "@/contexts/GANEContext";
import { getFSMStateInfo } from "@/engine/fsm";
import {
  X, Activity, Wifi, WifiOff, Satellite, Compass, Gauge,
  Shield, ShieldAlert, Brain, Cpu, Radio, Eye, AlertTriangle,
  ChevronDown, ChevronRight, Zap, Signal
} from "lucide-react";

const COLORS = {
  cyan: '#2563EB',
  green: '#16A34A',
  red: '#DC2626',
  purple: '#7C3AED',
  orange: '#F97316',
  gold: '#FACC15',
  blue: '#3B82F6',
};

function StatusDot({ color, pulse = false }: { color: string; pulse?: boolean }) {
  return (
    <div className="relative">
      <div className="w-2.5 h-2.5 rounded-full" style={{ background: color, boxShadow: `0 0 8px ${color}60` }} />
      {pulse && (
        <motion.div
          className="absolute inset-[-3px] rounded-full"
          style={{ border: `1px solid ${color}40` }}
          animate={{ scale: [1, 1.8, 1], opacity: [0.6, 0, 0.6] }}
          transition={{ duration: 2, repeat: Infinity }}
        />
      )}
    </div>
  );
}

function SectionHeader({ title, icon: Icon, color, expanded, onToggle }: {
  title: string; icon: typeof Activity; color: string; expanded: boolean; onToggle: () => void;
}) {
  return (
    <button
      onClick={onToggle}
      className="w-full flex items-center gap-2.5 py-2.5 px-1 cursor-pointer hover:bg-white/[0.02] rounded-lg transition-colors"
    >
      <Icon className="w-4 h-4" style={{ color }} />
      <span className="text-xs font-bold tracking-wider uppercase" style={{ color }}>{title}</span>
      <div className="ml-auto">
        {expanded ? <ChevronDown className="w-3.5 h-3.5 text-white/30" /> : <ChevronRight className="w-3.5 h-3.5 text-white/30" />}
      </div>
    </button>
  );
}

function DataRow({ label, value, color, unit }: { label: string; value: string | number; color?: string; unit?: string }) {
  return (
    <div className="flex items-center justify-between py-1.5 px-2">
      <span className="text-[11px] text-white/40 font-mono">{label}</span>
      <span className="text-[11px] font-mono font-bold" style={{ color: color ?? 'rgba(55,65,81,0.9)' }}>
        {value}{unit && <span className="text-white/25 ml-1">{unit}</span>}
      </span>
    </div>
  );
}

export default function GANEStatusPanel({ onClose }: { onClose: () => void }) {
  const { state } = useGANE();
  const [expandedSections, setExpandedSections] = useState<Record<string, boolean>>({
    fsm: true,
    sensors: true,
    network: true,
    position: true,
    anomalies: false,
  });

  const toggleSection = (key: string) => {
    setExpandedSections(prev => ({ ...prev, [key]: !prev[key] }));
  };

  const fsmInfo = getFSMStateInfo(state.fsmState);
  const fsmColor = fsmInfo.color === 'green' ? COLORS.green
    : fsmInfo.color === 'yellow' ? COLORS.orange
    : fsmInfo.color === 'red' ? COLORS.red
    : COLORS.cyan;

  return (
    <motion.div
      initial={{ x: -320, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -320, opacity: 0 }}
      transition={{ type: 'spring', damping: 28, stiffness: 300 }}
      className="expand-panel fixed left-[96px] top-0 bottom-0 z-50 overflow-y-auto"
      style={{
        width: '340px',
        background: 'linear-gradient(180deg, rgba(4,8,18,0.97) 0%, rgba(2,4,10,0.99) 100%)',
        borderRight: '1px solid rgba(229,231,235,0.6)',
        boxShadow: '8px 0 40px rgba(249,250,251,0.8)',
      }}
    >
      {/* Header */}
      <div className="sticky top-0 z-10 px-5 py-4" style={{ background: 'rgba(4,8,18,0.98)', borderBottom: '1px solid rgba(229,231,235,0.5)' }}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl flex items-center justify-center"
              style={{ background: `${COLORS.cyan}15`, border: `1px solid ${COLORS.cyan}25` }}>
              <Brain className="w-5 h-5" style={{ color: COLORS.cyan }} />
            </div>
            <div>
              <h2 className="text-sm font-bold text-white/90 tracking-wide">G.A.N.E STATUS</h2>
              <p className="text-[10px] text-white/30 font-mono">ENGINE DIAGNOSTICS</p>
            </div>
          </div>
          <button onClick={onClose} className="p-2 rounded-lg hover:bg-white/5 transition-colors cursor-pointer">
            <X className="w-4 h-4 text-white/40" />
          </button>
        </div>

        {/* FSM State Badge */}
        <div className="mt-3 flex items-center gap-2 px-3 py-2 rounded-lg"
          style={{ background: `${fsmColor}10`, border: `1px solid ${fsmColor}20` }}>
          <StatusDot color={fsmColor} pulse />
          <span className="text-xs font-bold tracking-wider" style={{ color: fsmColor }}>{state.fsmState}</span>
          <span className="text-[10px] text-white/30 ml-auto font-mono">{fsmInfo.label}</span>
        </div>
      </div>

      <div className="px-4 py-3 space-y-2">
        {/* ─── FSM Section ─── */}
        <SectionHeader title="State Machine" icon={Cpu} color={COLORS.cyan} expanded={expandedSections.fsm} onToggle={() => toggleSection('fsm')} />
        <AnimatePresence>
          {expandedSections.fsm && (
            <motion.div initial={{ height: 0, opacity: 0 }} animate={{ height: 'auto', opacity: 1 }} exit={{ height: 0, opacity: 0 }}
              className="overflow-hidden">
              <div className="rounded-lg p-2 space-y-1" style={{ background: 'rgba(243,244,246,0.4)' }}>
                <DataRow label="Current State" value={state.fsmState} color={fsmColor} />
                <DataRow label="Profile" value={state.profileId.toUpperCase()} color={COLORS.purple} />
                <DataRow label="Transitions" value={state.transitionHistory.length} />
                {state.lastTransition && (
                  <>
                    <DataRow label="Last From" value={state.lastTransition.from} />
                    <DataRow label="Last To" value={state.lastTransition.to} color={COLORS.green} />
                    <DataRow label="Reason" value={state.lastTransition.reason} />
                  </>
                )}
              </div>

              {/* UI Profile Info */}
              <div className="mt-2 rounded-lg p-3" style={{ background: `${state.activeProfile.colors.primary}08`, border: `1px solid ${state.activeProfile.colors.primary}15` }}>
                <div className="flex items-center gap-2 mb-2">
                  <span className="text-xs font-bold" style={{ color: state.activeProfile.colors.primary }}>{state.activeProfile.name}</span>
                  <span className="text-[10px] text-white/30" style={{ direction: 'rtl' }}>{state.activeProfile.nameHe}</span>
                </div>
                <div className="grid grid-cols-2 gap-1">
                  <DataRow label="Map Style" value={state.activeProfile.mapStyle} />
                  <DataRow label="HUD Layout" value={state.activeProfile.hudLayout} />
                  <DataRow label="Voice" value={state.activeProfile.features.voiceGuidance ? 'ON' : 'OFF'} color={state.activeProfile.features.voiceGuidance ? COLORS.green : COLORS.red} />
                  <DataRow label="Haptic" value={state.activeProfile.features.hapticFeedback ? 'ON' : 'OFF'} color={state.activeProfile.features.hapticFeedback ? COLORS.green : COLORS.red} />
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        {/* ─── Sensors Section ─── */}
        <SectionHeader title="Sensor Health" icon={Activity} color={COLORS.green} expanded={expandedSections.sensors} onToggle={() => toggleSection('sensors')} />
        <AnimatePresence>
          {expandedSections.sensors && (
            <motion.div initial={{ height: 0, opacity: 0 }} animate={{ height: 'auto', opacity: 1 }} exit={{ height: 0, opacity: 0 }}
              className="overflow-hidden">
              <div className="rounded-lg p-2 space-y-2" style={{ background: 'rgba(243,244,246,0.4)' }}>
                {state.sensorHealth ? (
                  <>
                    <div className="flex items-center gap-3 py-1.5 px-2">
                      <Satellite className="w-4 h-4" style={{ color: state.sensorHealth.gnss ? COLORS.green : COLORS.red }} />
                      <span className="text-[11px] text-white/50">GNSS</span>
                      <StatusDot color={state.sensorHealth.gnss ? COLORS.green : COLORS.red} pulse={state.sensorHealth.gnss} />
                      <span className="text-[11px] font-mono ml-auto" style={{ color: state.sensorHealth.gnss ? COLORS.green : COLORS.red }}>
                        {state.sensorHealth.gnss ? 'LOCKED' : 'LOST'}
                      </span>
                    </div>
                    <div className="flex items-center gap-3 py-1.5 px-2">
                      <Zap className="w-4 h-4" style={{ color: state.sensorHealth.imu ? COLORS.green : COLORS.red }} />
                      <span className="text-[11px] text-white/50">IMU</span>
                      <StatusDot color={state.sensorHealth.imu ? COLORS.green : COLORS.red} />
                      <span className="text-[11px] font-mono ml-auto" style={{ color: state.sensorHealth.imu ? COLORS.green : COLORS.red }}>
                        {state.sensorHealth.imu ? 'ACTIVE' : 'OFFLINE'}
                      </span>
                    </div>
                    <div className="flex items-center gap-3 py-1.5 px-2">
                      <Eye className="w-4 h-4" style={{ color: state.sensorHealth.vision ? COLORS.green : COLORS.orange }} />
                      <span className="text-[11px] text-white/50">Vision</span>
                      <StatusDot color={state.sensorHealth.vision ? COLORS.green : COLORS.orange} />
                      <span className="text-[11px] font-mono ml-auto" style={{ color: state.sensorHealth.vision ? COLORS.green : COLORS.orange }}>
                        {state.sensorHealth.vision ? 'TRACKING' : 'STANDBY'}
                      </span>
                    </div>
                    <div className="flex items-center gap-3 py-1.5 px-2">
                      {state.sensorHealth.spoofingDetected
                        ? <ShieldAlert className="w-4 h-4" style={{ color: COLORS.red }} />
                        : <Shield className="w-4 h-4" style={{ color: COLORS.green }} />}
                      <span className="text-[11px] text-white/50">FDE</span>
                      <StatusDot color={state.sensorHealth.spoofingDetected ? COLORS.red : COLORS.green} pulse={state.sensorHealth.spoofingDetected} />
                      <span className="text-[11px] font-mono ml-auto" style={{ color: state.sensorHealth.spoofingDetected ? COLORS.red : COLORS.green }}>
                        {state.sensorHealth.spoofingDetected ? 'ALERT!' : 'CLEAR'}
                      </span>
                    </div>
                    <DataRow label="Accuracy" value={`±${state.sensorHealth.gnssAccuracy.toFixed(1)}`} unit="m" color={state.sensorHealth.gnssAccuracy < 20 ? COLORS.green : COLORS.orange} />
                    <DataRow label="Confidence" value={`${(state.sensorHealth.confidenceScore * 100).toFixed(0)}`} unit="%" color={state.sensorHealth.confidenceScore > 0.7 ? COLORS.green : COLORS.orange} />
                    <DataRow label="Satellites" value={state.sensorHealth.satellites} color={COLORS.blue} />
                  </>
                ) : (
                  <div className="text-center py-4 text-white/20 text-xs font-mono">INITIALIZING SENSORS...</div>
                )}
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        {/* ─── Network Section ─── */}
        <SectionHeader title="Network & WS" icon={Radio} color={COLORS.blue} expanded={expandedSections.network} onToggle={() => toggleSection('network')} />
        <AnimatePresence>
          {expandedSections.network && (
            <motion.div initial={{ height: 0, opacity: 0 }} animate={{ height: 'auto', opacity: 1 }} exit={{ height: 0, opacity: 0 }}
              className="overflow-hidden">
              <div className="rounded-lg p-2 space-y-1" style={{ background: 'rgba(243,244,246,0.4)' }}>
                <div className="flex items-center gap-3 py-1.5 px-2">
                  {state.wsConnected
                    ? <Wifi className="w-4 h-4" style={{ color: COLORS.green }} />
                    : <WifiOff className="w-4 h-4" style={{ color: COLORS.red }} />}
                  <span className="text-[11px] text-white/50">WebSocket</span>
                  <StatusDot color={state.wsConnected ? COLORS.green : COLORS.red} pulse={state.wsConnected} />
                  <span className="text-[11px] font-mono ml-auto" style={{ color: state.wsConnected ? COLORS.green : COLORS.red }}>
                    {state.wsConnected ? 'CONNECTED' : 'OFFLINE'}
                  </span>
                </div>
                {state.wsClientId && (
                  <DataRow label="Client ID" value={state.wsClientId.slice(0, 12) + '...'} color={COLORS.cyan} />
                )}
                <DataRow label="Delta Updates" value={state.deltaUpdates.length} />
                <DataRow label="Browser" value={navigator.onLine ? 'ONLINE' : 'OFFLINE'} color={navigator.onLine ? COLORS.green : COLORS.red} />
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        {/* ─── Position Section ─── */}
        <SectionHeader title="Position & Nav" icon={Compass} color={COLORS.orange} expanded={expandedSections.position} onToggle={() => toggleSection('position')} />
        <AnimatePresence>
          {expandedSections.position && (
            <motion.div initial={{ height: 0, opacity: 0 }} animate={{ height: 'auto', opacity: 1 }} exit={{ height: 0, opacity: 0 }}
              className="overflow-hidden">
              <div className="rounded-lg p-2 space-y-1" style={{ background: 'rgba(243,244,246,0.4)' }}>
                {state.position ? (
                  <>
                    <DataRow label="Latitude" value={state.position.lat.toFixed(6)} unit="°" color={COLORS.cyan} />
                    <DataRow label="Longitude" value={state.position.lon.toFixed(6)} unit="°" color={COLORS.cyan} />
                    <DataRow label="Altitude" value={state.position.alt.toFixed(1)} unit="m" />
                  </>
                ) : (
                  <DataRow label="Position" value="ACQUIRING..." color={COLORS.orange} />
                )}
                <DataRow label="Heading" value={state.heading.toFixed(1)} unit="°" color={COLORS.blue} />
                <DataRow label="Speed" value={state.speed.toFixed(1)} unit="km/h" color={COLORS.green} />
                <DataRow label="Confidence" value={`${(state.confidence * 100).toFixed(0)}`} unit="%" />
                <DataRow label="Satellites" value={state.satellites} color={COLORS.purple} />
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        {/* ─── Anomalies Section ─── */}
        <SectionHeader title="Anomalies" icon={AlertTriangle} color={COLORS.red} expanded={expandedSections.anomalies} onToggle={() => toggleSection('anomalies')} />
        <AnimatePresence>
          {expandedSections.anomalies && (
            <motion.div initial={{ height: 0, opacity: 0 }} animate={{ height: 'auto', opacity: 1 }} exit={{ height: 0, opacity: 0 }}
              className="overflow-hidden">
              <div className="rounded-lg p-2" style={{ background: 'rgba(243,244,246,0.4)' }}>
                {state.nearbyAnomalies.length > 0 ? (
                  <div className="space-y-2">
                    {state.nearbyAnomalies.slice(-5).reverse().map((anomaly, i) => (
                      <div key={i} className="flex items-center gap-2 py-1.5 px-2 rounded-md"
                        style={{ background: `${COLORS.red}08`, border: `1px solid ${COLORS.red}10` }}>
                        <AlertTriangle className="w-3.5 h-3.5" style={{ color: COLORS.red }} />
                        <div className="flex-1 min-w-0">
                          <div className="text-[11px] font-bold text-white/60">{anomaly.type}</div>
                          {anomaly.description && (
                            <div className="text-[10px] text-white/30 truncate">{anomaly.description}</div>
                          )}
                        </div>
                        <div className="flex items-center gap-1">
                          {Array.from({ length: anomaly.severity }).map((_, j) => (
                            <div key={j} className="w-1.5 h-1.5 rounded-full" style={{ background: COLORS.red }} />
                          ))}
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-center py-4 text-white/20 text-xs font-mono">NO ANOMALIES DETECTED</div>
                )}
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        {/* ─── Engine Version ─── */}
        <div className="mt-4 pt-3 text-center" style={{ borderTop: '1px solid rgba(229,231,235,0.4)' }}>
          <div className="text-[9px] font-mono text-white/15 tracking-wider">
            G.A.N.E OMNI-MATRIX v3.0 — DISTRIBUTED MESH
          </div>
          <div className="text-[8px] font-mono text-white/10 mt-1">
            ESKF • PDR • VO • FDE • SLAM • FSM • WS • AR
          </div>
        </div>
      </div>
    </motion.div>
  );
}
