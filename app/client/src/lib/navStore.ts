/**
 * G.A.N.E — Navigation State Store
 * Central state management using React context + useReducer pattern
 */

export interface LatLng {
  lat: number;
  lng: number;
}

export interface Place {
  id: string;
  name: string;
  address: string;
  location: LatLng;
  type?: string;
  icon?: string;
}

export interface RouteOption {
  id: string;
  name: string;
  distance: string;
  duration: string;
  durationValue: number;
  color: string;
  summary: string;
  trafficLevel: 'free' | 'moderate' | 'heavy' | 'severe';
  steps: RouteStep[];
}

export interface RouteStep {
  instruction: string;
  distance: string;
  duration: string;
  maneuver?: string;
  startLocation: LatLng;
  endLocation: LatLng;
}

export type DrivingProfile = 'standard' | 'eco' | 'fast' | 'truck' | 'motorcycle';
export type NavMode = 'drive' | 'walk' | 'emergency' | 'plan';
export type MapLayer = 'traffic' | 'transit' | 'bicycling' | 'satellite' | 'weather-radar' | 'weather-temp' | 'weather-wind' | 'weather-precip' | 'air-quality' | 'earthquake' | 'terrain' | 'heatmap' | 'night-vision' | 'risk-zones' | 'ev-charging' | 'parking' | '3d-buildings' | 'indoor' | 'lane-level';
export type AppView = 'map' | 'search' | 'route-plan' | 'navigating' | 'settings' | 'profile' | 'traffic' | 'places' | 'onboarding';
export type SmartPanel = 'parking' | 'charging' | 'weather' | 'v2x' | 'digital-twin' | 'driver-score' | 'fleet' | 'payments' | 'evidence' | 'multimodal' | 'spec-vault' | 'analytics' | 'map-layers' | 'system-arch' | 'command-center' | 'eoc' | 'satellite' | 'traffic-cam' | 'gnss-manager' | 'indoor-pos' | 'ar-nav' | 'risk-engine' | 'route-history' | 'smart-alerts' | 'ai-optimizer' | 'offline-mode' | 'social-nav' | 'data-pipeline' | 'satellite-imagery' | 'accessibility' | 'analytics-engine' | 'gane-status' | 'c4isr' | 'incident-reporter' | 'live-sharing' | 'battery-status' | 'satellite-coverage' | 'radio-comms' | 'offline-tiles' | 'realtime-collab' | null;

export interface NavState {
  view: AppView;
  userLocation: LatLng | null;
  mapCenter: LatLng;
  mapZoom: number;
  origin: Place | null;
  destination: Place | null;
  waypoints: Place[];
  routes: RouteOption[];
  selectedRouteId: string | null;
  currentStepIndex: number;
  isNavigating: boolean;
  searchQuery: string;
  searchResults: Place[];
  recentSearches: Place[];
  favorites: Place[];
  drivingProfile: DrivingProfile;
  activeLayers: MapLayer[];
  voiceEnabled: boolean;
  voiceLanguage: string;
  speedKmh: number;
  heading: number;
  isLoading: boolean;
  bottomSheetHeight: 'collapsed' | 'half' | 'full';
  showOnboarding: boolean;
  activePanel: SmartPanel;
  navMode: NavMode;
}

export const initialNavState: NavState = {
  view: 'map',
  userLocation: null,
  mapCenter: { lat: 32.0853, lng: 34.7818 }, // Tel Aviv
  mapZoom: 14,
  origin: null,
  destination: null,
  waypoints: [],
  routes: [],
  selectedRouteId: null,
  currentStepIndex: 0,
  isNavigating: false,
  searchQuery: '',
  searchResults: [],
  recentSearches: [],
  favorites: [
    { id: 'fav-1', name: 'Home', address: 'Rothschild Blvd 1, Tel Aviv', location: { lat: 32.0636, lng: 34.7742 }, icon: '🏠' },
    { id: 'fav-2', name: 'Work', address: 'Azrieli Center, Tel Aviv', location: { lat: 32.0741, lng: 34.7922 }, icon: '💼' },
  ],
  drivingProfile: 'standard',
  activeLayers: [],
  voiceEnabled: true,
  voiceLanguage: 'en',
  speedKmh: 0,
  heading: 0,
  isLoading: false,
  bottomSheetHeight: 'collapsed',
  showOnboarding: false,
  activePanel: null,
  navMode: 'drive' as NavMode,
};

