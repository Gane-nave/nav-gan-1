/**
 * G.A.N.E — Global Mobility Intelligence Network
 *
 * Main Home page — orchestrates all sub-components.
 * Refactored from 1,276 lines into modular sub-components.
 *
 * Sub-components:
 *   QuantumBoot   — Cinematic boot sequence animation
 *   AppSidebar    — Left sidebar with collapsible groups
 *   SearchBar     — Top search bar with voice + mode pill
 *   BottomDock    — Bottom navigation dock with mode selector
 *   LiveStatusBar — Top-right real-time status display
 *   PanelRenderer — Smart panel switch/renderer
 */
import React, { useState, useCallback, useMemo, Suspense } from "react";
import useAnimationPerformance from "@/hooks/useAnimationPerformance";
import { AnimatePresence } from "framer-motion";
import { useNavigation } from "@/contexts/NavigationContext";
import { useRealDataContext } from "@/contexts/RealDataContext";
import type { SmartPanel } from "@/lib/navStore";

// Map & overlay components
import OpenNavigationMap from "@/components/OpenNavigationMap";
import ComponentErrorBoundary from "@/components/ComponentErrorBoundary";
import MapOverlayRenderer from "@/components/MapOverlayRenderer";
import MapControls from "@/components/MapControls";
import WeatherRadarOverlay from "@/components/WeatherRadarOverlay";
const SatelliteTerrainOverlay = React.lazy(
  () => import("@/components/SatelliteImageryOverlay")
);
import { AmbientWeatherOverlay } from "@/components/GestureHints";
import { NightModeController } from "@/components/NightMode";
import {
  ParticleField,
  EnergyGrid,
  HolographicCorners,
  AmbientGlow,
} from "@/components/HolographicEffects";
import VoiceCommandSystem from "@/components/VoiceCommandSystem";
import GestureController from "@/components/GestureController";
import { AIOrb, AIChatPanel } from "@/components/AICopilot";
import OfflineIndicator from "@/components/OfflineIndicator";
import PWAInstallPrompt from "@/components/PWAInstallPrompt";
import CollaboratorCursors from "@/components/CollaboratorCursors";
import DrawingToolbar from "@/components/DrawingToolbar";
import { useCollaboration } from "@/hooks/useCollaboration";
import { useAuth } from "@/_core/hooks/useAuth";

// Full-screen overlay views
import SearchPanel from "@/components/SearchPanel";
import RoutePlanner from "@/components/RoutePlanner";
import NavigationHUD from "@/components/NavigationHUD";
import SettingsPanel from "@/components/SettingsPanel";
import TrafficDashboard from "@/components/TrafficDashboard";
import OnboardingScreen from "@/components/OnboardingScreen";

// Refactored sub-components
import {
  QuantumBoot,
  AppSidebar,
  SearchBar,
  BottomDock,
  LiveStatusBar,
  PanelRenderer,
  navModes,
} from "./home";

