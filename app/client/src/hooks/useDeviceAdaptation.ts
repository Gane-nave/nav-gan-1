/**
 * useDeviceAdaptation — Autonomous device detection & adaptive layout system
 * 
 * Detects and reacts to:
 * - Device type: phone, tablet, desktop, ultrawide
 * - Orientation: portrait, landscape
 * - Screen dimensions, DPR, safe areas
 * - Touch capability, pointer precision
 * - OS type: iOS, Android, Windows, macOS, Linux
 * - Standalone PWA mode
 * - Connection quality
 * - Reduced motion / high contrast preferences
 */
import { useSyncExternalStore } from "react";

export type DeviceType = 'phone' | 'tablet' | 'desktop' | 'ultrawide';
export type Orientation = 'portrait' | 'landscape';
export type OSType = 'ios' | 'android' | 'windows' | 'macos' | 'linux' | 'unknown';
export type PointerType = 'coarse' | 'fine' | 'none';
export type ConnectionQuality = 'offline' | 'slow-2g' | '2g' | '3g' | '4g' | 'unknown';

export interface DeviceAdaptation {
  deviceType: DeviceType;
  os: OSType;
  orientation: Orientation;
  isStandalone: boolean;
  screenWidth: number;
  screenHeight: number;
  viewportWidth: number;
  viewportHeight: number;
  dpr: number;
  hasTouch: boolean;
  pointerType: PointerType;
  hasHover: boolean;
  isOnline: boolean;
  connectionQuality: ConnectionQuality;
  prefersReducedMotion: boolean;
  prefersHighContrast: boolean;
  prefersDarkMode: boolean;
  isPhone: boolean;
  isTablet: boolean;
  isDesktop: boolean;
  isUltrawide: boolean;
  isMobile: boolean;
  isPortrait: boolean;
  isLandscape: boolean;
  isRetina: boolean;
  isIOS: boolean;
  isAndroid: boolean;
  sidebarMode: 'hidden' | 'bottom-sheet' | 'collapsed' | 'expanded';
  panelMode: 'fullscreen' | 'sheet' | 'side' | 'floating';
  dockMode: 'compact' | 'standard' | 'expanded';
  searchBarMode: 'minimal' | 'standard' | 'expanded';
  statusBarMode: 'hidden' | 'minimal' | 'full';
  fontScale: number;
  touchTargetSize: number;
  spacing: number;
}

function detectOS(): OSType {
  if (typeof navigator === 'undefined') return 'unknown';
  const ua = navigator.userAgent.toLowerCase();
  const platform = (navigator as any).userAgentData?.platform?.toLowerCase() || '';
  if (/iphone|ipad|ipod/.test(ua) || platform === 'ios') return 'ios';
  if (/android/.test(ua)) return 'android';
  if (/win/.test(platform) || /windows/.test(ua)) return 'windows';
  if (/mac/.test(platform) || /macintosh/.test(ua)) return 'macos';
  if (/linux/.test(platform) || /linux/.test(ua)) return 'linux';
  return 'unknown';
}

function detectDeviceType(width: number): DeviceType {
  if (width < 640) return 'phone';
  if (width < 1024) return 'tablet';
  if (width < 2560) return 'desktop';
  return 'ultrawide';
}

function detectPointerType(): PointerType {
  if (typeof window === 'undefined') return 'fine';
  if (window.matchMedia('(pointer: coarse)').matches) return 'coarse';
  if (window.matchMedia('(pointer: fine)').matches) return 'fine';
  return 'none';
}

function detectConnectionQuality(): ConnectionQuality {
  if (typeof navigator === 'undefined') return 'unknown';
  if (!navigator.onLine) return 'offline';
  const conn = (navigator as any).connection;
  if (!conn) return 'unknown';
  const ect = conn.effectiveType;
  if (ect === 'slow-2g') return 'slow-2g';
  if (ect === '2g') return '2g';
  if (ect === '3g') return '3g';
  if (ect === '4g') return '4g';
  return 'unknown';
}

function isStandaloneMode(): boolean {
  if (typeof window === 'undefined') return false;
  return (
    window.matchMedia('(display-mode: standalone)').matches ||
    (window.navigator as any).standalone === true ||
    document.referrer.includes('android-app://')
  );
}

