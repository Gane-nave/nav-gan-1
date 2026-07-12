/**
 * useAnimationPerformance — Central hook for animation performance decisions.
 *
 * Provides:
 * - prefersReducedMotion: respects OS-level "reduce motion" setting
 * - isMobile: viewport-based mobile detection
 * - shouldAnimate: combined flag — false when reduced motion OR mobile
 * - isTabVisible: tracks document visibility for rAF guards
 */
import { useState, useEffect, useSyncExternalStore } from "react";

// ── Reduced Motion ──
function subscribeReducedMotion(cb: () => void) {
  const mq = window.matchMedia("(prefers-reduced-motion: reduce)");
  mq.addEventListener("change", cb);
  return () => mq.removeEventListener("change", cb);
}
function getReducedMotion() {
  return typeof window !== "undefined"
    ? window.matchMedia("(prefers-reduced-motion: reduce)").matches
    : false;
}

// ── Mobile Detection ──
function subscribeMobile(cb: () => void) {
  const mq = window.matchMedia("(max-width: 767px)");
  mq.addEventListener("change", cb);
  return () => mq.removeEventListener("change", cb);
}
function getMobile() {
  return typeof window !== "undefined"
    ? window.matchMedia("(max-width: 767px)").matches
    : false;
}

// ── Tab Visibility ──
function subscribeVisibility(cb: () => void) {
  document.addEventListener("visibilitychange", cb);
  return () => document.removeEventListener("visibilitychange", cb);
}
function getVisibility() {
  return typeof document !== "undefined" ? !document.hidden : true;
}

export function useAnimationPerformance() {
  const prefersReducedMotion = useSyncExternalStore(subscribeReducedMotion, getReducedMotion, () => false);
  const isMobile = useSyncExternalStore(subscribeMobile, getMobile, () => false);
  const isTabVisible = useSyncExternalStore(subscribeVisibility, getVisibility, () => true);

  return {
    prefersReducedMotion,
    isMobile,
    isTabVisible,
    /** True when heavy animations should run (desktop + no reduced motion + tab visible) */
    shouldAnimate: !prefersReducedMotion && !isMobile && isTabVisible,
    /** True when lightweight animations are OK (not reduced motion + tab visible) */
    shouldAnimateLight: !prefersReducedMotion && isTabVisible,
  };
}

export default useAnimationPerformance;