export default function Home() {
  const { state, dispatch, mapRef } = useNavigation();
  const { user } = useAuth();
  const [booted, setBooted] = useState(false);
  const [activeCollabSession, setActiveCollabSession] = useState<string | null>(
    null
  );
  const {
    remoteCursors,
    updateCursor,
    participants,
    addAnnotation,
    deleteAnnotation,
    annotations,
  } = useCollaboration(activeCollabSession);
  const [showDrawingTools, setShowDrawingTools] = useState(false);

  // Enrich cursors with participant names
  const enrichedCursors = useMemo(() => {
    const enriched = new Map<
      number,
      { lat: number; lon: number; color: string; name?: string }
    >();
    remoteCursors.forEach((cursor, userId) => {
      const participant = (participants ?? []).find(
        (p: any) => p.userId === userId
      );
      enriched.set(userId, {
        ...cursor,
        name: participant?.displayName || `User ${userId}`,
      });
    });
    return enriched;
  }, [remoteCursors, participants]);
  const [showContent, setShowContent] = useState(false);
  const { weather } = useRealDataContext();

  const activeMode = useMemo(
    () => navModes.find(m => m.id === state.navMode) || navModes[0],
    [state.navMode]
  );

  // Performance-aware animation control
  const { shouldAnimate, shouldAnimateLight } = useAnimationPerformance();

  const showSidebar = state.view === "map" && !state.isNavigating;

  const handleBootComplete = useCallback(() => {
    setBooted(true);
    setTimeout(() => setShowContent(true), 200);
  }, []);

  const closePanel = useCallback(() => {
    dispatch({ type: "SET_ACTIVE_PANEL", panel: null });
  }, [dispatch]);

  const togglePanel = useCallback(
    (panel: SmartPanel) => {
      // Special handling for drawing tools — toggle toolbar instead of panel
      if (panel === ("drawing-tools" as SmartPanel)) {
        setShowDrawingTools(prev => !prev);
        return;
      }
      dispatch({
        type: "SET_ACTIVE_PANEL",
        panel: state.activePanel === panel ? null : panel,
      });
    },
    [dispatch, state.activePanel]
  );

  return (
    <div
      className="h-screen w-screen overflow-hidden relative"
      style={{ background: "#F9FAFB" }}
    >
      {/* ═══ OFFLINE INDICATOR ═══ */}
      <OfflineIndicator />

      {/* ═══ QUANTUM BOOT SEQUENCE ═══ */}
      <AnimatePresence>
        {!booted && <QuantumBoot key="boot" onComplete={handleBootComplete} />}
      </AnimatePresence>

      {/* ═══ HOLOGRAPHIC AMBIENT LAYER ═══ */}
      {/* Heavy ambient effects only on desktop with no reduced-motion preference */}
      {shouldAnimate && (
        <>
          <AmbientGlow />
          <EnergyGrid />
          <ParticleField intensity={0.4} interactive />
        </>
      )}
      {/* Lightweight static SVG corners always render */}
      <HolographicCorners />

      {/* Full-screen map */}
      <ComponentErrorBoundary
        componentName="OpenNavigationMap"
        variant="map"
        className="h-full w-full"
      >
        <OpenNavigationMap
          onMapClick={
            activeCollabSession
              ? (lat: number, lon: number) => updateCursor(lat, lon)
              : undefined
          }
        />
      </ComponentErrorBoundary>
      {/* Collaborator cursors overlay */}
      {activeCollabSession && (
        <CollaboratorCursors
          cursors={enrichedCursors}
          currentUserId={user?.id}
        />
      )}
      <MapOverlayRenderer />
      <WeatherRadarOverlay />
      <Suspense fallback={null}>
        <SatelliteTerrainOverlay />
      </Suspense>
      {shouldAnimateLight && (
        <AmbientWeatherOverlay condition={weather?.weatherDescription} />
      )}
      <NightModeController>{() => null}</NightModeController>

      {/* Live status bar */}
      {showContent && state.view === "map" && !state.isNavigating && (
        <LiveStatusBar color={activeMode.color} />
      )}

      {/* AI Copilot Orb */}
      {showContent && state.view === "map" && !state.isNavigating && (
        <>
          <div className="fixed z-40" style={{ right: "16px", bottom: "90px" }}>
            <AIOrb
              onClick={() =>
                dispatch({
                  type: "SET_ACTIVE_PANEL",
                  panel:
                    state.activePanel === "smart-alerts"
                      ? null
                      : ("smart-alerts" as any),
                })
              }
              hasAlerts={true}
            />
          </div>
          <AnimatePresence>
            {state.activePanel === "smart-alerts" && (
              <AIChatPanel isOpen={true} onClose={closePanel} />
            )}
          </AnimatePresence>
        </>
      )}

      {/* ═══ SIDEBAR ═══ */}
      <AnimatePresence>
        {showSidebar && showContent && (
          <AppSidebar
            key="sidebar"
            activeMode={activeMode}
            onTogglePanel={togglePanel}
          />
        )}
      </AnimatePresence>

      {/* ═══ SMART PANELS ═══ */}
      <AnimatePresence>
        {state.activePanel && showSidebar && showContent && (
          <div key={state.activePanel}>
            <PanelRenderer
              activePanel={state.activePanel}
              onClose={closePanel}
            />
          </div>
        )}
      </AnimatePresence>

      {/* Drawing Toolbar */}
      {showContent &&
        showDrawingTools &&
        state.view === "map" &&
        !state.isNavigating && (
          <ComponentErrorBoundary
            componentName="DrawingToolbar"
            variant="panel"
          >
            <DrawingToolbar
              map={mapRef.current}
              isCollaborating={!!activeCollabSession}
              onAddAnnotation={addAnnotation}
              onDeleteAnnotation={deleteAnnotation}
              remoteAnnotations={annotations as any}
            />
          </ComponentErrorBoundary>
        )}

      {/* Map controls */}
      {showContent &&
        state.view !== "search" &&
        state.view !== "settings" &&
        state.view !== "traffic" && <MapControls />}

      {/* ═══ SEARCH BAR ═══ */}
      {showContent && state.view === "map" && !state.isNavigating && (
        <SearchBar activeMode={activeMode} showSidebar={showSidebar} />
      )}

      {/* ═══ BOTTOM DOCK ═══ */}
      {showContent && state.view === "map" && !state.isNavigating && (
        <BottomDock activeMode={activeMode} showSidebar={showSidebar} />
      )}

      {/* Voice Command System */}
      {showContent && <VoiceCommandSystem />}

      {/* Gesture Controller */}
      {showContent && <GestureController />}

      {/* PWA Install Prompt */}
      {showContent && <PWAInstallPrompt />}

      {/* ═══ OVERLAYS ═══ */}
      <AnimatePresence>
        {state.view === "search" && <SearchPanel key="search" />}
        {state.view === "route-plan" && <RoutePlanner key="route" />}
        {state.isNavigating && <NavigationHUD key="hud" />}
        {state.view === "settings" && <SettingsPanel key="settings" />}
        {state.view === "traffic" && <TrafficDashboard key="traffic" />}
        {state.showOnboarding && <OnboardingScreen key="onboarding" />}
      </AnimatePresence>
    </div>
  );
}
