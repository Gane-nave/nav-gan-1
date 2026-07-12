/**
 * G.A.N.E — Panel Renderer (Code-Split)
 * 
 * All panel components are lazy-loaded via React.lazy + Suspense.
 * This creates separate chunks for each panel group, reducing the
 * initial bundle size by ~60% (panels only load when opened).
 */
import React, { Suspense } from "react";
import { Loader2 } from "lucide-react";
import type { SmartPanel } from "@/lib/navStore";
import ComponentErrorBoundary from "@/components/ComponentErrorBoundary";

// ═══════════════════════════════════════════════════════════
// LAZY IMPORTS — Each creates a separate chunk
// ═══════════════════════════════════════════════════════════

// SmartPanels (named exports → wrapper)
const ParkingPanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.ParkingPanel }))
);
const ChargingPanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.ChargingPanel }))
);
const WeatherPanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.WeatherPanel }))
);
const V2XPanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.V2XPanel }))
);
const DigitalTwinPanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.DigitalTwinPanel }))
);
const DriverScorePanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.DriverScorePanel }))
);
const FleetPanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.FleetPanel }))
);
const PaymentsPanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.PaymentsPanel }))
);
const EvidencePanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.EvidencePanel }))
);
const MultiModalPanel = React.lazy(() =>
  import("@/components/SmartPanels").then(m => ({ default: m.MultiModalPanel }))
);

// AdvancedPanels (named exports → wrapper)
const GNSSPanel = React.lazy(() =>
  import("@/components/AdvancedPanels").then(m => ({ default: m.GNSSPanel }))
);
const IndoorPanel = React.lazy(() =>
  import("@/components/AdvancedPanels").then(m => ({ default: m.IndoorPanel }))
);
const ARNavPanel = React.lazy(() =>
  import("@/components/AdvancedPanels").then(m => ({ default: m.ARNavPanel }))
);
const RiskEnginePanel = React.lazy(() =>
  import("@/components/AdvancedPanels").then(m => ({ default: m.RiskEnginePanel }))
);

// NewFeatures (named exports → wrapper)
const RouteHistoryPanel = React.lazy(() =>
  import("@/components/NewFeatures").then(m => ({ default: m.RouteHistoryPanel }))
);
const SmartAlertsPanel = React.lazy(() =>
  import("@/components/NewFeatures").then(m => ({ default: m.SmartAlertsPanel }))
);

// FuturisticFeatures (named exports → wrapper)
const AIRouteOptimizer = React.lazy(() =>
  import("@/components/FuturisticFeatures").then(m => ({ default: m.AIRouteOptimizer }))
);
const OfflineModePanel = React.lazy(() =>
  import("@/components/FuturisticFeatures").then(m => ({ default: m.OfflineModePanel }))
);
const SocialNavPanel = React.lazy(() =>
  import("@/components/FuturisticFeatures").then(m => ({ default: m.SocialNavPanel }))
);

// SatelliteImageryOverlay (named export → wrapper)
const SatelliteLayerControl = React.lazy(() =>
  import("@/components/SatelliteImageryOverlay").then(m => ({ default: m.SatelliteLayerControl }))
);

// AccessibilityLayer (named export → wrapper)
const AccessibilityPanel = React.lazy(() =>
  import("@/components/AccessibilityLayer").then(m => ({ default: m.AccessibilityPanel }))
);

// Default export components — direct lazy import
const SpecVaultPanel = React.lazy(() => import("@/components/SpecVaultPanel"));
const AnalyticsPanel = React.lazy(() => import("@/components/AnalyticsPanel"));
const MapLayersPanel = React.lazy(() => import("@/components/MapLayersPanel"));
const SystemArchPanel = React.lazy(() => import("@/components/SystemArchPanel"));
const CommandCenter = React.lazy(() => import("@/components/CommandCenter"));
const EOCPanel = React.lazy(() => import("@/components/EOCPanel"));
const SatellitePanel = React.lazy(() => import("@/components/SatellitePanel"));
const TrafficCamPanel = React.lazy(() => import("@/components/TrafficCamPanel"));
const DataPipelineMonitor = React.lazy(() => import("@/components/DataPipelineMonitor"));
const AnalyticsEngine = React.lazy(() => import("@/components/AnalyticsEngine"));
const GANEStatusPanel = React.lazy(() => import("@/components/GANEStatusPanel"));
const C4ISRDashboard = React.lazy(() => import("@/components/C4ISRDashboard"));
const IncidentReporterPanel = React.lazy(() => import("@/components/IncidentReporterPanel"));
const LiveSharingPanel = React.lazy(() => import("@/components/LiveSharingPanel"));
const BatteryStatusPanel = React.lazy(() => import("@/components/BatteryStatusPanel"));
const CollaborationPanel = React.lazy(() => import("@/components/CollaborationPanel"));
const SatelliteCoverageMap = React.lazy(() => import("@/components/SatelliteCoverageMap"));
const RadioCommsPanel = React.lazy(() => import("@/components/RadioCommsPanel"));
const OfflineMapPanel = React.lazy(() => import("@/components/OfflineMapPanel"));
const RealtimeCollabPanel = React.lazy(() => import("@/components/RealtimeCollabPanel"));

