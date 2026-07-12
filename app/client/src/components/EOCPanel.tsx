/**
 * G.A.N.E — Emergency Operations Center (EOC) Panel
 * Full command & control dashboard for MDA, Police, Fire departments.
 * Real-time incident management, resource allocation, corridor control,
 * and live situation awareness for emergency response.
 */
import { useState, useEffect, useCallback, useRef } from "react";
import { useRealDataContext } from "@/contexts/RealDataContext";
import { motion, AnimatePresence } from "framer-motion";
import {
  Flame, ShieldCheck, Phone, Antenna, MapPin, Clock,
  Users, TowerControl, Heart, Radar, ScanEye, Navigation,
  ChevronRight, ChevronDown, Signal, Zap, X,
  Fingerprint, WifiOff, Target, Crosshair, Sparkles,
  ArrowUpRight, ArrowDownRight, Minus, Bell,
  CircleDot, Layers, Send, MessageSquare, Volume2
} from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";

type IncidentSeverity = 'critical' | 'high' | 'medium' | 'low';
type IncidentType = 'accident' | 'fire' | 'medical' | 'security' | 'hazmat' | 'infrastructure';
type ResourceStatus = 'deployed' | 'en-route' | 'available' | 'offline';
type EOCTab = 'situation' | 'resources' | 'corridors' | 'comms';

interface Incident {
  id: string;
  type: IncidentType;
  severity: IncidentSeverity;
  title: string;
  titleHe: string;
  location: string;
  time: string;
  responders: number;
  casualties?: number;
  status: 'active' | 'contained' | 'resolved';
  lat: number;
  lng: number;
}

interface Resource {
  id: string;
  type: 'ambulance' | 'police' | 'fire' | 'helicopter' | 'command';
  callSign: string;
  status: ResourceStatus;
  location: string;
  eta?: string;
  assignedTo?: string;
  crew: number;
}

interface Corridor {
  id: string;
  name: string;
  direction: 'inbound' | 'outbound' | 'both';
  status: 'open' | 'restricted' | 'closed' | 'emergency-only';
  flow: number; // vehicles per minute
  avgSpeed: number;
  incidents: number;
}

const severityConfig: Record<IncidentSeverity, { color: string; label: string; i18nKey?: string; pulse: boolean }> = {
  critical: { color: 'oklch(0.65 0.22 25)', label: 'CRITICAL', i18nKey: 'severity.critical', pulse: true },
  high: { color: 'oklch(0.70 0.20 40)', label: 'HIGH', i18nKey: 'severity.high', pulse: true },
  medium: { color: 'oklch(0.80 0.16 75)', label: 'MEDIUM', i18nKey: 'severity.medium', pulse: false },
  low: { color: 'oklch(0.75 0.18 150)', label: 'LOW', i18nKey: 'severity.low', pulse: false } };

const typeIcons: Record<IncidentType, typeof Flame> = {
  accident: Flame,
  fire: Flame,
  medical: Heart,
  security: ShieldCheck,
  hazmat: Zap,
  infrastructure: Layers };

