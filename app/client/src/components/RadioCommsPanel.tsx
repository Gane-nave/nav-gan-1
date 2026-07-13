/**
 * RadioCommsPanel — VHF/UHF/HF Radio Communications Interface
 * =============================================================
 * Full radio management panel with:
 * - Band selector (HF/VHF/UHF)
 * - Frequency tuner with dial
 * - Emergency frequency quick-access
 * - APRS position beaconing control
 * - Mesh network topology view
 * - Channel list with squelch control
 * - PTT (Push-to-Talk) button
 * - Transmission log
 */
import { useState, useEffect, useMemo, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Radio, X, AlertTriangle, Wifi, WifiOff, Zap,
  Volume2, VolumeX, Send, Mic, MicOff, Shield,
  Signal, Activity, MapPin, Users, ChevronRight,
  ToggleLeft, ToggleRight, Antenna, Radar,
  Navigation, Satellite, Phone, PhoneOff
} from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";
import {
  RadioCommsEngine, getRadioEngine,
  EMERGENCY_FREQUENCIES, STANDARD_FREQUENCIES,
  type RadioBand, type RadioStatus, type RadioChannel
} from "@/engine/radioCommsEngine";

type Tab = 'frequencies' | 'emergency' | 'mesh' | 'log';

const BAND_COLORS: Record<RadioBand, string> = {
  HF: '#ff9900',
  VHF: '#00e5ff',
  UHF: '#aa66ff',
};

const BAND_RANGES: Record<RadioBand, { min: number; max: number; unit: string }> = {
  HF: { min: 3, max: 30, unit: 'MHz' },
  VHF: { min: 30, max: 300, unit: 'MHz' },
  UHF: { min: 300, max: 3000, unit: 'MHz' },
};