// ═══════════════════════════════════════════════════════════
// LOADING FALLBACK
// ═══════════════════════════════════════════════════════════
function PanelFallback() {
  return (
    <div className="flex items-center justify-center p-12 min-h-[200px]">
      <div className="flex flex-col items-center gap-3">
        <Loader2 className="w-6 h-6 animate-spin text-cyan-400" />
        <span className="text-xs font-mono text-gray-400 tracking-wider">LOADING MODULE</span>
      </div>
    </div>
  );
}

// ═══════════════════════════════════════════════════════════
// PANEL MAP — Maps panel IDs to lazy components
// ═══════════════════════════════════════════════════════════
const PANEL_MAP: Record<string, React.LazyExoticComponent<React.ComponentType<{ onClose: () => void }>>> = {
  'parking': ParkingPanel,
  'charging': ChargingPanel,
  'weather': WeatherPanel,
  'v2x': V2XPanel,
  'digital-twin': DigitalTwinPanel,
  'driver-score': DriverScorePanel,
  'fleet': FleetPanel,
  'payments': PaymentsPanel,
  'evidence': EvidencePanel,
  'multimodal': MultiModalPanel,
  'spec-vault': SpecVaultPanel,
  'analytics': AnalyticsPanel,
  'map-layers': MapLayersPanel,
  'system-arch': SystemArchPanel,
  'command-center': CommandCenter,
  'eoc': EOCPanel,
  'satellite': SatellitePanel,
  'traffic-cam': TrafficCamPanel,
  'gnss-manager': GNSSPanel,
  'indoor-pos': IndoorPanel,
  'ar-nav': ARNavPanel,
  'risk-engine': RiskEnginePanel,
  'route-history': RouteHistoryPanel,
  'smart-alerts': SmartAlertsPanel,
  'ai-optimizer': AIRouteOptimizer,
  'offline-mode': OfflineModePanel,
  'social-nav': SocialNavPanel,
  'data-pipeline': DataPipelineMonitor,
  'satellite-imagery': SatelliteLayerControl,
  'analytics-engine': AnalyticsEngine,
  'accessibility': AccessibilityPanel,
  'gane-status': GANEStatusPanel,
  'c4isr': C4ISRDashboard,
  'incident-reporter': IncidentReporterPanel,
  'live-sharing': LiveSharingPanel,
  'battery-status': BatteryStatusPanel,
  'collaboration': CollaborationPanel as any,
  'satellite-coverage': SatelliteCoverageMap,
  'radio-comms': RadioCommsPanel,
  'offline-tiles': OfflineMapPanel,
  'realtime-collab': RealtimeCollabPanel,
};

// ═══════════════════════════════════════════════════════════
// RENDERER
// ═══════════════════════════════════════════════════════════
interface PanelRendererProps {
  activePanel: SmartPanel | null;
  onClose: () => void;
}

// Determine variant based on panel type
function getPanelVariant(panel: string): "collaboration" | "panel" {
  if (panel === "collaboration") return "collaboration";
  return "panel";
}

export default function PanelRenderer({ activePanel, onClose }: PanelRendererProps) {
  if (!activePanel) return null;

  const PanelComponent = PANEL_MAP[activePanel];
  if (!PanelComponent) return null;

  return (
    <ComponentErrorBoundary
      componentName={`Panel:${activePanel}`}
      variant={getPanelVariant(activePanel)}
    >
      <Suspense fallback={<PanelFallback />}>
        <PanelComponent onClose={onClose} />
      </Suspense>
    </ComponentErrorBoundary>
  );
}
