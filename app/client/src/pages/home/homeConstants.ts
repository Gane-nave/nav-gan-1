/**
 * G.A.N.E — Home Page Shared Constants
 * 
 * Color system, navigation modes, and sidebar group definitions
 * shared across all Home sub-components.
 * 
 * i18n: Uses TranslationKey references. Components call t(item.i18nKey) to get localized text.
 */
import {
  Hexagon, Bolt, Orbit, Radar, Crosshair,
  Diamond, Sparkles, CircuitBoard, Cpu, Fingerprint, Gem,
  Compass, BrainCircuit, Signal, Antenna, Pencil,
  Telescope, Rocket, ScanEye, Focus, Target,
  ShieldCheck, Waypoints, Flame, TowerControl,
  SatelliteDish, Gauge, Zap, Activity, History, Bell,
  Eye, Brain, Globe, Layers, Users, Radio, MapPin, Download
} from "lucide-react";
import type { SmartPanel, NavMode } from "@/lib/navStore";
import type { TranslationKey } from "@/lib/i18n";

// ═══════════════════════════════════════════════════════════
// CLEAN COLOR SYSTEM — Functional, readable, approachable
// ═══════════════════════════════════════════════════════════
export const COLORS = {
  cyan: '#2563EB',
  green: '#16A34A',
  red: '#DC2626',
  purple: '#7C3AED',
  orange: '#F97316',
  gold: '#FACC15',
  blue: '#2563EB',
  pink: '#EC4899',
  white: '#FFFFFF',
};

// ═══════════════════════════════════════════════════════════
// NAVIGATION MODES
// ═══════════════════════════════════════════════════════════
export const navModes: {
  id: NavMode;
  icon: typeof Hexagon;
  label: string;
  labelHe: string;
  i18nKey: TranslationKey;
  color: string;
  gradient: string;
}[] = [
  { id: 'drive', icon: Rocket, label: 'DRIVE', labelHe: 'נהיגה', i18nKey: 'mode.drive', color: COLORS.cyan, gradient: `linear-gradient(135deg, ${COLORS.cyan}, ${COLORS.blue})` },
  { id: 'walk', icon: Compass, label: 'WALK', labelHe: 'הליכה', i18nKey: 'mode.walk', color: COLORS.green, gradient: `linear-gradient(135deg, ${COLORS.green}, #00cc66)` },
  { id: 'emergency', icon: Flame, label: 'SOS', labelHe: 'חירום', i18nKey: 'mode.emergency', color: COLORS.red, gradient: `linear-gradient(135deg, ${COLORS.red}, #ff0033)` },
  { id: 'plan', icon: Waypoints, label: 'PLAN', labelHe: 'תכנון', i18nKey: 'mode.plan', color: COLORS.purple, gradient: `linear-gradient(135deg, ${COLORS.purple}, #7744dd)` },
];

// ═══════════════════════════════════════════════════════════
// SIDEBAR GROUP TYPES & DATA
// ═══════════════════════════════════════════════════════════
export type SidebarGroupType = {
  label: string;
  labelHe: string;
  i18nKey: TranslationKey;
  color: string;
  items: SidebarItemType[];
};

export type SidebarItemType = {
  id: SmartPanel;
  icon: typeof Hexagon;
  label: string;
  labelHe: string;
  i18nKey: TranslationKey;
  accent: string;
};

