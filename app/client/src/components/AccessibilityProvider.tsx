/**
 * G.A.N.E — Accessibility Provider (Lightweight)
 * 
 * Extracted from AccessibilityLayer.tsx so the heavy AccessibilityPanel
 * can be dynamically imported without pulling the provider into the same chunk.
 * 
 * Re-exports: AccessibilityProvider, useA11y
 */
export { AccessibilityProvider, useA11y } from './AccessibilityLayer';