export default function EOCPanel({ onClose }: { onClose: () => void }) {
  const { t, dir, lang } = useLanguage();
  const [activeTab, setActiveTab] = useState<EOCTab>('situation');
  const [alertLevel, setAlertLevel] = useState<'green' | 'yellow' | 'orange' | 'red'>('yellow');
  const [expandedIncident, setExpandedIncident] = useState<string | null>(null);
  const [liveTimer, setLiveTimer] = useState(0);

  // Consolidated timer: uptime + corridor flow in one interval
  useEffect(() => {
    let tick = 0;
    const interval = setInterval(() => {
      tick++;
      setLiveTimer(t => t + 1);
      // Update corridor flow every 4 seconds
      if (tick % 4 === 0) {
        setCorridors(prev => prev.map(c => ({
          ...c,
          flow: Math.max(0, c.flow + Math.round((Math.random() - 0.5) * 6)),
          avgSpeed: Math.max(0, Math.min(80, c.avgSpeed + Math.round((Math.random() - 0.5) * 4))) })));
      }
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const formatUptime = (seconds: number) => {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  };

  const { weather, earthquakes } = useRealDataContext();

  // Live incidents — generated from real USGS earthquake data + weather + simulated local events
  const [incidents, setIncidents] = useState<Incident[]>([]);
  const incidentsGenerated = useRef(false);

  useEffect(() => {
    if (incidentsGenerated.current) return;
    const liveIncidents: Incident[] = [];

    // Convert real USGS earthquakes to incidents
    if (earthquakes && earthquakes.length > 0) {
      earthquakes.slice(0, 2).forEach((eq: any, i: number) => {
        liveIncidents.push({
          id: `EQ-${i + 1}`,
          type: 'infrastructure',
          severity: eq.magnitude > 5 ? 'critical' : eq.magnitude > 3 ? 'high' : 'medium',
          title: `Earthquake M${eq.magnitude.toFixed(1)}`,
          titleHe: `רעידת אדמה ${eq.magnitude.toFixed(1)}`,
          location: eq.place || 'Unknown',
          time: new Date(eq.time).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
          responders: Math.ceil(eq.magnitude * 2),
          casualties: eq.magnitude > 5 ? Math.floor(Math.random() * 5) : undefined,
          status: 'active',
          lat: eq.coordinates?.[1] || 32.07,
          lng: eq.coordinates?.[0] || 34.79 });
      });
    }

    // Weather-based incidents
    if (weather && weather.precipitation > 2) {
      liveIncidents.push({
        id: 'WX-001',
        type: 'infrastructure',
        severity: weather.precipitation > 10 ? 'high' : 'medium',
        title: 'Flooding Risk — Heavy Rain',
        titleHe: 'סכנת הצפה — גשם כבד',
        location: 'Greater Tel Aviv area',
        time: 'Active now',
        responders: 4,
        status: 'active',
        lat: 32.07, lng: 34.78 });
    }

    // Always add simulated local incidents for realism
    liveIncidents.push(
      { id: 'INC-001', type: 'accident', severity: 'critical', title: 'Multi-Vehicle Collision', titleHe: 'תאונה רב-רכבית', location: 'Ayalon Hwy N, km 12.4', time: '3 min ago', responders: 8, casualties: 4, status: 'active', lat: 32.07, lng: 34.79 },
      { id: 'INC-002', type: 'fire', severity: 'high', title: 'Vehicle Fire', titleHe: 'שריפת רכב', location: 'Begin Road / HaShalom', time: '12 min ago', responders: 5, status: 'contained', lat: 32.072, lng: 34.788 },
      { id: 'INC-003', type: 'medical', severity: 'medium', title: 'Medical Emergency', titleHe: 'אירוע רפואי', location: 'Dizengoff Center', time: '28 min ago', responders: 2, casualties: 1, status: 'active', lat: 32.075, lng: 34.774 },
      { id: 'INC-005', type: 'security', severity: 'high', title: 'Suspicious Object', titleHe: 'חפץ חשוד', location: 'Arlozorov Station', time: '8 min ago', responders: 6, status: 'active', lat: 32.083, lng: 34.782 },
    );

    setIncidents(liveIncidents);
    incidentsGenerated.current = true;
  }, [earthquakes, weather]);

  // Live resource updates — responder counts fluctuate
  const [resources, setResources] = useState<Resource[]>([
    { id: 'R-101', type: 'ambulance', callSign: 'MDA-101', status: 'deployed', location: 'Ayalon Hwy N', assignedTo: 'INC-001', crew: 3 },
    { id: 'R-102', type: 'ambulance', callSign: 'MDA-102', status: 'deployed', location: 'Ayalon Hwy N', assignedTo: 'INC-001', crew: 2 },
    { id: 'R-103', type: 'ambulance', callSign: 'MDA-103', status: 'en-route', location: 'Ibn Gabirol', eta: '4 min', assignedTo: 'INC-001', crew: 3 },
    { id: 'R-201', type: 'police', callSign: 'POL-201', status: 'deployed', location: 'Begin / HaShalom', assignedTo: 'INC-002', crew: 2 },
    { id: 'R-202', type: 'police', callSign: 'POL-202', status: 'deployed', location: 'Arlozorov Station', assignedTo: 'INC-005', crew: 4 },
    { id: 'R-301', type: 'fire', callSign: 'FD-301', status: 'deployed', location: 'Begin Road', assignedTo: 'INC-002', crew: 6 },
    { id: 'R-401', type: 'helicopter', callSign: 'HELI-01', status: 'en-route', location: 'Airborne — ETA Ayalon', eta: '6 min', assignedTo: 'INC-001', crew: 4 },
    { id: 'R-501', type: 'ambulance', callSign: 'MDA-104', status: 'available', location: 'Ichilov Hospital', crew: 2 },
    { id: 'R-502', type: 'police', callSign: 'POL-203', status: 'available', location: 'HaYarkon Station', crew: 2 },
    { id: 'R-601', type: 'command', callSign: 'CMD-01', status: 'deployed', location: 'Ayalon Hwy N', assignedTo: 'INC-001', crew: 3 },
  ]);

  // Live corridor data — flow and speed update in real-time
  const [corridors, setCorridors] = useState<Corridor[]>([
    { id: 'C-01', name: 'Ayalon Highway North', direction: 'inbound', status: 'closed', flow: 0, avgSpeed: 0, incidents: 1 },
    { id: 'C-02', name: 'Ayalon Highway South', direction: 'outbound', status: 'restricted', flow: 12, avgSpeed: 25, incidents: 0 },
    { id: 'C-03', name: 'Begin Road', direction: 'both', status: 'restricted', flow: 18, avgSpeed: 30, incidents: 1 },
    { id: 'C-04', name: 'Namir Road', direction: 'both', status: 'open', flow: 45, avgSpeed: 48, incidents: 1 },
    { id: 'C-05', name: 'Ibn Gabirol', direction: 'both', status: 'open', flow: 38, avgSpeed: 42, incidents: 0 },
    { id: 'C-06', name: 'Kaplan Street', direction: 'both', status: 'emergency-only', flow: 2, avgSpeed: 60, incidents: 0 },
  ]);

  // Corridor flow is now updated in the consolidated timer above

  // Auto-escalate alert level based on incidents
  useEffect(() => {
    const criticals = incidents.filter(i => i.severity === 'critical' && i.status === 'active').length;
    const highs = incidents.filter(i => i.severity === 'high' && i.status === 'active').length;
    if (criticals >= 2) setAlertLevel('red');
    else if (criticals >= 1 || highs >= 2) setAlertLevel('orange');
    else if (highs >= 1) setAlertLevel('yellow');
    else setAlertLevel('green');
  }, [incidents]);

  const activeIncidents = incidents.filter(i => i.status === 'active').length;
  const deployedResources = resources.filter(r => r.status === 'deployed' || r.status === 'en-route').length;
  const availableResources = resources.filter(r => r.status === 'available').length;

  const tabs: { id: EOCTab; label: string; labelEn: string; icon: typeof Signal }[] = [
    { id: 'situation', label: 'מצב', labelEn: 'Situation', icon: ScanEye },
    { id: 'resources', label: 'משאבים', labelEn: 'Resources', icon: TowerControl },
    { id: 'corridors', label: 'צירים', labelEn: 'Corridors', icon: Navigation },
    { id: 'comms', label: 'תקשורת', labelEn: 'Comms', icon: Antenna },
  ];

  const alertColors = {
    green: { color: 'oklch(0.75 0.18 150)', label: 'GREEN', i18nKey: 'alert.green' },
    yellow: { color: 'oklch(0.80 0.16 75)', label: 'YELLOW', i18nKey: 'alert.yellow' },
    orange: { color: 'oklch(0.70 0.20 40)', label: 'ORANGE', i18nKey: 'alert.orange' },
    red: { color: 'oklch(0.65 0.22 25)', label: 'RED', i18nKey: 'alert.red' } };

  const resourceTypeConfig: Record<string, { icon: typeof Heart; color: string; label: string; labelEn?: string }> = {
    ambulance: { icon: Heart, color: 'oklch(0.65 0.22 25)', label: 'אמבולנס', labelEn: 'Ambulance' },
    police: { icon: ShieldCheck, color: 'oklch(0.55 0.22 264)', label: t('incident.police') },
    fire: { icon: Flame, color: 'oklch(0.70 0.20 40)', label: 'כיבוי', labelEn: 'Fire Dept' },
    helicopter: { icon: Navigation, color: 'oklch(0.82 0.15 192)', label: 'מסוק', labelEn: 'Helicopter' },
    command: { icon: Antenna, color: 'oklch(0.80 0.16 75)', label: t('sidebar.cmd') } };

  const corridorStatusConfig: Record<string, { color: string; label: string; labelEn?: string }> = {
    open: { color: 'oklch(0.75 0.18 150)', label: 'פתוח', labelEn: 'Open' },
    restricted: { color: 'oklch(0.80 0.16 75)', label: 'מוגבל', labelEn: 'Restricted' },
    closed: { color: 'oklch(0.65 0.22 25)', label: t('common.close') },
    'emergency-only': { color: 'oklch(0.55 0.22 264)', label: 'חירום בלבד', labelEn: 'Emergency Only' } };

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: "spring", damping: 30, stiffness: 320 }}
      className="expand-panel"
    >
      {/* ═══ HEADER — Alert Level + Live Timer ═══ */}
      <div className="sticky top-0 z-10 px-5 pt-5 pb-3" style={{
        background: 'linear-gradient(180deg, oklch(0.09 0.015 264 / 98%) 0%, oklch(0.09 0.015 264 / 90%) 80%, transparent 100%)' }}>
        <div className="flex items-center gap-3">
          <motion.div
            initial={{ scale: 0.8 }}
            animate={{ scale: [1, 1.1, 1] }}
            transition={{ repeat: Infinity, duration: 2 }}
            className="w-9 h-9 rounded-xl flex items-center justify-center"
            style={{
              background: `color-mix(in oklch, ${alertColors[alertLevel].color}, transparent 85%)`,
              border: `1px solid color-mix(in oklch, ${alertColors[alertLevel].color}, transparent 60%)`,
              boxShadow: `0 0 20px color-mix(in oklch, ${alertColors[alertLevel].color}, transparent 70%)` }}
          >
            <Radar className="w-5 h-5" style={{ color: alertColors[alertLevel].color }} />
          </motion.div>
          <div className="flex-1">
            <h2 className="text-base font-bold text-white/90" style={{ fontFamily: 'Syne, sans-serif' }}>
              מרכז פיקוד
            </h2>
            <div className="flex items-center gap-2 mt-0.5">
              <div className="flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-bold"
                style={{
                  background: `color-mix(in oklch, ${alertColors[alertLevel].color}, transparent 88%)`,
                  color: alertColors[alertLevel].color }}>
                {alertColors[alertLevel].label}
              </div>
              <span className="text-[10px] text-white/30 metric-value">{alertColors[alertLevel].i18nKey}</span>
            </div>
          </div>
          <div className="text-right">
            <div className="text-[10px] text-white/20 uppercase tracking-wider">LIVE</div>
            <div className="text-xs metric-value text-gane-red flex items-center gap-1">
              <div className="w-1.5 h-1.5 rounded-full bg-gane-red glow-dot" />
              {formatUptime(liveTimer)}
            </div>
          </div>
          <button onClick={onClose} className="w-8 h-8 rounded-lg flex items-center justify-center text-white/20 hover:text-white/60 hover:bg-white/5 transition-all duration-200 hover:rotate-90">
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Quick stats bar */}
        <div className="grid grid-cols-4 gap-2 mt-3">
          {[
            { label: 'אירועים', labelEn: 'Incidents', value: activeIncidents, color: 'oklch(0.65 0.22 25)' },
            { label: 'פרוסים', labelEn: 'Deployed', value: deployedResources, color: 'oklch(0.80 0.16 75)' },
            { label: 'זמינים', labelEn: 'Available', value: availableResources, color: 'oklch(0.75 0.18 150)' },
            { label: 'צירים', labelEn: 'Corridors', value: corridors.filter(c => c.status !== 'open').length, color: 'oklch(0.55 0.22 264)' },
          ].map((stat, idx) => (
            <motion.div key={idx}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.1 + idx * 0.05 }}
              className="text-center py-2 rounded-lg"
              style={{ background: `color-mix(in oklch, ${stat.color}, transparent 92%)` }}
            >
              <div className="text-lg font-bold metric-value" style={{ color: stat.color }}>{stat.value}</div>
              <div className="text-[9px] text-white/30">{lang === 'he' ? stat.label : stat.labelEn}</div>
            </motion.div>
          ))}
        </div>

        {/* Tab bar */}
        <div className="flex gap-1 mt-3 p-0.5 rounded-xl" style={{ background: 'oklch(1 0 0 / 3%)' }}>
          {tabs.map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`flex-1 flex items-center justify-center gap-1.5 py-2 rounded-lg text-[11px] font-medium transition-all duration-200 ${
                activeTab === tab.id ? 'text-white/90' : 'text-white/30 hover:text-white/50'
              }`}
              style={activeTab === tab.id ? {
                background: 'oklch(1 0 0 / 8%)',
                boxShadow: '0 2px 8px oklch(0 0 0 / 30%)' } : undefined}
            >
              <tab.icon className="w-3.5 h-3.5" />
              {lang === 'he' ? tab.label : tab.labelEn}
            </button>
          ))}
        </div>

        <div className="mt-3 h-px w-full" style={{ background: `linear-gradient(90deg, ${alertColors[alertLevel].color}, transparent 80%)`, opacity: 0.15 }} />
      </div>

      {/* ═══ CONTENT ═══ */}
      <div className="px-5 pb-6">
        <AnimatePresence mode="wait">
          {/* ─── SITUATION TAB ─── */}
          {activeTab === 'situation' && (
            <motion.div key="situation" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              <div className="text-xs text-white/30 uppercase tracking-wider mb-2 mt-2 flex items-center gap-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                <ScanEye className="w-3 h-3" /> אירועים פעילים
              </div>
              <div className="space-y-2">
                {incidents.map((incident, idx) => {
                  const sev = severityConfig[incident.severity];
                  const TypeIcon = typeIcons[incident.type];
                  const isExpanded = expandedIncident === incident.id;
                  return (
                    <motion.div
                      key={incident.id}
                      initial={{ opacity: 0, x: -20 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ delay: idx * 0.06 }}
                      className="feature-card cursor-pointer"
                      onClick={() => setExpandedIncident(isExpanded ? null : incident.id)}
                    >
                      <div className="flex items-start gap-3">
                        <div className={`w-9 h-9 rounded-xl flex items-center justify-center flex-shrink-0 ${sev.pulse ? 'glow-dot' : ''}`}
                          style={{
                            background: `color-mix(in oklch, ${sev.color}, transparent 85%)`,
                            border: `1px solid color-mix(in oklch, ${sev.color}, transparent 70%)`,
                            color: sev.color }}>
                          <TypeIcon className="w-4.5 h-4.5" />
                        </div>
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2">
                            <span className="text-sm font-semibold text-white/90 truncate">{lang === 'he' ? incident.titleHe : incident.title}</span>
                            <span className="text-[9px] font-bold px-1.5 py-0.5 rounded" style={{
                              background: `color-mix(in oklch, ${sev.color}, transparent 88%)`,
                              color: sev.color }}>{sev.i18nKey}</span>
                          </div>
                          <div className="text-[10px] text-white/35 mt-0.5">{incident.title}</div>
                          <div className="flex items-center gap-3 mt-1">
                            <span className="text-[10px] text-white/25 flex items-center gap-1">
                              <MapPin className="w-2.5 h-2.5" /> {incident.location}
                            </span>
                            <span className="text-[10px] text-white/25 flex items-center gap-1">
                              <Clock className="w-2.5 h-2.5" /> {incident.time}
                            </span>
                          </div>
                        </div>
                        <div className="flex-shrink-0">
                          <ChevronDown className={`w-4 h-4 text-white/20 transition-transform duration-200 ${isExpanded ? 'rotate-180' : ''}`} />
                        </div>
                      </div>

                      {/* Expanded details */}
                      <AnimatePresence>
                        {isExpanded && (
                          <motion.div
                            initial={{ height: 0, opacity: 0 }}
                            animate={{ height: 'auto', opacity: 1 }}
                            exit={{ height: 0, opacity: 0 }}
                            transition={{ duration: 0.2 }}
                            className="overflow-hidden"
                          >
                            <div className="mt-3 pt-3 border-t border-white/5">
                              <div className="grid grid-cols-3 gap-2 mb-3">
                                <div className="text-center py-2 rounded-lg bg-white/3">
                                  <div className="text-sm font-bold metric-value text-gane-cyan">{incident.responders}</div>
                                  <div className="text-[9px] text-white/25">מגיבים</div>
                                </div>
                                {incident.casualties !== undefined && (
                                  <div className="text-center py-2 rounded-lg bg-white/3">
                                    <div className="text-sm font-bold metric-value text-gane-red">{incident.casualties}</div>
                                    <div className="text-[9px] text-white/25">נפגעים</div>
                                  </div>
                                )}
                                <div className="text-center py-2 rounded-lg bg-white/3">
                                  <div className="text-sm font-bold metric-value" style={{ color: incident.status === 'active' ? 'oklch(0.65 0.22 25)' : incident.status === 'contained' ? 'oklch(0.80 0.16 75)' : 'oklch(0.75 0.18 150)' }}>
                                    {incident.status === 'active' ? 'פעיל' : incident.status === 'contained' ? 'מוכל' : 'טופל'}
                                  </div>
                                  <div className="text-[9px] text-white/25">סטטוס</div>
                                </div>
                              </div>
                              {/* Assigned resources */}
                              <div className="text-[10px] text-white/25 mb-1">משאבים מוקצים:</div>
                              <div className="flex flex-wrap gap-1">
                                {resources.filter(r => r.assignedTo === incident.id).map(r => {
                                  const rc = resourceTypeConfig[r.type];
                                  return (
                                    <div key={r.id} className="flex items-center gap-1 px-2 py-1 rounded-lg text-[10px]"
                                      style={{ background: `color-mix(in oklch, ${rc.color}, transparent 90%)`, color: rc.color }}>
                                      <rc.icon className="w-2.5 h-2.5" />
                                      {r.callSign}
                                      {r.status === 'en-route' && <span className="text-[8px] opacity-60">({r.eta})</span>}
                                    </div>
                                  );
                                })}
                              </div>
                              {/* Action buttons */}
                              <div className="flex gap-2 mt-3">
                                <button className="flex-1 py-2 rounded-lg text-[10px] font-medium transition-all hover:brightness-110"
                                  style={{ background: 'oklch(0.82 0.15 192 / 12%)', color: 'oklch(0.82 0.15 192)', border: '1px solid oklch(0.82 0.15 192 / 20%)' }}>
                                  <Target className="w-3 h-3 inline mr-1" /> מיקוד מפה
                                </button>
                                <button className="flex-1 py-2 rounded-lg text-[10px] font-medium transition-all hover:brightness-110"
                                  style={{ background: 'oklch(0.65 0.22 25 / 12%)', color: 'oklch(0.65 0.22 25)', border: '1px solid oklch(0.65 0.22 25 / 20%)' }}>
                                  <Send className="w-3 h-3 inline mr-1" /> שלח כוחות
                                </button>
                              </div>
                            </div>
                          </motion.div>
                        )}
                      </AnimatePresence>
                    </motion.div>
                  );
                })}
              </div>
            </motion.div>
          )}

          {/* ─── RESOURCES TAB ─── */}
          {activeTab === 'resources' && (
            <motion.div key="resources" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              {/* Resource summary */}
              <div className="grid grid-cols-5 gap-1.5 mt-2 mb-4">
                {Object.entries(resourceTypeConfig).map(([type, config]) => {
                  const count = resources.filter(r => r.type === type).length;
                  const deployed = resources.filter(r => r.type === type && (r.status === 'deployed' || r.status === 'en-route')).length;
                  return (
                    <div key={type} className="text-center py-2 rounded-lg" style={{ background: `color-mix(in oklch, ${config.color}, transparent 92%)` }}>
                      <config.icon className="w-4 h-4 mx-auto mb-1" style={{ color: config.color }} />
                      <div className="text-xs font-bold metric-value" style={{ color: config.color }}>{deployed}/{count}</div>
                      <div className="text-[8px] text-white/25">{config.label}</div>
                    </div>
                  );
                })}
              </div>

              <div className="text-xs text-white/30 uppercase tracking-wider mb-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                <TowerControl className="w-3 h-3 inline mr-1" /> כל המשאבים
              </div>
              <div className="space-y-1.5">
                {resources.map((resource, idx) => {
                  const rc = resourceTypeConfig[resource.type];
                  const statusColors: Record<ResourceStatus, string> = {
                    deployed: 'oklch(0.65 0.22 25)',
                    'en-route': 'oklch(0.80 0.16 75)',
                    available: 'oklch(0.75 0.18 150)',
                    offline: 'oklch(0.40 0.01 264)' };
                  const statusLabels: Record<ResourceStatus, string> = {
                    deployed: 'פרוס',
                    'en-route': 'בדרך',
                    available: 'זמין',
                    offline: 'לא פעיל' };
                  return (
                    <motion.div key={resource.id}
                      initial={{ opacity: 0, x: -15 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ delay: idx * 0.04 }}
                      className="flex items-center gap-3 px-3 py-2.5 rounded-xl hover:bg-white/3 transition-colors"
                    >
                      <div className="w-8 h-8 rounded-lg flex items-center justify-center"
                        style={{ background: `color-mix(in oklch, ${rc.color}, transparent 88%)` }}>
                        <rc.icon className="w-4 h-4" style={{ color: rc.color }} />
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2">
                          <span className="text-xs font-bold text-white/80 metric-value">{resource.callSign}</span>
                          <span className="text-[9px] px-1.5 py-0.5 rounded font-medium"
                            style={{ background: `color-mix(in oklch, ${statusColors[resource.status]}, transparent 88%)`, color: statusColors[resource.status] }}>
                            {statusLabels[resource.status]}
                          </span>
                        </div>
                        <div className="text-[10px] text-white/25 truncate">{resource.location}</div>
                      </div>
                      <div className="text-right flex-shrink-0">
                        {resource.eta && <div className="text-[10px] metric-value text-gane-amber">{resource.eta}</div>}
                        <div className="text-[9px] text-white/20">{resource.crew} צוות</div>
                      </div>
                    </motion.div>
                  );
                })}
              </div>
            </motion.div>
          )}

          {/* ─── CORRIDORS TAB ─── */}
          {activeTab === 'corridors' && (
            <motion.div key="corridors" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              <div className="text-xs text-white/30 uppercase tracking-wider mb-2 mt-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                <Navigation className="w-3 h-3 inline mr-1" /> ניהול צירים
              </div>
              <div className="space-y-2">
                {corridors.map((corridor, idx) => {
                  const cs = corridorStatusConfig[corridor.status];
                  return (
                    <motion.div key={corridor.id}
                      initial={{ opacity: 0, x: -20 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ delay: idx * 0.06 }}
                      className="feature-card"
                    >
                      <div className="flex items-center justify-between mb-2">
                        <div className="flex-1">
                          <div className="text-sm font-semibold text-white/90">{corridor.name}</div>
                          <div className="flex items-center gap-2 mt-0.5">
                            <span className="text-[9px] px-1.5 py-0.5 rounded font-bold"
                              style={{ background: `color-mix(in oklch, ${cs.color}, transparent 88%)`, color: cs.color }}>
                              {cs.label}
                            </span>
                            <span className="text-[10px] text-white/25">
                              {corridor.direction === 'inbound' ? '← נכנס' : corridor.direction === 'outbound' ? '→ יוצא' : '↔ דו-כיווני'}
                            </span>
                          </div>
                        </div>
                        {corridor.incidents > 0 && (
                          <div className="flex items-center gap-1 px-2 py-1 rounded-lg text-[10px] font-medium"
                            style={{ background: 'oklch(0.65 0.22 25 / 12%)', color: 'oklch(0.65 0.22 25)' }}>
                            <Flame className="w-3 h-3" /> {corridor.incidents}
                          </div>
                        )}
                      </div>
                      {/* Flow metrics */}
                      <div className="grid grid-cols-2 gap-2">
                        <div className="py-1.5 px-2 rounded-lg bg-white/3 text-center">
                          <div className="text-xs font-bold metric-value" style={{ color: corridor.flow > 30 ? 'oklch(0.75 0.18 150)' : corridor.flow > 10 ? 'oklch(0.80 0.16 75)' : 'oklch(0.65 0.22 25)' }}>
                            {corridor.flow}
                          </div>
                          <div className="text-[8px] text-white/20">רכב/דקה</div>
                        </div>
                        <div className="py-1.5 px-2 rounded-lg bg-white/3 text-center">
                          <div className="text-xs font-bold metric-value" style={{ color: corridor.avgSpeed > 40 ? 'oklch(0.75 0.18 150)' : corridor.avgSpeed > 20 ? 'oklch(0.80 0.16 75)' : 'oklch(0.65 0.22 25)' }}>
                            {corridor.avgSpeed} km/h
                          </div>
                          <div className="text-[8px] text-white/20">מהירות ממוצעת</div>
                        </div>
                      </div>
                      {/* Action buttons */}
                      <div className="flex gap-1.5 mt-2">
                        {corridor.status !== 'closed' && (
                          <button className="flex-1 py-1.5 rounded-lg text-[9px] font-medium bg-gane-red/10 text-gane-red border border-gane-red/15 hover:brightness-110 transition-all">
                            סגור ציר
                          </button>
                        )}
                        {corridor.status !== 'open' && (
                          <button className="flex-1 py-1.5 rounded-lg text-[9px] font-medium bg-gane-green/10 text-gane-green border border-gane-green/15 hover:brightness-110 transition-all">
                            פתח ציר
                          </button>
                        )}
                        <button className="flex-1 py-1.5 rounded-lg text-[9px] font-medium bg-gane-indigo/10 text-gane-indigo border border-gane-indigo/15 hover:brightness-110 transition-all">
                          חירום בלבד
                        </button>
                      </div>
                    </motion.div>
                  );
                })}
              </div>
            </motion.div>
          )}

          {/* ─── COMMS TAB ─── */}
          {activeTab === 'comms' && (
            <motion.div key="comms" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              <div className="text-xs text-white/30 uppercase tracking-wider mb-2 mt-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                <Antenna className="w-3 h-3 inline mr-1" /> ערוצי תקשורת
              </div>

              {/* Active channels */}
              <div className="space-y-2 mb-4">
                {[
                  { name: 'ערוץ פיקוד ראשי', freq: '154.650 MHz', active: true, users: 12, encrypted: true },
                  { name: 'מד"א — אירוע 001', freq: '155.100 MHz', active: true, users: 8, encrypted: true },
                  { name: 'משטרה — אירוע 005', freq: '156.200 MHz', active: true, users: 6, encrypted: true },
                  { name: 'כיבוי — אירוע 002', freq: '157.300 MHz', active: true, users: 5, encrypted: true },
                  { name: 'תיאום בין-גופי', freq: '158.000 MHz', active: false, users: 0, encrypted: true },
                ].map((channel, idx) => (
                  <motion.div key={idx}
                    initial={{ opacity: 0, x: -15 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: idx * 0.05 }}
                    className="feature-card flex items-center gap-3"
                  >
                    <div className={`w-8 h-8 rounded-lg flex items-center justify-center ${channel.active ? 'bg-gane-green/10' : 'bg-white/5'}`}>
                      {channel.active ? (
                        <Volume2 className="w-4 h-4 text-gane-green" />
                      ) : (
                        <WifiOff className="w-4 h-4 text-white/20" />
                      )}
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="text-xs font-semibold text-white/80">{channel.name}</div>
                      <div className="flex items-center gap-2 mt-0.5">
                        <span className="text-[10px] text-white/25 metric-value">{channel.freq}</span>
                        {channel.encrypted && <ShieldCheck className="w-2.5 h-2.5 text-gane-green" />}
                      </div>
                    </div>
                    <div className="text-right">
                      {channel.active && (
                        <div className="flex items-center gap-1 text-[10px] text-gane-green">
                          <Users className="w-3 h-3" /> {channel.users}
                        </div>
                      )}
                    </div>
                  </motion.div>
                ))}
              </div>

              {/* Quick broadcast */}
              <div className="feature-card">
                <div className="text-xs text-white/30 uppercase tracking-wider mb-2">שידור מהיר</div>
                <div className="grid grid-cols-2 gap-2">
                  <button className="py-3 rounded-xl text-xs font-medium bg-gane-red/10 text-gane-red border border-gane-red/15 hover:brightness-110 transition-all flex flex-col items-center gap-1">
                    <Bell className="w-4 h-4" />
                    התראת חירום
                  </button>
                  <button className="py-3 rounded-xl text-xs font-medium bg-gane-amber/10 text-gane-amber border border-gane-amber/15 hover:brightness-110 transition-all flex flex-col items-center gap-1">
                    <MessageSquare className="w-4 h-4" />
                    הודעה כללית
                  </button>
                  <button className="py-3 rounded-xl text-xs font-medium bg-gane-cyan/10 text-gane-cyan border border-gane-cyan/15 hover:brightness-110 transition-all flex flex-col items-center gap-1">
                    <Navigation className="w-4 h-4" />
                    הכוונת צירים
                  </button>
                  <button className="py-3 rounded-xl text-xs font-medium bg-gane-indigo/10 text-gane-indigo border border-gane-indigo/15 hover:brightness-110 transition-all flex flex-col items-center gap-1">
                    <Phone className="w-4 h-4" />
                    שיחת ועידה
                  </button>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
}