export type NavAction =
  | { type: 'SET_VIEW'; view: AppView }
  | { type: 'SET_USER_LOCATION'; location: LatLng }
  | { type: 'SET_MAP_CENTER'; center: LatLng }
  | { type: 'SET_MAP_ZOOM'; zoom: number }
  | { type: 'SET_ORIGIN'; place: Place | null }
  | { type: 'SET_DESTINATION'; place: Place | null }
  | { type: 'ADD_WAYPOINT'; place: Place }
  | { type: 'REMOVE_WAYPOINT'; index: number }
  | { type: 'SET_ROUTES'; routes: RouteOption[] }
  | { type: 'SELECT_ROUTE'; routeId: string }
  | { type: 'START_NAVIGATION' }
  | { type: 'STOP_NAVIGATION' }
  | { type: 'NEXT_STEP' }
  | { type: 'SET_SEARCH_QUERY'; query: string }
  | { type: 'SET_SEARCH_RESULTS'; results: Place[] }
  | { type: 'ADD_RECENT_SEARCH'; place: Place }
  | { type: 'ADD_FAVORITE'; place: Place }
  | { type: 'REMOVE_FAVORITE'; id: string }
  | { type: 'SET_DRIVING_PROFILE'; profile: DrivingProfile }
  | { type: 'TOGGLE_LAYER'; layer: MapLayer }
  | { type: 'SET_VOICE_ENABLED'; enabled: boolean }
  | { type: 'SET_VOICE_LANGUAGE'; language: string }
  | { type: 'SET_SPEED'; speed: number }
  | { type: 'SET_HEADING'; heading: number }
  | { type: 'SET_LOADING'; loading: boolean }
  | { type: 'SET_BOTTOM_SHEET'; height: 'collapsed' | 'half' | 'full' }
  | { type: 'CLEAR_ROUTE' }
  | { type: 'SET_ONBOARDING'; show: boolean }
  | { type: 'SET_ACTIVE_PANEL'; panel: SmartPanel }
  | { type: 'SET_NAV_MODE'; mode: NavMode };

export function navReducer(state: NavState, action: NavAction): NavState {
  switch (action.type) {
    case 'SET_VIEW': return { ...state, view: action.view, activePanel: null };
    case 'SET_USER_LOCATION': return { ...state, userLocation: action.location };
    case 'SET_MAP_CENTER': return { ...state, mapCenter: action.center };
    case 'SET_MAP_ZOOM': return { ...state, mapZoom: action.zoom };
    case 'SET_ORIGIN': return { ...state, origin: action.place };
    case 'SET_DESTINATION': return { ...state, destination: action.place };
    case 'ADD_WAYPOINT': return { ...state, waypoints: [...state.waypoints, action.place] };
    case 'REMOVE_WAYPOINT': return { ...state, waypoints: state.waypoints.filter((_, i) => i !== action.index) };
    case 'SET_ROUTES': return { ...state, routes: action.routes, isLoading: false };
    case 'SELECT_ROUTE': return { ...state, selectedRouteId: action.routeId };
    case 'START_NAVIGATION': return { ...state, isNavigating: true, view: 'navigating', currentStepIndex: 0 };
    case 'STOP_NAVIGATION': return { ...state, isNavigating: false, view: 'map', currentStepIndex: 0, routes: [], selectedRouteId: null, destination: null };
    case 'NEXT_STEP': return { ...state, currentStepIndex: Math.min(state.currentStepIndex + 1, (state.routes.find(r => r.id === state.selectedRouteId)?.steps.length ?? 1) - 1) };
    case 'SET_SEARCH_QUERY': return { ...state, searchQuery: action.query };
    case 'SET_SEARCH_RESULTS': return { ...state, searchResults: action.results, isLoading: false };
    case 'ADD_RECENT_SEARCH': return { ...state, recentSearches: [action.place, ...state.recentSearches.filter(p => p.id !== action.place.id)].slice(0, 10) };
    case 'ADD_FAVORITE': return { ...state, favorites: [...state.favorites, action.place] };
    case 'REMOVE_FAVORITE': return { ...state, favorites: state.favorites.filter(f => f.id !== action.id) };
    case 'SET_DRIVING_PROFILE': return { ...state, drivingProfile: action.profile };
    case 'TOGGLE_LAYER': {
      const has = state.activeLayers.includes(action.layer);
      return { ...state, activeLayers: has ? state.activeLayers.filter(l => l !== action.layer) : [...state.activeLayers, action.layer] };
    }
    case 'SET_VOICE_ENABLED': return { ...state, voiceEnabled: action.enabled };
    case 'SET_VOICE_LANGUAGE': return { ...state, voiceLanguage: action.language };
    case 'SET_SPEED': return { ...state, speedKmh: action.speed };
    case 'SET_HEADING': return { ...state, heading: action.heading };
    case 'SET_LOADING': return { ...state, isLoading: action.loading };
    case 'SET_BOTTOM_SHEET': return { ...state, bottomSheetHeight: action.height };
    case 'CLEAR_ROUTE': return { ...state, routes: [], selectedRouteId: null, destination: null, waypoints: [], origin: null, view: 'map', isNavigating: false };
    case 'SET_ONBOARDING': return { ...state, showOnboarding: action.show };
    case 'SET_ACTIVE_PANEL': return { ...state, activePanel: action.panel };
    case 'SET_NAV_MODE': return { ...state, navMode: action.mode };
    default: return state;
  }
}