export const sidebarGroups: SidebarGroupType[] = [
  {
    label: 'NAV', labelHe: 'ניווט', i18nKey: 'group.nav', color: COLORS.cyan,
    items: [
      { id: 'map-layers', icon: Layers, label: 'Layers', labelHe: 'שכבות', i18nKey: 'sidebar.layers', accent: COLORS.blue },
      { id: 'parking', icon: Hexagon, label: 'Parking', labelHe: 'חניה', i18nKey: 'sidebar.parking', accent: COLORS.cyan },
      { id: 'charging', icon: Bolt, label: 'Charging', labelHe: 'טעינה', i18nKey: 'sidebar.charging', accent: COLORS.green },
      { id: 'multimodal', icon: Orbit, label: 'Transport', labelHe: 'תחבורה', i18nKey: 'sidebar.transport', accent: COLORS.purple },
      { id: 'indoor-pos', icon: Fingerprint, label: 'Indoor', labelHe: 'פנימי', i18nKey: 'sidebar.indoor', accent: COLORS.cyan },
      { id: 'ar-nav', icon: ScanEye, label: 'AR Nav', labelHe: 'AR', i18nKey: 'sidebar.arNav', accent: COLORS.orange },
      { id: 'route-history' as SmartPanel, icon: History, label: 'History', labelHe: 'היסטוריה', i18nKey: 'sidebar.history', accent: COLORS.orange },
      { id: 'offline-tiles' as SmartPanel, icon: Download, label: 'Offline Maps', labelHe: 'מפות אופליין', i18nKey: 'sidebar.offlineMaps', accent: COLORS.green },
      { id: 'drawing-tools' as SmartPanel, icon: Pencil, label: 'Draw', labelHe: 'ציור', i18nKey: 'sidebar.draw', accent: COLORS.pink },
    ],
  },
  {
    label: 'INTEL', labelHe: 'מודיעין', i18nKey: 'group.intel', color: COLORS.purple,
    items: [
      { id: 'weather', icon: Crosshair, label: 'Weather', labelHe: 'מזג אוויר', i18nKey: 'sidebar.weather', accent: COLORS.blue },
      { id: 'satellite', icon: Telescope, label: 'Satellite', labelHe: 'לוויין', i18nKey: 'sidebar.satellite', accent: COLORS.purple },
      { id: 'satellite-imagery' as SmartPanel, icon: Globe, label: 'Imagery', labelHe: 'תצלומי לוויין', i18nKey: 'sidebar.imagery', accent: COLORS.cyan },
      { id: 'gnss-manager', icon: SatelliteDish, label: 'GNSS', labelHe: 'GNSS', i18nKey: 'sidebar.gnss', accent: COLORS.green },
      { id: 'satellite-coverage' as SmartPanel, icon: Globe, label: 'Coverage', labelHe: 'כיסוי', i18nKey: 'sidebar.coverage', accent: COLORS.blue },
      { id: 'v2x', icon: Antenna, label: 'V2X', labelHe: 'V2X', i18nKey: 'sidebar.v2x', accent: COLORS.purple },
      { id: 'digital-twin', icon: Diamond, label: 'Twin', labelHe: 'תאום', i18nKey: 'sidebar.twin', accent: COLORS.cyan },
      { id: 'risk-engine', icon: ShieldCheck, label: 'Risk', labelHe: 'סיכון', i18nKey: 'sidebar.risk', accent: COLORS.red },
    ],
  },
  {
    label: 'OPS', labelHe: 'מבצעים', i18nKey: 'group.ops', color: COLORS.orange,
    items: [
      { id: 'command-center', icon: BrainCircuit, label: 'CMD', labelHe: 'פיקוד', i18nKey: 'sidebar.cmd', accent: COLORS.cyan },
      { id: 'eoc', icon: Radar, label: 'EOC', labelHe: 'חירום', i18nKey: 'sidebar.eoc', accent: COLORS.red },
      { id: 'fleet', icon: TowerControl, label: 'Fleet', labelHe: 'צי', i18nKey: 'sidebar.fleet', accent: COLORS.purple },
      { id: 'traffic-cam', icon: Focus, label: 'Cameras', labelHe: 'מצלמות', i18nKey: 'sidebar.cameras', accent: COLORS.orange },
      { id: 'evidence', icon: Crosshair, label: 'Report', labelHe: 'דיווח', i18nKey: 'sidebar.report', accent: COLORS.red },
      { id: 'incident-reporter' as SmartPanel, icon: Target, label: 'Incident', labelHe: 'אירוע', i18nKey: 'sidebar.incident', accent: COLORS.red },
      { id: 'smart-alerts' as SmartPanel, icon: Bell, label: 'Alerts', labelHe: 'התראות', i18nKey: 'sidebar.alerts', accent: COLORS.gold },
      { id: 'c4isr' as SmartPanel, icon: Crosshair, label: 'C4ISR', labelHe: 'פיקוד', i18nKey: 'sidebar.c4isr', accent: COLORS.red },
      { id: 'radio-comms' as SmartPanel, icon: Radio, label: 'Radio', labelHe: 'רדיו', i18nKey: 'sidebar.radio', accent: COLORS.orange },
    ],
  },
  {
    label: 'AI', labelHe: 'בינה', i18nKey: 'group.ai', color: COLORS.gold,
    items: [
      { id: 'ai-optimizer' as SmartPanel, icon: BrainCircuit, label: 'Optimizer', labelHe: 'אופטימיזציה', i18nKey: 'sidebar.optimizer', accent: COLORS.purple },
      { id: 'driver-score', icon: Gauge, label: 'Score', labelHe: 'ציון', i18nKey: 'sidebar.score', accent: COLORS.gold },
      { id: 'analytics', icon: Sparkles, label: 'Analytics', labelHe: 'אנליטיקה', i18nKey: 'sidebar.analytics', accent: COLORS.gold },
      { id: 'social-nav' as SmartPanel, icon: Orbit, label: 'Social', labelHe: 'חברתי', i18nKey: 'sidebar.social', accent: COLORS.pink },
      { id: 'live-sharing' as SmartPanel, icon: Signal, label: 'Share', labelHe: 'שיתוף', i18nKey: 'sidebar.liveShare', accent: COLORS.cyan },
      { id: 'collaboration' as SmartPanel, icon: Users, label: 'Collab', labelHe: 'שיתוף פעולה', i18nKey: 'sidebar.collab', accent: COLORS.cyan },
      { id: 'realtime-collab' as SmartPanel, icon: MapPin, label: 'Live Team', labelHe: 'צוות חי', i18nKey: 'sidebar.liveTeam', accent: COLORS.green },
    ],
  },
  {
    label: 'SYS', labelHe: 'מערכת', i18nKey: 'group.sys', color: COLORS.green,
    items: [
      { id: 'system-arch', icon: Cpu, label: 'Arch', labelHe: 'ארכיטקטורה', i18nKey: 'sidebar.arch', accent: COLORS.green },
      { id: 'data-pipeline' as SmartPanel, icon: Activity, label: 'Pipeline', labelHe: 'צנרת', i18nKey: 'sidebar.pipeline', accent: COLORS.blue },
      { id: 'spec-vault', icon: CircuitBoard, label: 'Spec', labelHe: 'מפרט', i18nKey: 'sidebar.spec', accent: COLORS.purple },
      { id: 'payments', icon: Gem, label: 'Wallet', labelHe: 'ארנק', i18nKey: 'sidebar.wallet', accent: COLORS.gold },
      { id: 'offline-mode' as SmartPanel, icon: Fingerprint, label: 'Offline', labelHe: 'אופליין', i18nKey: 'sidebar.offline', accent: COLORS.green },
      { id: 'accessibility' as SmartPanel, icon: Eye, label: 'A11y', labelHe: 'נגישות', i18nKey: 'sidebar.a11y', accent: COLORS.green },
      { id: 'gane-status' as SmartPanel, icon: Brain, label: 'G.A.N.E', labelHe: 'מנוע', i18nKey: 'sidebar.engine', accent: COLORS.cyan },
      { id: 'battery-status' as SmartPanel, icon: Zap, label: 'Battery', labelHe: 'סוללה', i18nKey: 'sidebar.battery', accent: COLORS.green },
    ],
  },
];
