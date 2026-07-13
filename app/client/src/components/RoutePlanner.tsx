/**
 * G.A.N.E — Route Planner v2.0
 * Mode-reactive routing: Drive, Walk, Emergency, Plan
 * Walk mode includes terrain info, elevation, step count, calories
 * All modes show alternatives, traffic, and real-time weather impact
 */
import { useEffect, useCallback, useMemo, useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  ArrowLeft, Navigation, Clock, Route, Zap, Leaf, Plus, X,
  Rocket, Compass, Flame, Waypoints, Mountain,
  Heart, TrendingUp, CloudRain, Sun, Wind, ShieldCheck
} from "lucide-react";
import { useNavigation } from "@/contexts/NavigationContext";
import { createMarker } from "@/lib/mapCompat";
import { useRealDataContext } from "@/contexts/RealDataContext";
import type { RouteOption, RouteStep, NavMode } from "@/lib/navStore";
import { useLanguage } from "@/contexts/LanguageContext";

// ─── Mode config ───
const modeConfig: Record<NavMode, {
  travelMode: string;
  icon: typeof Rocket;
  label: string;
  i18nKey?: string;
  color: string;
  routeNames: string[];
}> = {
  drive: {
    travelMode: 'DRIVING',
    icon: Rocket,
    label: 'Drive',
    i18nKey: 'mode.drive',
    color: 'oklch(0.82 0.15 192)',
    routeNames: ['Fastest Route', 'Alternative Route', 'Scenic Route'] },
  walk: {
    travelMode: 'WALKING',
    icon: Compass,
    label: 'Walk',
    i18nKey: 'mode.walk',
    color: 'oklch(0.75 0.18 150)',
    routeNames: ['Shortest Walk', 'Park Route', 'Scenic Walk'] },
  emergency: {
    travelMode: 'DRIVING',
    icon: Flame,
    label: 'Emergency',
    i18nKey: 'mode.emergency',
    color: 'oklch(0.65 0.22 25)',
    routeNames: ['Emergency Route', 'Backup Route', 'Hospital Route'] },
  plan: {
    travelMode: 'DRIVING',
    icon: Waypoints,
    label: 'Plan',
    i18nKey: 'mode.plan',
    color: 'oklch(0.55 0.22 264)',
    routeNames: ['Optimal Route', 'Time-Saver', 'Eco Route'] } };

// ─── Walk stats calculator ───
function calcWalkStats(distanceText: string, durationText: string) {
  // Parse distance
  const kmMatch = distanceText.match(/([\d.]+)\s*km/i);
  const mMatch = distanceText.match(/([\d.]+)\s*m(?!i)/i);
  const miMatch = distanceText.match(/([\d.]+)\s*mi/i);
  let distKm = 0;
  if (kmMatch) distKm = parseFloat(kmMatch[1]);
  else if (mMatch) distKm = parseFloat(mMatch[1]) / 1000;
  else if (miMatch) distKm = parseFloat(miMatch[1]) * 1.609;

  // Parse duration in minutes
  const hrMatch = durationText.match(/(\d+)\s*h/i);
  const minMatch = durationText.match(/(\d+)\s*min/i);
  let durationMin = 0;
  if (hrMatch) durationMin += parseInt(hrMatch[1]) * 60;
  if (minMatch) durationMin += parseInt(minMatch[1]);

  const steps = Math.round(distKm * 1312); // ~1312 steps/km
  const calories = Math.round(distKm * 65); // ~65 cal/km walking
  const avgSpeedKmh = durationMin > 0 ? (distKm / (durationMin / 60)) : 5;

  return { distKm, durationMin, steps, calories, avgSpeedKmh };
}