function getSnapshot(): DeviceAdaptation {
  if (typeof window === 'undefined') return getServerSnapshot();
  const vw = window.innerWidth;
  const vh = window.innerHeight;
  const dpr = window.devicePixelRatio || 1;
  const deviceType = detectDeviceType(vw);
  const orientation: Orientation = vw > vh ? 'landscape' : 'portrait';
  const os = detectOS();
  const pointerType = detectPointerType();
  const connectionQuality = detectConnectionQuality();
  const standalone = isStandaloneMode();
  const isPhone = deviceType === 'phone';
  const isTablet = deviceType === 'tablet';
  const isDesktop = deviceType === 'desktop';
  const isUltrawide = deviceType === 'ultrawide';
  const isMobile = isPhone || isTablet;

  let sidebarMode: DeviceAdaptation['sidebarMode'];
  if (isPhone) sidebarMode = 'bottom-sheet';
  else if (isTablet && orientation === 'portrait') sidebarMode = 'collapsed';
  else if (isTablet) sidebarMode = 'collapsed';
  else sidebarMode = 'expanded';

  let panelMode: DeviceAdaptation['panelMode'];
  if (isPhone) panelMode = 'fullscreen';
  else if (isTablet) panelMode = 'sheet';
  else if (isUltrawide) panelMode = 'floating';
  else panelMode = 'side';

  let dockMode: DeviceAdaptation['dockMode'];
  if (isPhone) dockMode = 'compact';
  else if (isTablet) dockMode = 'standard';
  else dockMode = 'expanded';

  let searchBarMode: DeviceAdaptation['searchBarMode'];
  if (isPhone) searchBarMode = 'minimal';
  else if (isTablet) searchBarMode = 'standard';
  else searchBarMode = 'expanded';

  let statusBarMode: DeviceAdaptation['statusBarMode'];
  if (isPhone) statusBarMode = 'hidden';
  else if (isTablet) statusBarMode = 'minimal';
  else statusBarMode = 'full';

  let fontScale = 1.0;
  if (vw < 360) fontScale = 0.85;
  else if (vw < 640) fontScale = 0.9;
  else if (vw >= 2560) fontScale = 1.1;

  return {
    deviceType, os, orientation, isStandalone: standalone,
    screenWidth: screen.width, screenHeight: screen.height,
    viewportWidth: vw, viewportHeight: vh, dpr,
    hasTouch: 'ontouchstart' in window || navigator.maxTouchPoints > 0,
    pointerType, hasHover: window.matchMedia('(hover: hover)').matches,
    isOnline: navigator.onLine, connectionQuality,
    prefersReducedMotion: window.matchMedia('(prefers-reduced-motion: reduce)').matches,
    prefersHighContrast: window.matchMedia('(prefers-contrast: high)').matches,
    prefersDarkMode: window.matchMedia('(prefers-color-scheme: dark)').matches,
    isPhone, isTablet, isDesktop, isUltrawide, isMobile,
    isPortrait: orientation === 'portrait', isLandscape: orientation === 'landscape',
    isRetina: dpr >= 2, isIOS: os === 'ios', isAndroid: os === 'android',
    sidebarMode, panelMode, dockMode, searchBarMode, statusBarMode,
    fontScale, touchTargetSize: isMobile ? 44 : 32,
    spacing: isPhone ? 0.85 : isTablet ? 0.95 : 1.0,
  };
}

function getServerSnapshot(): DeviceAdaptation {
  return {
    deviceType: 'desktop', os: 'unknown', orientation: 'landscape', isStandalone: false,
    screenWidth: 1920, screenHeight: 1080, viewportWidth: 1920, viewportHeight: 1080, dpr: 1,
    hasTouch: false, pointerType: 'fine', hasHover: true,
    isOnline: true, connectionQuality: 'unknown',
    prefersReducedMotion: false, prefersHighContrast: false, prefersDarkMode: true,
    isPhone: false, isTablet: false, isDesktop: true, isUltrawide: false, isMobile: false,
    isPortrait: false, isLandscape: true, isRetina: false, isIOS: false, isAndroid: false,
    sidebarMode: 'expanded', panelMode: 'side', dockMode: 'expanded',
    searchBarMode: 'expanded', statusBarMode: 'full',
    fontScale: 1.0, touchTargetSize: 32, spacing: 1.0,
  };
}

let cachedSnapshot: DeviceAdaptation | null = null;
const listeners = new Set<() => void>();

function notify() {
  cachedSnapshot = null;
  listeners.forEach(fn => fn());
}

function subscribe(callback: () => void) {
  listeners.add(callback);
  if (listeners.size === 1) {
    window.addEventListener('resize', notify);
    window.addEventListener('orientationchange', notify);
    window.addEventListener('online', notify);
    window.addEventListener('offline', notify);
    const mqRM = window.matchMedia('(prefers-reduced-motion: reduce)');
    const mqHC = window.matchMedia('(prefers-contrast: high)');
    const mqDM = window.matchMedia('(prefers-color-scheme: dark)');
    const mqSA = window.matchMedia('(display-mode: standalone)');
    mqRM.addEventListener('change', notify);
    mqHC.addEventListener('change', notify);
    mqDM.addEventListener('change', notify);
    mqSA.addEventListener('change', notify);
    const conn = (navigator as any).connection;
    if (conn) conn.addEventListener('change', notify);
  }
  return () => {
    listeners.delete(callback);
    if (listeners.size === 0) {
      window.removeEventListener('resize', notify);
      window.removeEventListener('orientationchange', notify);
      window.removeEventListener('online', notify);
      window.removeEventListener('offline', notify);
    }
  };
}

function getCachedSnapshot(): DeviceAdaptation {
  if (!cachedSnapshot) cachedSnapshot = getSnapshot();
  return cachedSnapshot;
}

export function useDeviceAdaptation(): DeviceAdaptation {
  return useSyncExternalStore(subscribe, getCachedSnapshot, getServerSnapshot);
}

export default useDeviceAdaptation;