// Driving profile configs
export const drivingProfiles: Record<DrivingProfile, { label: string; icon: string; description: string }> = {
  standard: { label: 'Standard', icon: '🚗', description: 'Balanced route optimization' },
  eco: { label: 'Eco', icon: '🌱', description: 'Minimize fuel & emissions' },
  fast: { label: 'Fast', icon: '⚡', description: 'Fastest route, highways preferred' },
  truck: { label: 'Truck', icon: '🚛', description: 'Truck-safe routes, weight limits' },
  motorcycle: { label: 'Motorcycle', icon: '🏍️', description: 'Lane splitting, agile routing' },
};

// Map dark style for Google Maps
export const darkMapStyle: google.maps.MapTypeStyle[] = [
  { elementType: "geometry", stylers: [{ color: "#0d0d1a" }] },
  { elementType: "labels.text.stroke", stylers: [{ color: "#0d0d1a" }] },
  { elementType: "labels.text.fill", stylers: [{ color: "#5a6080" }] },
  { featureType: "administrative.locality", elementType: "labels.text.fill", stylers: [{ color: "#8090b0" }] },
  { featureType: "poi", elementType: "labels.text.fill", stylers: [{ color: "#5a6080" }] },
  { featureType: "poi.park", elementType: "geometry", stylers: [{ color: "#0f1a15" }] },
  { featureType: "poi.park", elementType: "labels.text.fill", stylers: [{ color: "#3a5a40" }] },
  { featureType: "road", elementType: "geometry", stylers: [{ color: "#1a1a2e" }] },
  { featureType: "road", elementType: "geometry.stroke", stylers: [{ color: "#1a1a2e" }] },
  { featureType: "road.highway", elementType: "geometry", stylers: [{ color: "#1e2040" }] },
  { featureType: "road.highway", elementType: "geometry.stroke", stylers: [{ color: "#252850" }] },
  { featureType: "road.highway", elementType: "labels.text.fill", stylers: [{ color: "#6a7090" }] },
  { featureType: "road.arterial", elementType: "geometry", stylers: [{ color: "#181830" }] },
  { featureType: "transit", elementType: "geometry", stylers: [{ color: "#151528" }] },
  { featureType: "transit.station", elementType: "labels.text.fill", stylers: [{ color: "#5a6080" }] },
  { featureType: "water", elementType: "geometry", stylers: [{ color: "#080818" }] },
  { featureType: "water", elementType: "labels.text.fill", stylers: [{ color: "#2a3050" }] },
];
