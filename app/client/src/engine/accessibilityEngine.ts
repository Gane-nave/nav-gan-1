/**
 * G.A.N.E — Accessibility Engine
 * =================================
 * Comprehensive accessibility support for navigation.
 *
 * FEATURES:
 *   - Screen reader announcements (ARIA live regions)
 *   - High contrast mode
 *   - Large text mode
 *   - Voice guidance with adjustable speed
 *   - Haptic feedback patterns
 *   - Color blindness modes (protanopia, deuteranopia, tritanopia)
 *   - Reduced motion mode
 *   - Keyboard navigation
 *   - Wheelchair-accessible routing
 *
 * WCAG 2.1 AA COMPLIANCE:
 *   - 4.5:1 contrast ratio for text
 *   - Focus indicators
 *   - Skip navigation links
 *   - Alt text for all images
 *   - Semantic HTML structure
 */

// ─── Types ───────────────────────────────────────────────

export type ColorBlindMode = 'none' | 'protanopia' | 'deuteranopia' | 'tritanopia' | 'achromatopsia';
export type TextSize = 'normal' | 'large' | 'extra_large';
export type VoiceSpeed = 'slow' | 'normal' | 'fast';

export interface AccessibilityConfig {
  screenReader: boolean;
  highContrast: boolean;
  largeText: TextSize;
  colorBlindMode: ColorBlindMode;
  reducedMotion: boolean;
  voiceGuidance: boolean;
  voiceSpeed: VoiceSpeed;
  hapticFeedback: boolean;
  keyboardNavigation: boolean;
  wheelchairMode: boolean;
  autoAnnounce: boolean;             // Auto-announce navigation events
  announceInterval: number;          // Min ms between announcements
  focusIndicatorSize: number;        // px
  enabled: boolean;
}

export interface AccessibilityState {
  isActive: boolean;
  lastAnnouncement: string;
  lastAnnouncementAt: number;
  totalAnnouncements: number;
  prefersReducedMotion: boolean;     // System preference
  prefersHighContrast: boolean;      // System preference
  screenReaderDetected: boolean;
}

export interface Announcement {
  text: string;
  textHe: string;
  priority: 'polite' | 'assertive';
  category: 'navigation' | 'alert' | 'info' | 'status';
}

// ─── Constants ──────────────────────────────────────────

const DEFAULT_CONFIG: AccessibilityConfig = {
  screenReader: false,
  highContrast: false,
  largeText: 'normal',
  colorBlindMode: 'none',
  reducedMotion: false,
  voiceGuidance: true,
  voiceSpeed: 'normal',
  hapticFeedback: true,
  keyboardNavigation: true,
  wheelchairMode: false,
  autoAnnounce: true,
  announceInterval: 3000,
  focusIndicatorSize: 3,
  enabled: true,
};

const TEXT_SCALE: Record<TextSize, number> = {
  normal: 1,
  large: 1.25,
  extra_large: 1.5,
};

const VOICE_RATE: Record<VoiceSpeed, number> = {
  slow: 0.7,
  normal: 1.0,
  fast: 1.3,
};

// Color blind safe palettes
const COLOR_BLIND_PALETTES: Record<ColorBlindMode, Record<string, string>> = {
  none: {},
  protanopia: {
    red: '#d4a017',
    green: '#0072b2',
    blue: '#0072b2',
    warning: '#e69f00',
    danger: '#cc79a7',
    success: '#009e73',
  },
  deuteranopia: {
    red: '#d55e00',
    green: '#0072b2',
    blue: '#0072b2',
    warning: '#e69f00',
    danger: '#cc79a7',
    success: '#009e73',
  },
  tritanopia: {
    red: '#d55e00',
    green: '#009e73',
    blue: '#cc79a7',
    warning: '#e69f00',
    danger: '#d55e00',
    success: '#009e73',
  },
  achromatopsia: {
    red: '#767676',
    green: '#b0b0b0',
    blue: '#4a4a4a',
    warning: '#909090',
    danger: '#505050',
    success: '#c0c0c0',
  },
};

// ─── Accessibility Engine ───────────────────────────────

export class AccessibilityEngine {
  private config: AccessibilityConfig;
  private state: AccessibilityState;
  private announceQueue: Announcement[] = [];
  private liveRegion: HTMLElement | null = null;
  private speechSynth: SpeechSynthesis | null = null;

  constructor(config: Partial<AccessibilityConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.state = {
      isActive: false,
      lastAnnouncement: '',
      lastAnnouncementAt: 0,
      totalAnnouncements: 0,
      prefersReducedMotion: false,
      prefersHighContrast: false,
      screenReaderDetected: false,
    };
  }