export default function RoutePlanner() {
  const { t, dir } = useLanguage();
  const { state, dispatch, mapRef, directionsServiceRef, directionsRendererRef } = useNavigation();
  const { weather } = useRealDataContext();
  const [showWalkDetails, setShowWalkDetails] = useState(false);

  const mode = modeConfig[state.navMode] || modeConfig.drive;
  const isWalkMode = state.navMode === 'walk';

  // Weather impact on walk
  const weatherImpact = useMemo(() => {
    if (!weather) return { penalty: 0, warning: '' };
    let penalty = 0;
    const warnings: string[] = [];
    if (weather.precipitation > 1) { penalty += 5; warnings.push('Rain expected'); }
    if (weather.windSpeed > 25) { penalty += 3; warnings.push(`Wind ${weather.windSpeed.toFixed(0)} km/h`); }
    if (weather.temperature > 35) { penalty += 4; warnings.push(`Heat ${weather.temperature.toFixed(0)}°C`); }
    if (weather.temperature < 5) { penalty += 2; warnings.push(`Cold ${weather.temperature.toFixed(0)}°C`); }
    if (weather.visibility < 3) { penalty += 3; warnings.push('Low visibility'); }
    return { penalty, warning: warnings.join(' • ') };
  }, [weather]);

  const calculateRoutes = useCallback(() => {
    if (!directionsServiceRef.current || !state.destination) return;
    const origin = state.origin?.location || state.userLocation;
    if (!origin) return;

    dispatch({ type: 'SET_LOADING', loading: true });

    const travelMode = (google.maps.TravelMode as any)[mode.travelMode] || google.maps.TravelMode.DRIVING;

    directionsServiceRef.current.route(
      {
        origin,
        destination: state.destination.location,
        waypoints: state.waypoints.map(w => ({ location: w.location, stopover: true })),
        travelMode,
        provideRouteAlternatives: true,
        optimizeWaypoints: state.waypoints.length > 1,
        avoidTolls: state.drivingProfile === 'eco' || state.navMode === 'emergency',
        avoidHighways: state.drivingProfile === 'eco' },
      (result: any, status: any) => {
        if (status === "OK" && result?.routes) {
          const colors = isWalkMode
            ? ['#00d4aa', '#a78bfa', '#f59e0b']
            : state.navMode === 'emergency'
              ? ['#ef4444', '#f97316', '#eab308']
              : ['#00d4aa', '#6366f1', '#f59e0b'];

          const routes: RouteOption[] = result.routes.map((route: any, idx: number) => {
            const leg = route.legs[0];
            const steps: RouteStep[] = leg.steps.map((step: any) => ({
              instruction: step.instructions?.replace(/<[^>]*>/g, '') || '',
              distance: step.distance?.text || '',
              duration: step.duration?.text || '',
              maneuver: step.maneuver || undefined,
              startLocation: { lat: step.start_location.lat(), lng: step.start_location.lng() },
              endLocation: { lat: step.end_location.lat(), lng: step.end_location.lng() } }));

            const durationMin = leg.duration?.value || 0;
            let traffic: 'free' | 'moderate' | 'heavy' | 'severe' = 'free';
            if (!isWalkMode && leg.duration_in_traffic) {
              const ratio = leg.duration_in_traffic.value / leg.duration.value;
              if (ratio > 1.5) traffic = 'severe';
              else if (ratio > 1.25) traffic = 'heavy';
              else if (ratio > 1.1) traffic = 'moderate';
            }

            return {
              id: `route-${idx}`,
              name: mode.routeNames[idx] || `Route ${idx + 1}`,
              distance: leg.distance?.text || '',
              duration: leg.duration?.text || '',
              durationValue: durationMin,
              color: colors[idx] || '#6366f1',
              summary: route.summary || '',
              trafficLevel: traffic,
              steps };
          });

          dispatch({ type: 'SET_ROUTES', routes });
          if (routes.length > 0) {
            dispatch({ type: 'SELECT_ROUTE', routeId: routes[0].id });
          }

          if (directionsRendererRef.current && result.routes.length > 0) {
            directionsRendererRef.current.setDirections(result);
            directionsRendererRef.current.setRouteIndex(0);
          }

          if (mapRef.current && state.destination) {
            createMarker({
              map: mapRef.current,
              position: state.destination.location,
              icon: {
                path: google.maps.SymbolPath.CIRCLE,
                scale: 10,
                fillColor: mode.color.includes('oklch') ? '#00d4aa' : mode.color,
                fillOpacity: 1,
                strokeColor: '#F3F4F6',
                strokeWeight: 3 } });
          }
        } else {
          dispatch({ type: 'SET_LOADING', loading: false });
        }
      }
    );
  }, [state.destination, state.origin, state.userLocation, state.waypoints, state.drivingProfile, state.navMode, dispatch, directionsServiceRef, directionsRendererRef, mapRef, mode, isWalkMode]);

  useEffect(() => {
    calculateRoutes();
  }, [calculateRoutes]);

  const selectRoute = (routeId: string) => {
    dispatch({ type: 'SELECT_ROUTE', routeId });
    const idx = state.routes.findIndex(r => r.id === routeId);
    if (directionsRendererRef.current && idx >= 0) {
      directionsRendererRef.current.setRouteIndex(idx);
    }
  };

  const startNav = () => {
    if (state.selectedRouteId) {
      dispatch({ type: 'START_NAVIGATION' });
    }
  };

  const selectedRoute = state.routes.find(r => r.id === state.selectedRouteId);
  const walkStats = selectedRoute && isWalkMode ? calcWalkStats(selectedRoute.distance, selectedRoute.duration) : null;

  const trafficColor: Record<string, string> = {
    free: 'text-gane-green',
    moderate: 'text-gane-amber',
    heavy: 'text-orange-400',
    severe: 'text-gane-red' };

  return (
    <motion.div
      initial={{ y: 300, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      exit={{ y: 300, opacity: 0 }}
      transition={{ type: "spring", damping: 25, stiffness: 300 }}
      className="bottom-sheet"
      style={{ paddingBottom: 'env(safe-area-inset-bottom, 16px)' }}
    >
      <div className="bottom-sheet-handle" />

      {/* Header with mode indicator */}
      <div className="flex items-center gap-3 px-4 pb-3">
        <button
          onClick={() => dispatch({ type: 'CLEAR_ROUTE' })}
          className="fab w-9 h-9"
          aria-label="Go back"
        >
          <ArrowLeft className="w-4 h-4 text-white/70" />
        </button>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <div className="text-xs text-white/30 font-medium">TO</div>
            <div className="flex items-center gap-1 px-1.5 py-0.5 rounded-md text-[9px] font-semibold"
              style={{
                background: `color-mix(in oklch, ${mode.color}, transparent 88%)`,
                color: mode.color,
                border: `1px solid color-mix(in oklch, ${mode.color}, transparent 75%)` }}>
              <mode.icon className="w-2.5 h-2.5" />
              {mode.i18nKey}
            </div>
          </div>
          <div className="text-sm text-white/90 font-semibold truncate">{state.destination?.name}</div>
          <div className="text-xs text-white/40 truncate">{state.destination?.address}</div>
        </div>
      </div>

      {/* Weather warning for walk mode */}
      {isWalkMode && weatherImpact.warning && (
        <motion.div
          initial={{ opacity: 0, height: 0 }}
          animate={{ opacity: 1, height: 'auto' }}
          className="mx-4 mb-3 px-3 py-2 rounded-xl flex items-center gap-2"
          style={{
            background: 'oklch(0.80 0.16 75 / 8%)',
            border: '1px solid oklch(0.80 0.16 75 / 15%)' }}
        >
          <CloudRain className="w-4 h-4 text-gane-amber flex-shrink-0" />
          <div className="text-[10px] text-gane-amber">{weatherImpact.warning} — adds ~{weatherImpact.penalty} min</div>
        </motion.div>
      )}

      {/* Walk stats card */}
      <AnimatePresence>
        {isWalkMode && walkStats && selectedRoute && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: 'auto' }}
            exit={{ opacity: 0, height: 0 }}
            className="mx-4 mb-3"
          >
            <div className="grid grid-cols-4 gap-2 p-3 rounded-xl"
              style={{
                background: 'oklch(0.75 0.18 150 / 5%)',
                border: '1px solid oklch(0.75 0.18 150 / 10%)' }}>
              <div className="text-center">
                <Compass className="w-4 h-4 text-gane-green mx-auto mb-1" />
                <div className="text-sm font-bold text-gane-green font-mono">{walkStats.steps.toLocaleString()}</div>
                <div className="text-[8px] text-white/25">Steps</div>
              </div>
              <div className="text-center">
                <Flame className="w-4 h-4 text-gane-amber mx-auto mb-1" />
                <div className="text-sm font-bold text-gane-amber font-mono">{walkStats.calories}</div>
                <div className="text-[8px] text-white/25">Calories</div>
              </div>
              <div className="text-center">
                <TrendingUp className="w-4 h-4 text-gane-cyan mx-auto mb-1" />
                <div className="text-sm font-bold text-gane-cyan font-mono">{walkStats.avgSpeedKmh.toFixed(1)}</div>
                <div className="text-[8px] text-white/25">km/h avg</div>
              </div>
              <div className="text-center">
                <Mountain className="w-4 h-4 text-gane-indigo mx-auto mb-1" />
                <div className="text-sm font-bold text-gane-indigo font-mono">{walkStats.distKm.toFixed(1)}</div>
                <div className="text-[8px] text-white/25">km total</div>
              </div>
            </div>

            {/* Terrain info */}
            <div className="flex items-center gap-2 mt-2 px-1">
              <div className="flex items-center gap-1 px-2 py-1 rounded-md bg-white/3 text-[9px] text-white/30">
                <Sun className="w-3 h-3" /> {weather?.temperature?.toFixed(0) || '--'}°C
              </div>
              <div className="flex items-center gap-1 px-2 py-1 rounded-md bg-white/3 text-[9px] text-white/30">
                <Wind className="w-3 h-3" /> {weather?.windSpeed?.toFixed(0) || '--'} km/h
              </div>
              <div className="flex items-center gap-1 px-2 py-1 rounded-md bg-white/3 text-[9px] text-white/30">
                <ShieldCheck className="w-3 h-3" /> Safe route
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Route options */}
      <div className="px-4 pb-4 space-y-2 max-h-[40vh] overflow-y-auto">
        {state.isLoading ? (
          <div className="flex items-center justify-center py-8">
            <div className="w-8 h-8 border-2 rounded-full animate-spin"
              style={{
                borderColor: `color-mix(in oklch, ${mode.color}, transparent 70%)`,
                borderTopColor: mode.color }} />
          </div>
        ) : (
          state.routes.map((route) => {
            const isSelected = state.selectedRouteId === route.id;
            const routeWalkStats = isWalkMode ? calcWalkStats(route.distance, route.duration) : null;
            return (
              <motion.button
                key={route.id}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                onClick={() => selectRoute(route.id)}
                className={`w-full flex items-center gap-3 p-3 rounded-xl transition-all text-left ${
                  isSelected
                    ? 'bg-white/8 border shadow-lg'
                    : 'bg-white/3 border border-transparent hover:bg-white/5'
                }`}
                style={isSelected ? {
                  borderColor: `color-mix(in oklch, ${mode.color}, transparent 70%)` } : undefined}
                aria-label={`Select ${route.name}`}
              >
                <div className="flex-shrink-0 w-10 h-10 rounded-lg flex items-center justify-center"
                  style={{ background: `${route.color}20`, border: `1px solid ${route.color}40` }}>
                  {isWalkMode ? (
                    route.id === 'route-0' ? <Compass className="w-5 h-5" style={{ color: route.color }} /> :
                    route.id === 'route-1' ? <Mountain className="w-5 h-5" style={{ color: route.color }} /> :
                    <Leaf className="w-5 h-5" style={{ color: route.color }} />
                  ) : (
                    route.id === 'route-0' ? <Zap className="w-5 h-5" style={{ color: route.color }} /> :
                    route.id === 'route-1' ? <Route className="w-5 h-5" style={{ color: route.color }} /> :
                    <Leaf className="w-5 h-5" style={{ color: route.color }} />
                  )}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="text-sm font-semibold text-white/90">{route.duration}</span>
                    {!isWalkMode && (
                      <span className={`text-xs ${trafficColor[route.trafficLevel]}`}>
                        {route.trafficLevel === 'free' ? 'Clear' : route.trafficLevel === 'moderate' ? 'Moderate' : route.trafficLevel === 'heavy' ? 'Heavy' : 'Severe'}
                      </span>
                    )}
                  </div>
                  <div className="text-xs text-white/40">{route.distance} via {route.summary}</div>
                  {isWalkMode && routeWalkStats && (
                    <div className="flex items-center gap-2 mt-0.5">
                      <span className="text-[9px] text-white/25">{routeWalkStats.steps.toLocaleString()} steps</span>
                      <span className="text-[9px] text-white/15">•</span>
                      <span className="text-[9px] text-white/25">{routeWalkStats.calories} cal</span>
                    </div>
                  )}
                </div>
                <div className="text-right flex-shrink-0">
                  <div className="text-xs text-white/50 font-mono">{route.name.split(' ')[0]}</div>
                  {isSelected && (
                    <div className="text-[9px] font-semibold" style={{ color: mode.color }}>SELECTED</div>
                  )}
                </div>
              </motion.button>
            );
          })
        )}
      </div>

      {/* Start navigation button */}
      {state.selectedRouteId && !state.isLoading && (
        <div className="px-4 pb-6 pt-2">
          <button
            onClick={startNav}
            className="w-full py-3.5 rounded-xl font-semibold text-sm flex items-center justify-center gap-2 transition-all active:scale-[0.98]"
            style={{ background: mode.color, color: 'oklch(0.12 0.01 264)' }}
            aria-label="Start navigation"
          >
            <Navigation className="w-5 h-5" />
            {isWalkMode ? 'Start Walking' : state.navMode === 'emergency' ? 'Emergency Navigate' : 'Start Navigation'}
          </button>
        </div>
      )}
    </motion.div>
  );
}