export default function RadioCommsPanel({ onClose }: { onClose: () => void }) {
  const { t, dir } = useLanguage();
  const [engine] = useState(() => getRadioEngine());
  const [status, setStatus] = useState<RadioStatus>(engine.getStatus());
  const [activeTab, setActiveTab] = useState<Tab>('frequencies');
  const [activeBand, setActiveBand] = useState<RadioBand>('VHF');
  const [frequency, setFrequency] = useState(145.500);
  const [squelch, setSquelch] = useState(3);
  const [pttActive, setPttActive] = useState(false);
  const [aprsEnabled, setAprsEnabled] = useState(false);
  const [meshEnabled, setMeshEnabled] = useState(false);
  const [emergencyMode, setEmergencyMode] = useState(false);
  const [txPower, setTxPower] = useState(5);

  // Update status periodically
  useEffect(() => {
    const interval = setInterval(() => {
      setStatus(engine.getStatus());
    }, 1000);
    return () => clearInterval(interval);
  }, [engine]);

  // Band change
  const handleBandChange = useCallback((band: RadioBand) => {
    setActiveBand(band);
    const range = BAND_RANGES[band];
    const midFreq = (range.min + range.max) / 2;
    setFrequency(Math.round(midFreq * 1000) / 1000);
    engine.tune(midFreq, band);
  }, [engine]);

  // Frequency change
  const handleFrequencyChange = useCallback((freq: number) => {
    setFrequency(freq);
    engine.tune(freq);
  }, [engine]);

  // PTT
  const handlePTT = useCallback((active: boolean) => {
    setPttActive(active);
    if (active) engine.startTransmit();
    else engine.stopTransmit();
  }, [engine]);

  // APRS toggle
  const toggleAPRS = useCallback(() => {
    if (aprsEnabled) {
      engine.stopAPRSBeacon();
      setAprsEnabled(false);
    } else {
      engine.startAPRSBeacon();
      setAprsEnabled(true);
    }
  }, [engine, aprsEnabled]);

  // Mesh toggle
  const toggleMesh = useCallback(() => {
    if (meshEnabled) {
      engine.stopMeshDiscovery();
      setMeshEnabled(false);
    } else {
      engine.startMeshDiscovery();
      setMeshEnabled(true);
    }
  }, [engine, meshEnabled]);

  // Emergency toggle
  const toggleEmergency = useCallback(() => {
    if (emergencyMode) {
      engine.deactivateEmergency();
      setEmergencyMode(false);
    } else {
      engine.activateEmergency();
      setEmergencyMode(true);
    }
  }, [engine, emergencyMode]);

  const tabs: { id: Tab; label: string; icon: typeof Radio }[] = [
    { id: 'frequencies', label: 'Freq', icon: Radio },
    { id: 'emergency', label: 'SOS', icon: AlertTriangle },
    { id: 'mesh', label: 'Mesh', icon: Users },
    { id: 'log', label: 'Log', icon: Activity },
  ];

  const signalBars = useMemo(() => {
    const strength = status.signalStrength;
    if (strength > -50) return 5;
    if (strength > -65) return 4;
    if (strength > -75) return 3;
    if (strength > -85) return 2;
    if (strength > -95) return 1;
    return 0;
  }, [status.signalStrength]);

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: 'spring', damping: 30, stiffness: 300 }}
      className="expand-panel scrollbar-none"
      style={{ direction: dir }}
    >
      {/* Header */}
      <div className="sticky top-0 z-10 p-4 pb-3" style={{
        background: emergencyMode ? 'rgba(255,20,20,0.08)' : 'rgba(4,8,18,0.95)',
        borderBottom: `1px solid ${emergencyMode ? 'rgba(255,51,85,0.3)' : 'rgba(0,229,255,0.08)'}`,
      }}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <motion.div
              animate={emergencyMode ? { scale: [1, 1.15, 1], boxShadow: ['0 0 0px rgba(255,51,85,0)', '0 0 20px rgba(255,51,85,0.4)', '0 0 0px rgba(255,51,85,0)'] } : {}}
              transition={{ duration: 1.5, repeat: Infinity }}
              className="w-10 h-10 rounded-xl flex items-center justify-center"
              style={{
                background: emergencyMode ? 'rgba(255,51,85,0.15)' : `rgba(0,229,255,0.08)`,
                border: `1px solid ${emergencyMode ? 'rgba(255,51,85,0.3)' : 'rgba(0,229,255,0.15)'}`,
              }}
            >
              <Radio size={18} style={{ color: emergencyMode ? '#ff3355' : '#00e5ff' }} />
            </motion.div>
            <div>
              <h3 className="text-sm font-bold tracking-wider uppercase" style={{
                fontFamily: 'Syne, sans-serif',
                color: emergencyMode ? '#ff3355' : '#00e5ff',
              }}>
                {emergencyMode ? 'EMERGENCY' : 'RADIO COMMS'}
              </h3>
              <p className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>
                VHF · UHF · HF · APRS · Mesh
              </p>
            </div>
          </div>
          <button onClick={onClose} className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
            style={{ background: 'rgba(229,231,235,0.5)', border: '1px solid rgba(209,213,219,0.5)' }}>
            <X size={14} style={{ color: 'rgba(107,114,128,0.9)' }} />
          </button>
        </div>

        {/* Band Selector */}
        <div className="flex gap-2 mt-3">
          {(['HF', 'VHF', 'UHF'] as RadioBand[]).map(band => (
            <button
              key={band}
              onClick={() => handleBandChange(band)}
              className="flex-1 py-1.5 rounded-lg text-xs font-bold tracking-wider transition-all cursor-pointer"
              style={{
                background: activeBand === band ? `${BAND_COLORS[band]}15` : 'rgba(243,244,246,0.3)',
                border: `1px solid ${activeBand === band ? `${BAND_COLORS[band]}40` : 'rgba(229,231,235,0.3)'}`,
                color: activeBand === band ? BAND_COLORS[band] : 'rgba(107,114,128,0.7)',
                fontFamily: 'JetBrains Mono, monospace',
              }}
            >
              {band}
            </button>
          ))}
        </div>
      </div>

      {/* Frequency Display */}
      <div className="px-4 pt-3">
        <div className="rounded-xl p-4 relative overflow-hidden" style={{
          background: 'rgba(0,0,0,0.03)',
          border: `1px solid ${BAND_COLORS[activeBand]}20`,
        }}>
          {/* Signal strength bars */}
          <div className="absolute top-3 right-3 flex gap-0.5 items-end">
            {[1, 2, 3, 4, 5].map(i => (
              <div key={i} className="w-1 rounded-full transition-all" style={{
                height: 4 + i * 3,
                background: i <= signalBars ? BAND_COLORS[activeBand] : 'rgba(209,213,219,0.3)',
                opacity: i <= signalBars ? 1 : 0.3,
              }} />
            ))}
          </div>

          <div className="text-[10px] font-bold tracking-wider uppercase mb-1" style={{
            color: `${BAND_COLORS[activeBand]}80`,
            fontFamily: 'Syne, sans-serif',
          }}>
            {activeBand} Band · {BAND_RANGES[activeBand].min}–{BAND_RANGES[activeBand].max} MHz
          </div>

          <div className="text-3xl font-bold font-mono tracking-tight" style={{
            color: BAND_COLORS[activeBand],
            textShadow: `0 0 20px ${BAND_COLORS[activeBand]}30`,
          }}>
            {frequency.toFixed(3)} <span className="text-sm opacity-50">MHz</span>
          </div>

          {/* Frequency slider */}
          <input
            type="range"
            min={BAND_RANGES[activeBand].min * 1000}
            max={BAND_RANGES[activeBand].max * 1000}
            value={frequency * 1000}
            onChange={e => handleFrequencyChange(Number(e.target.value) / 1000)}
            className="w-full mt-2 accent-cyan-400"
            style={{ accentColor: BAND_COLORS[activeBand] }}
          />

          {/* Status row */}
          <div className="flex items-center gap-3 mt-2">
            <div className="flex items-center gap-1">
              <div className="w-1.5 h-1.5 rounded-full" style={{
                background: status.squelchOpen ? '#00ff88' : 'rgba(209,213,219,0.5)',
                boxShadow: status.squelchOpen ? '0 0 6px rgba(0,255,136,0.5)' : 'none',
              }} />
              <span className="text-[9px] font-mono" style={{ color: 'rgba(107,114,128,0.7)' }}>
                SQL:{squelch}
              </span>
            </div>
            <span className="text-[9px] font-mono" style={{ color: 'rgba(107,114,128,0.7)' }}>
              PWR:{txPower}W
            </span>
            <span className="text-[9px] font-mono" style={{ color: 'rgba(107,114,128,0.7)' }}>
              S:{status.signalStrength.toFixed(0)}dBm
            </span>
            {aprsEnabled && (
              <span className="text-[9px] font-mono" style={{ color: '#00ff88' }}>APRS</span>
            )}
            {meshEnabled && (
              <span className="text-[9px] font-mono" style={{ color: '#aa66ff' }}>MESH:{status.meshNodeCount}</span>
            )}
          </div>
        </div>
      </div>

      {/* PTT Button */}
      <div className="px-4 py-3">
        <motion.button
          onPointerDown={() => handlePTT(true)}
          onPointerUp={() => handlePTT(false)}
          onPointerLeave={() => handlePTT(false)}
          whileTap={{ scale: 0.95 }}
          className="w-full py-3 rounded-xl flex items-center justify-center gap-2 font-bold text-sm tracking-wider cursor-pointer"
          style={{
            background: pttActive
              ? 'linear-gradient(135deg, #ff3355, #ff5500)'
              : 'linear-gradient(135deg, rgba(0,229,255,0.1), rgba(170,102,255,0.1))',
            border: `1px solid ${pttActive ? 'rgba(255,51,85,0.4)' : 'rgba(0,229,255,0.2)'}`,
            color: pttActive ? '#fff' : '#00e5ff',
            boxShadow: pttActive ? '0 0 30px rgba(255,51,85,0.3)' : 'none',
            fontFamily: 'Syne, sans-serif',
          }}
        >
          {pttActive ? <Mic size={16} /> : <MicOff size={16} />}
          {pttActive ? 'TRANSMITTING' : 'PUSH TO TALK'}
        </motion.button>
      </div>

      {/* Tabs */}
      <div className="px-4">
        <div className="flex gap-1 p-1 rounded-lg" style={{ background: 'rgba(243,244,246,0.3)' }}>
          {tabs.map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className="flex-1 py-1.5 rounded-md flex items-center justify-center gap-1 text-[10px] font-bold tracking-wider transition-all cursor-pointer"
              style={{
                background: activeTab === tab.id ? 'white' : 'transparent',
                color: activeTab === tab.id ? '#00e5ff' : 'rgba(107,114,128,0.6)',
                boxShadow: activeTab === tab.id ? '0 1px 3px rgba(0,0,0,0.05)' : 'none',
              }}
            >
              <tab.icon size={11} />
              {tab.label}
            </button>
          ))}
        </div>
      </div>

      {/* Tab Content */}
      <div className="px-4 pb-6 pt-3 space-y-3">
        <AnimatePresence mode="wait">
          {activeTab === 'frequencies' && (
            <motion.div key="freq" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }} className="space-y-2">
              {/* Quick Controls */}
              <div className="grid grid-cols-2 gap-2">
                <button onClick={toggleAPRS} className="p-2.5 rounded-xl flex items-center gap-2 cursor-pointer transition-all"
                  style={{
                    background: aprsEnabled ? 'rgba(0,255,136,0.08)' : 'rgba(243,244,246,0.4)',
                    border: `1px solid ${aprsEnabled ? 'rgba(0,255,136,0.2)' : 'rgba(229,231,235,0.3)'}`,
                  }}>
                  <MapPin size={13} style={{ color: aprsEnabled ? '#00ff88' : 'rgba(156,163,175,0.6)' }} />
                  <span className="text-[10px] font-bold" style={{ color: aprsEnabled ? '#00ff88' : 'rgba(107,114,128,0.7)' }}>APRS Beacon</span>
                </button>
                <button onClick={toggleMesh} className="p-2.5 rounded-xl flex items-center gap-2 cursor-pointer transition-all"
                  style={{
                    background: meshEnabled ? 'rgba(170,102,255,0.08)' : 'rgba(243,244,246,0.4)',
                    border: `1px solid ${meshEnabled ? 'rgba(170,102,255,0.2)' : 'rgba(229,231,235,0.3)'}`,
                  }}>
                  <Users size={13} style={{ color: meshEnabled ? '#aa66ff' : 'rgba(156,163,175,0.6)' }} />
                  <span className="text-[10px] font-bold" style={{ color: meshEnabled ? '#aa66ff' : 'rgba(107,114,128,0.7)' }}>Mesh Net</span>
                </button>
              </div>

              {/* Squelch & Power */}
              <div className="rounded-xl p-3 space-y-3" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.3)' }}>
                <div>
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-[10px] font-bold" style={{ color: 'rgba(75,85,99,0.8)' }}>Squelch Level</span>
                    <span className="text-[10px] font-mono" style={{ color: BAND_COLORS[activeBand] }}>{squelch}</span>
                  </div>
                  <input type="range" min={0} max={9} value={squelch} onChange={e => { setSquelch(Number(e.target.value)); engine.setSquelch(Number(e.target.value)); }}
                    className="w-full" style={{ accentColor: BAND_COLORS[activeBand] }} />
                </div>
                <div>
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-[10px] font-bold" style={{ color: 'rgba(75,85,99,0.8)' }}>TX Power</span>
                    <span className="text-[10px] font-mono" style={{ color: BAND_COLORS[activeBand] }}>{txPower}W</span>
                  </div>
                  <input type="range" min={0.5} max={100} step={0.5} value={txPower} onChange={e => { setTxPower(Number(e.target.value)); engine.setTxPower(Number(e.target.value)); }}
                    className="w-full" style={{ accentColor: BAND_COLORS[activeBand] }} />
                </div>
              </div>

              {/* Channel List */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: `${BAND_COLORS[activeBand]}60` }}>
                Preset Channels
              </div>
              {STANDARD_FREQUENCIES.filter(f => f.band === activeBand).map((freq, i) => (
                <motion.button
                  key={freq.id}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.03 }}
                  onClick={() => handleFrequencyChange(freq.frequency)}
                  className="w-full p-2.5 rounded-xl flex items-center gap-3 cursor-pointer transition-all"
                  style={{
                    background: Math.abs(frequency - freq.frequency) < 0.001 ? `${BAND_COLORS[activeBand]}08` : 'rgba(243,244,246,0.3)',
                    border: `1px solid ${Math.abs(frequency - freq.frequency) < 0.001 ? `${BAND_COLORS[activeBand]}25` : 'rgba(229,231,235,0.3)'}`,
                  }}
                >
                  <div className="w-1.5 h-1.5 rounded-full" style={{
                    background: Math.abs(frequency - freq.frequency) < 0.001 ? BAND_COLORS[activeBand] : 'rgba(209,213,219,0.5)',
                  }} />
                  <div className="flex-1 text-left">
                    <div className="text-xs font-bold" style={{ color: 'rgba(55,65,81,0.9)' }}>{freq.label}</div>
                    <div className="text-[9px]" style={{ color: 'rgba(107,114,128,0.6)' }}>{freq.description}</div>
                  </div>
                  <span className="text-[10px] font-mono font-bold" style={{ color: BAND_COLORS[activeBand] }}>
                    {freq.frequency.toFixed(3)}
                  </span>
                </motion.button>
              ))}
            </motion.div>
          )}

          {activeTab === 'emergency' && (
            <motion.div key="sos" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }} className="space-y-3">
              {/* Emergency Toggle */}
              <motion.button
                onClick={toggleEmergency}
                whileTap={{ scale: 0.97 }}
                className="w-full p-4 rounded-xl flex items-center gap-3 cursor-pointer"
                style={{
                  background: emergencyMode
                    ? 'linear-gradient(135deg, rgba(255,51,85,0.15), rgba(255,85,0,0.1))'
                    : 'rgba(243,244,246,0.4)',
                  border: `2px solid ${emergencyMode ? 'rgba(255,51,85,0.4)' : 'rgba(229,231,235,0.3)'}`,
                }}
              >
                <motion.div
                  animate={emergencyMode ? { rotate: [0, 10, -10, 0] } : {}}
                  transition={{ duration: 0.5, repeat: Infinity }}
                >
                  <AlertTriangle size={24} style={{ color: emergencyMode ? '#ff3355' : 'rgba(156,163,175,0.5)' }} />
                </motion.div>
                <div className="flex-1 text-left">
                  <div className="text-sm font-bold" style={{ color: emergencyMode ? '#ff3355' : 'rgba(75,85,99,0.8)' }}>
                    {emergencyMode ? 'EMERGENCY ACTIVE' : 'Activate Emergency'}
                  </div>
                  <div className="text-[10px]" style={{ color: 'rgba(107,114,128,0.6)' }}>
                    {emergencyMode ? 'Broadcasting on all emergency frequencies' : 'Enables all distress frequencies + APRS beacon'}
                  </div>
                </div>
                <div className="w-8 h-8 rounded-full flex items-center justify-center" style={{
                  background: emergencyMode ? 'rgba(255,51,85,0.2)' : 'rgba(229,231,235,0.3)',
                }}>
                  {emergencyMode ? <PhoneOff size={14} style={{ color: '#ff3355' }} /> : <Phone size={14} style={{ color: 'rgba(156,163,175,0.5)' }} />}
                </div>
              </motion.button>

              {/* Emergency Frequencies */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,51,85,0.5)' }}>
                Emergency Frequencies
              </div>
              {EMERGENCY_FREQUENCIES.map((freq, i) => (
                <motion.button
                  key={freq.id}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.05 }}
                  onClick={() => { handleFrequencyChange(freq.frequency); handleBandChange(freq.band); }}
                  className="w-full p-3 rounded-xl flex items-center gap-3 cursor-pointer transition-all"
                  style={{
                    background: emergencyMode ? 'rgba(255,51,85,0.05)' : 'rgba(243,244,246,0.4)',
                    border: `1px solid ${emergencyMode ? 'rgba(255,51,85,0.15)' : 'rgba(229,231,235,0.3)'}`,
                  }}
                >
                  <div className="w-8 h-8 rounded-lg flex items-center justify-center" style={{
                    background: 'rgba(255,51,85,0.08)',
                    border: '1px solid rgba(255,51,85,0.15)',
                  }}>
                    <AlertTriangle size={14} style={{ color: '#ff3355' }} />
                  </div>
                  <div className="flex-1 text-left">
                    <div className="text-xs font-bold" style={{ color: 'rgba(55,65,81,0.9)' }}>{freq.label}</div>
                    <div className="text-[9px]" style={{ color: 'rgba(107,114,128,0.6)' }}>{freq.description}</div>
                  </div>
                  <div className="text-right">
                    <div className="text-xs font-mono font-bold" style={{ color: '#ff3355' }}>{freq.frequency.toFixed(3)}</div>
                    <div className="text-[9px]" style={{ color: 'rgba(156,163,175,0.6)' }}>{freq.band} · {freq.modulation}</div>
                  </div>
                </motion.button>
              ))}
            </motion.div>
          )}

          {activeTab === 'mesh' && (
            <motion.div key="mesh" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }} className="space-y-3">
              {/* Mesh Status */}
              <div className="rounded-xl p-4" style={{
                background: meshEnabled ? 'rgba(170,102,255,0.05)' : 'rgba(243,244,246,0.4)',
                border: `1px solid ${meshEnabled ? 'rgba(170,102,255,0.15)' : 'rgba(229,231,235,0.3)'}`,
              }}>
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-2">
                    <Users size={14} style={{ color: meshEnabled ? '#aa66ff' : 'rgba(156,163,175,0.5)' }} />
                    <span className="text-xs font-bold" style={{ color: meshEnabled ? '#aa66ff' : 'rgba(75,85,99,0.7)' }}>
                      Mesh Network
                    </span>
                  </div>
                  <button onClick={toggleMesh} className="cursor-pointer">
                    {meshEnabled
                      ? <ToggleRight size={20} style={{ color: '#aa66ff' }} />
                      : <ToggleLeft size={20} style={{ color: 'rgba(209,213,219,0.6)' }} />
                    }
                  </button>
                </div>
                <div className="grid grid-cols-3 gap-2">
                  {[
                    { label: 'Nodes', value: status.meshNodeCount.toString(), color: '#aa66ff' },
                    { label: 'Hops', value: '0', color: '#00e5ff' },
                    { label: 'Latency', value: meshEnabled ? '< 50ms' : '—', color: '#00ff88' },
                  ].map((stat, i) => (
                    <div key={i} className="text-center p-2 rounded-lg" style={{ background: 'rgba(243,244,246,0.3)' }}>
                      <div className="text-sm font-bold font-mono" style={{ color: stat.color }}>{stat.value}</div>
                      <div className="text-[9px]" style={{ color: 'rgba(107,114,128,0.6)' }}>{stat.label}</div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Mesh Topology Visualization */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(170,102,255,0.5)' }}>
                Network Topology
              </div>
              <div className="rounded-xl p-4 flex items-center justify-center" style={{
                background: 'rgba(0,0,0,0.02)',
                border: '1px solid rgba(229,231,235,0.3)',
                minHeight: 160,
              }}>
                {meshEnabled ? (
                  <svg width="200" height="140" viewBox="0 0 200 140">
                    {/* Central node (self) */}
                    <circle cx="100" cy="70" r="8" fill="#aa66ff" opacity={0.3} />
                    <circle cx="100" cy="70" r="4" fill="#aa66ff" />
                    <text x="100" y="90" textAnchor="middle" fontSize="8" fill="#aa66ff" fontFamily="JetBrains Mono">YOU</text>

                    {/* Simulated mesh nodes */}
                    {[
                      { x: 40, y: 30, label: 'N1' },
                      { x: 160, y: 40, label: 'N2' },
                      { x: 50, y: 110, label: 'N3' },
                      { x: 150, y: 100, label: 'N4' },
                    ].map((node, i) => (
                      <g key={i}>
                        <line x1={100} y1={70} x2={node.x} y2={node.y} stroke="#aa66ff" strokeWidth={1} opacity={0.2} strokeDasharray="3,3" />
                        <circle cx={node.x} cy={node.y} r={3} fill="#00e5ff" />
                        <text x={node.x} y={node.y - 8} textAnchor="middle" fontSize="7" fill="rgba(107,114,128,0.6)" fontFamily="JetBrains Mono">{node.label}</text>
                      </g>
                    ))}

                    {/* Inter-node links */}
                    <line x1={40} y1={30} x2={50} y2={110} stroke="rgba(0,229,255,0.15)" strokeWidth={0.5} strokeDasharray="2,2" />
                    <line x1={160} y1={40} x2={150} y2={100} stroke="rgba(0,229,255,0.15)" strokeWidth={0.5} strokeDasharray="2,2" />
                  </svg>
                ) : (
                  <div className="text-center">
                    <Users size={24} style={{ color: 'rgba(209,213,219,0.4)' }} className="mx-auto mb-2" />
                    <p className="text-[10px]" style={{ color: 'rgba(156,163,175,0.6)' }}>Enable mesh to discover nearby nodes</p>
                  </div>
                )}
              </div>

              {/* Mesh Features */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(170,102,255,0.5)' }}>
                Mesh Capabilities
              </div>
              {[
                { name: 'Multi-hop Relay', desc: 'Extend range through intermediate nodes', active: meshEnabled },
                { name: 'Store & Forward', desc: 'Queue messages for offline nodes', active: meshEnabled },
                { name: 'Position Sharing', desc: 'Broadcast GPS to all mesh peers', active: meshEnabled && aprsEnabled },
                { name: 'Encrypted Comms', desc: 'AES-256 end-to-end encryption', active: meshEnabled },
              ].map((cap, i) => (
                <div key={i} className="flex items-center gap-3 p-2.5 rounded-xl" style={{ background: 'rgba(243,244,246,0.3)' }}>
                  <div className="w-2 h-2 rounded-full" style={{
                    background: cap.active ? '#aa66ff' : 'rgba(209,213,219,0.5)',
                    boxShadow: cap.active ? '0 0 6px rgba(170,102,255,0.4)' : 'none',
                  }} />
                  <div className="flex-1">
                    <span className="text-xs font-bold" style={{ color: 'rgba(55,65,81,0.9)' }}>{cap.name}</span>
                    <p className="text-[9px]" style={{ color: 'rgba(107,114,128,0.6)' }}>{cap.desc}</p>
                  </div>
                </div>
              ))}
            </motion.div>
          )}

          {activeTab === 'log' && (
            <motion.div key="log" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }} className="space-y-2">
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(0,229,255,0.5)' }}>
                Transmission Log
              </div>
              {/* Simulated log entries */}
              {[
                { time: '12:34:56', type: 'RX', freq: '145.500', from: 'GANE-02', msg: 'Position update received', signal: -72 },
                { time: '12:33:21', type: 'TX', freq: '144.390', from: 'YOU', msg: 'APRS beacon sent', signal: 0 },
                { time: '12:30:45', type: 'RX', freq: '156.800', from: 'COAST-1', msg: 'Weather advisory Ch.16', signal: -85 },
                { time: '12:28:10', type: 'MESH', freq: 'mesh', from: 'NODE-3', msg: 'Relay: position from NODE-7', signal: -68 },
                { time: '12:25:00', type: 'RX', freq: '121.500', from: 'GUARD', msg: 'No emergency traffic', signal: -90 },
              ].map((entry, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, y: 5 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: i * 0.05 }}
                  className="p-2.5 rounded-xl flex items-start gap-2"
                  style={{ background: 'rgba(243,244,246,0.3)', border: '1px solid rgba(229,231,235,0.3)' }}
                >
                  <div className="w-6 h-6 rounded-md flex items-center justify-center mt-0.5" style={{
                    background: entry.type === 'TX' ? 'rgba(255,51,85,0.1)' : entry.type === 'MESH' ? 'rgba(170,102,255,0.1)' : 'rgba(0,229,255,0.1)',
                  }}>
                    <span className="text-[8px] font-bold font-mono" style={{
                      color: entry.type === 'TX' ? '#ff3355' : entry.type === 'MESH' ? '#aa66ff' : '#00e5ff',
                    }}>{entry.type}</span>
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="text-[10px] font-bold" style={{ color: 'rgba(55,65,81,0.9)' }}>{entry.from}</span>
                      <span className="text-[9px] font-mono" style={{ color: 'rgba(156,163,175,0.6)' }}>{entry.freq} MHz</span>
                    </div>
                    <p className="text-[9px] truncate" style={{ color: 'rgba(107,114,128,0.7)' }}>{entry.msg}</p>
                  </div>
                  <div className="text-right">
                    <span className="text-[9px] font-mono" style={{ color: 'rgba(156,163,175,0.5)' }}>{entry.time}</span>
                    {entry.signal !== 0 && (
                      <div className="text-[8px] font-mono" style={{ color: 'rgba(107,114,128,0.5)' }}>{entry.signal}dBm</div>
                    )}
                  </div>
                </motion.div>
              ))}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
}