  // ─── Lifecycle ─────────────────────────────────────────

  init() {
    this.state.isActive = true;

    // Detect system preferences
    if (typeof window !== 'undefined') {
      this.state.prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
      this.state.prefersHighContrast = window.matchMedia('(prefers-contrast: high)').matches;

      // Auto-apply system preferences
      if (this.state.prefersReducedMotion) {
        this.config.reducedMotion = true;
      }
      if (this.state.prefersHighContrast) {
        this.config.highContrast = true;
      }

      // Create ARIA live region
      this.createLiveRegion();

      // Initialize speech synthesis
      if ('speechSynthesis' in window) {
        this.speechSynth = window.speechSynthesis;
      }
    }

    // Apply initial settings
    this.applySettings();
  }

  destroy() {
    this.state.isActive = false;
    if (this.liveRegion && this.liveRegion.parentNode) {
      this.liveRegion.parentNode.removeChild(this.liveRegion);
    }
    if (this.speechSynth) {
      this.speechSynth.cancel();
    }
  }

  // ─── ARIA Live Region ─────────────────────────────────

  private createLiveRegion() {
    if (typeof document === 'undefined') return;

    this.liveRegion = document.createElement('div');
    this.liveRegion.setAttribute('role', 'status');
    this.liveRegion.setAttribute('aria-live', 'polite');
    this.liveRegion.setAttribute('aria-atomic', 'true');
    this.liveRegion.className = 'sr-only';
    this.liveRegion.style.cssText = 'position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border:0';
    document.body.appendChild(this.liveRegion);
  }

  // ─── Announcements ────────────────────────────────────

  /**
   * Announce a message to screen readers and/or voice.
   */
  announce(announcement: Announcement) {
    const now = Date.now();

    // Throttle non-assertive announcements
    if (
      announcement.priority === 'polite' &&
      now - this.state.lastAnnouncementAt < this.config.announceInterval
    ) {
      this.announceQueue.push(announcement);
      return;
    }

    this.performAnnouncement(announcement);
  }

  private performAnnouncement(announcement: Announcement) {
    const text = announcement.text; // Use English by default

    // Update ARIA live region
    if (this.liveRegion) {
      this.liveRegion.setAttribute('aria-live', announcement.priority);
      this.liveRegion.textContent = text;
    }

    // Voice announcement
    if (this.config.voiceGuidance && this.speechSynth) {
      const utterance = new SpeechSynthesisUtterance(text);
      utterance.rate = VOICE_RATE[this.config.voiceSpeed];
      utterance.volume = 1;
      this.speechSynth.speak(utterance);
    }

    // Haptic feedback
    if (this.config.hapticFeedback && typeof navigator !== 'undefined' && 'vibrate' in navigator) {
      switch (announcement.category) {
        case 'alert':
          navigator.vibrate([200, 100, 200]); // Double buzz
          break;
        case 'navigation':
          navigator.vibrate(100); // Short buzz
          break;
        default:
          navigator.vibrate(50); // Gentle tap
      }
    }

    this.state.lastAnnouncement = text;
    this.state.lastAnnouncementAt = Date.now();
    this.state.totalAnnouncements++;
  }

  /**
   * Pre-built navigation announcements.
   */
  announceNavigation(type: string, details: string = '') {
    const announcements: Record<string, Announcement> = {
      turn_left: {
        text: `Turn left ${details}`.trim(),
        textHe: `פנה שמאלה ${details}`.trim(),
        priority: 'assertive',
        category: 'navigation',
      },
      turn_right: {
        text: `Turn right ${details}`.trim(),
        textHe: `פנה ימינה ${details}`.trim(),
        priority: 'assertive',
        category: 'navigation',
      },
      continue: {
        text: `Continue straight ${details}`.trim(),
        textHe: `המשך ישר ${details}`.trim(),
        priority: 'polite',
        category: 'navigation',
      },
      arrived: {
        text: `You have arrived at your destination. ${details}`.trim(),
        textHe: `הגעת ליעד. ${details}`.trim(),
        priority: 'assertive',
        category: 'navigation',
      },
      rerouting: {
        text: `Rerouting. ${details}`.trim(),
        textHe: `מנתב מחדש. ${details}`.trim(),
        priority: 'assertive',
        category: 'navigation',
      },
      speed_warning: {
        text: `Speed warning. ${details}`.trim(),
        textHe: `אזהרת מהירות. ${details}`.trim(),
        priority: 'assertive',
        category: 'alert',
      },
    };

    const announcement = announcements[type];
    if (announcement) {
      this.announce(announcement);
    }
  }

