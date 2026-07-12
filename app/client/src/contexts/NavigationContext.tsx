/**
 * G.A.N.E — Navigation Context
 * Provides global navigation state and dispatch to all components.
 * 
 * HMR-safe: uses useMemo to stabilize context value and gracefully
 * handles missing provider during hot-reload recovery.
 */
import { createContext, useContext, useReducer, useRef, useMemo, type ReactNode, type Dispatch } from "react";
import { navReducer, initialNavState, type NavState, type NavAction } from "@/lib/navStore";

interface NavigationContextValue {
  state: NavState;
  dispatch: Dispatch<NavAction>;
  mapRef: React.MutableRefObject<google.maps.Map | null>;
  directionsServiceRef: React.MutableRefObject<google.maps.DirectionsService | null>;
  directionsRendererRef: React.MutableRefObject<google.maps.DirectionsRenderer | null>;
  geocoderRef: React.MutableRefObject<google.maps.Geocoder | null>;
  placesServiceRef: React.MutableRefObject<google.maps.places.PlacesService | null>;
  trafficLayerRef: React.MutableRefObject<google.maps.TrafficLayer | null>;
  transitLayerRef: React.MutableRefObject<google.maps.TransitLayer | null>;
  bicyclingLayerRef: React.MutableRefObject<google.maps.BicyclingLayer | null>;
}

const NavigationContext = createContext<NavigationContextValue | null>(null);

export function NavigationProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(navReducer, initialNavState);
  const mapRef = useRef<google.maps.Map | null>(null);
  const directionsServiceRef = useRef<google.maps.DirectionsService | null>(null);
  const directionsRendererRef = useRef<google.maps.DirectionsRenderer | null>(null);
  const geocoderRef = useRef<google.maps.Geocoder | null>(null);
  const placesServiceRef = useRef<google.maps.places.PlacesService | null>(null);
  const trafficLayerRef = useRef<google.maps.TrafficLayer | null>(null);
  const transitLayerRef = useRef<google.maps.TransitLayer | null>(null);
  const bicyclingLayerRef = useRef<google.maps.BicyclingLayer | null>(null);

  // Stabilize context value to prevent unnecessary re-renders during HMR
  const value = useMemo(() => ({
    state, dispatch,
    mapRef, directionsServiceRef, directionsRendererRef,
    geocoderRef, placesServiceRef,
    trafficLayerRef, transitLayerRef, bicyclingLayerRef,
  }), [state]);

  return (
    <NavigationContext.Provider value={value}>
      {children}
    </NavigationContext.Provider>
  );
}

/**
 * HMR-safe hook: during hot-reload, the context tree may temporarily
 * unmount and remount. Instead of throwing, we return a fallback state
 * that prevents crashes. Components will re-render correctly once the
 * provider is back in the tree.
 */
const fallbackDispatch: Dispatch<NavAction> = () => {
  console.warn("[NavigationContext] dispatch called outside provider — likely HMR recovery");
};

const fallbackRef = { current: null };

const fallbackValue: NavigationContextValue = {
  state: initialNavState,
  dispatch: fallbackDispatch,
  mapRef: fallbackRef as React.MutableRefObject<google.maps.Map | null>,
  directionsServiceRef: fallbackRef as React.MutableRefObject<google.maps.DirectionsService | null>,
  directionsRendererRef: fallbackRef as React.MutableRefObject<google.maps.DirectionsRenderer | null>,
  geocoderRef: fallbackRef as React.MutableRefObject<google.maps.Geocoder | null>,
  placesServiceRef: fallbackRef as React.MutableRefObject<google.maps.places.PlacesService | null>,
  trafficLayerRef: fallbackRef as React.MutableRefObject<google.maps.TrafficLayer | null>,
  transitLayerRef: fallbackRef as React.MutableRefObject<google.maps.TransitLayer | null>,
  bicyclingLayerRef: fallbackRef as React.MutableRefObject<google.maps.BicyclingLayer | null>,
};

export function useNavigation() {
  const ctx = useContext(NavigationContext);
  if (!ctx) {
    // During HMR, the provider may be temporarily missing.
    // Return fallback instead of throwing to prevent crash loops.
    if (import.meta.hot) {
      return fallbackValue;
    }
    throw new Error("useNavigation must be used within NavigationProvider");
  }
  return ctx;
}