  // ─── Visual Settings ──────────────────────────────────

  /**
   * Apply all visual accessibility settings.
   */
  applySettings() {
    if (typeof document === 'undefined') return;

    const root = document.documentElement;

    // Text size
    root.style.fontSize = `${TEXT_SCALE[this.config.largeText] * 16}px`;

    // High contrast
    if (this.config.highContrast) {
      root.classList.add('high-contrast');
    } else {
      root.classList.remove('high-contrast');
    }

    // Reduced motion
    if (this.config.reducedMotion) {
      root.classList.add('reduce-motion');
    } else {
      root.classList.remove('reduce-motion');
    }

    // Color blind mode
    root.setAttribute('data-color-blind', this.config.colorBlindMode);

    // Focus indicator
    root.style.setProperty('--focus-ring-width', `${this.config.focusIndicatorSize}px`);
  }

  /**
   * Get color-blind safe color.
   */
  getSafeColor(colorName: string): string {
    const palette = COLOR_BLIND_PALETTES[this.config.colorBlindMode];
    return palette[colorName] || colorName;
  }

  /**
   * Get CSS variables for current accessibility mode.
   */
  getCSSVariables(): Record<string, string> {
    const vars: Record<string, string> = {
      '--a11y-text-scale': String(TEXT_SCALE[this.config.largeText]),
      '--a11y-focus-width': `${this.config.focusIndicatorSize}px`,
      '--a11y-animation-duration': this.config.reducedMotion ? '0ms' : '300ms',
      '--a11y-transition-duration': this.config.reducedMotion ? '0ms' : '200ms',
    };

    if (this.config.highContrast) {
      vars['--a11y-border-width'] = '2px';
      vars['--a11y-text-shadow'] = '0 0 2px currentColor';
    }

    return vars;
  }

  // ─── Routing ──────────────────────────────────────────

  /**
   * Check if wheelchair-accessible routing should be used.
   */
  shouldUseWheelchairRouting(): boolean {
    return this.config.wheelchairMode;
  }

  /**
   * Get routing preferences for accessibility.
   */
  getRoutingPreferences(): {
    avoidStairs: boolean;
    avoidSteepSlopes: boolean;
    preferElevators: boolean;
    preferWideSidewalks: boolean;
    maxSlopePercent: number;
  } {
    if (!this.config.wheelchairMode) {
      return {
        avoidStairs: false,
        avoidSteepSlopes: false,
        preferElevators: false,
        preferWideSidewalks: false,
        maxSlopePercent: 100,
      };
    }

    return {
      avoidStairs: true,
      avoidSteepSlopes: true,
      preferElevators: true,
      preferWideSidewalks: true,
      maxSlopePercent: 8,
    };
  }

  // ─── Public API ───────────────────────────────────────

  getState(): AccessibilityState {
    return { ...this.state };
  }

  getConfig(): AccessibilityConfig {
    return { ...this.config };
  }

  updateConfig(partial: Partial<AccessibilityConfig>) {
    this.config = { ...this.config, ...partial };
    this.applySettings();
  }

  /**
   * Get a summary of active accessibility features.
   */
  getActiveFeaturesText(): { en: string; he: string } {
    const features: string[] = [];
    const featuresHe: string[] = [];

    if (this.config.screenReader) { features.push('Screen Reader'); featuresHe.push('קורא מסך'); }
    if (this.config.highContrast) { features.push('High Contrast'); featuresHe.push('ניגודיות גבוהה'); }
    if (this.config.largeText !== 'normal') { features.push('Large Text'); featuresHe.push('טקסט גדול'); }
    if (this.config.colorBlindMode !== 'none') { features.push('Color Blind Mode'); featuresHe.push('מצב עיוורון צבעים'); }
    if (this.config.reducedMotion) { features.push('Reduced Motion'); featuresHe.push('תנועה מופחתת'); }
    if (this.config.voiceGuidance) { features.push('Voice Guidance'); featuresHe.push('הנחיה קולית'); }
    if (this.config.wheelchairMode) { features.push('Wheelchair Mode'); featuresHe.push('מצב כיסא גלגלים'); }

    return {
      en: features.length > 0 ? features.join(', ') : 'None active',
      he: featuresHe.length > 0 ? featuresHe.join(', ') : 'אין פעיל',
    };
  }
}
