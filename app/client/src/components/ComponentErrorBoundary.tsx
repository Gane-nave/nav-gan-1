/**
 * G.A.N.E — Component-Level Error Boundary
 * ==========================================
 * Granular error boundaries for individual components.
 * Each boundary catches errors within its subtree without
 * crashing the entire application.
 *
 * Features:
 * - Per-component Sentry error tracking with component tags
 * - Context-specific fallback UIs (map, panel, admin, generic)
 * - Retry/recover buttons that re-mount the failed component
 * - Error count tracking with escalation to full reload
 * - Animated transitions for error states
 */
import { Component, type ReactNode, type ErrorInfo } from "react";
import { captureError, addBreadcrumb } from "@/lib/sentry";
import {
  AlertTriangle, RotateCcw, MapPin, Shield, Layers,
  RefreshCw, Home, MessageSquareWarning
} from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";

// ─── Fallback Variant Types ───
type FallbackVariant = "map" | "panel" | "admin" | "collaboration" | "generic";

interface ComponentErrorBoundaryProps {
  children: ReactNode;
  /** Name of the component being wrapped (for Sentry context) */
  componentName: string;
  /** Visual variant for the fallback UI */
  variant?: FallbackVariant;
  /** Optional custom fallback */
  fallback?: ReactNode;
  /** Callback when error occurs */
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
  /** Max retries before suggesting full reload (default: 3) */
  maxRetries?: number;
  /** Optional className for the fallback container */
  className?: string;
}

interface ComponentErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
  retryCount: number;
}

// ─── Variant Configurations ───
const VARIANT_CONFIG: Record<FallbackVariant, {
  icon: typeof AlertTriangle;
  title: string;
  titleHe: string;
  description: string;
  descriptionHe: string;
  gradient: string;
  iconColor: string;
  bgPattern: string;
}> = {
  map: {
    icon: MapPin,
    title: "Map Unavailable",
    titleHe: "המפה אינה זמינה",
    description: "The navigation map encountered an error. Your data is safe.",
    descriptionHe: "מפת הניווט נתקלה בשגיאה. הנתונים שלך בטוחים.",
    gradient: "linear-gradient(135deg, oklch(0.25 0.08 220), oklch(0.2 0.06 240))",
    iconColor: "oklch(0.7 0.15 220)",
    bgPattern: "radial-gradient(circle at 30% 70%, oklch(0.3 0.1 220 / 0.3), transparent 50%)",
  },
  panel: {
    icon: Layers,
    title: "Panel Error",
    titleHe: "שגיאת פאנל",
    description: "This panel failed to load. Other panels are unaffected.",
    descriptionHe: "הפאנל נכשל בטעינה. פאנלים אחרים לא מושפעים.",
    gradient: "linear-gradient(135deg, oklch(0.2 0.06 280), oklch(0.18 0.05 300))",
    iconColor: "oklch(0.7 0.15 280)",
    bgPattern: "radial-gradient(circle at 70% 30%, oklch(0.3 0.08 280 / 0.2), transparent 50%)",
  },
  admin: {
    icon: Shield,
    title: "Admin Panel Error",
    titleHe: "שגיאת פאנל ניהול",
    description: "The admin panel encountered an error. Please retry or contact support.",
    descriptionHe: "פאנל הניהול נתקל בשגיאה. נסה שוב או פנה לתמיכה.",
    gradient: "linear-gradient(135deg, oklch(0.25 0.1 25), oklch(0.2 0.08 15))",
    iconColor: "oklch(0.7 0.18 25)",
    bgPattern: "radial-gradient(circle at 50% 50%, oklch(0.3 0.1 25 / 0.2), transparent 50%)",
  },
  collaboration: {
    icon: MessageSquareWarning,
    title: "Collaboration Error",
    titleHe: "שגיאת שיתוף פעולה",
    description: "The collaboration module encountered an error. Your session data is preserved.",
    descriptionHe: "מודול השיתוף נתקל בשגיאה. נתוני הסשן שלך נשמרו.",
    gradient: "linear-gradient(135deg, oklch(0.25 0.08 160), oklch(0.2 0.06 180))",
    iconColor: "oklch(0.7 0.15 160)",
    bgPattern: "radial-gradient(circle at 40% 60%, oklch(0.3 0.1 160 / 0.3), transparent 50%)",
  },
  generic: {
    icon: AlertTriangle,
    title: "Component Error",
    titleHe: "שגיאת רכיב",
    description: "This component encountered an error. The rest of the app is unaffected.",
    descriptionHe: "רכיב זה נתקל בשגיאה. שאר האפליקציה לא מושפעת.",
    gradient: "linear-gradient(135deg, oklch(0.22 0.04 240), oklch(0.18 0.03 260))",
    iconColor: "oklch(0.6 0.1 240)",
    bgPattern: "radial-gradient(circle at 50% 50%, oklch(0.25 0.05 240 / 0.2), transparent 50%)",
  },
};

class ComponentErrorBoundary extends Component<ComponentErrorBoundaryProps, ComponentErrorBoundaryState> {
  constructor(props: ComponentErrorBoundaryProps) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null,
      retryCount: 0,
    };
  }

  static getDerivedStateFromError(error: Error): Partial<ComponentErrorBoundaryState> {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    this.setState((prev) => ({
      errorInfo,
      retryCount: prev.retryCount + 1,
    }));

    // Report to Sentry with component-specific context
    captureError(error, {
      componentStack: errorInfo.componentStack,
      boundaryName: this.props.componentName,
      variant: this.props.variant || "generic",
      retryCount: this.state.retryCount,
      url: window.location.href,
      timestamp: new Date().toISOString(),
    });

    // Add breadcrumb for debugging trail
    addBreadcrumb(
      "component-error-boundary",
      `Error in ${this.props.componentName} (${this.props.variant || "generic"})`,
      {
        errorMessage: error.message,
        errorName: error.name,
        retryCount: this.state.retryCount,
      }
    );

    // Log for development
    console.error(`[ComponentErrorBoundary:${this.props.componentName}] Caught error:`, error);

    // Notify parent if callback provided
    this.props.onError?.(error, errorInfo);
  }

  handleRetry = (): void => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
    });
  };

  handleFullReload = (): void => {
    window.location.reload();
  };

  handleGoHome = (): void => {
    window.location.href = "/";
  };

  render(): ReactNode {
    if (!this.state.hasError) {
      return this.props.children;
    }

    // Use custom fallback if provided
    if (this.props.fallback) {
      return this.props.fallback;
    }

    const variant = this.props.variant || "generic";
    const config = VARIANT_CONFIG[variant];
    const maxRetries = this.props.maxRetries ?? 3;
    const exhaustedRetries = this.state.retryCount >= maxRetries;
    const { error } = this.state;
    const isDev = import.meta.env.DEV;
    const Icon = config.icon;

    return (
      <div
        className={`flex items-center justify-center ${this.props.className || ""}`}
        style={{
          background: config.gradient,
          minHeight: variant === "map" ? "100%" : "200px",
          width: "100%",
          position: "relative",
          overflow: "hidden",
          borderRadius: variant === "map" ? 0 : "12px",
        }}
      >
        {/* Background pattern */}
        <div
          className="absolute inset-0 pointer-events-none"
          style={{ background: config.bgPattern }}
        />

        {/* Animated grid lines */}
        <div
          className="absolute inset-0 pointer-events-none opacity-[0.04]"
          style={{
            backgroundImage: `
              linear-gradient(${config.iconColor} 1px, transparent 1px),
              linear-gradient(90deg, ${config.iconColor} 1px, transparent 1px)
            `,
            backgroundSize: "40px 40px",
          }}
        />

        <div className="relative z-10 flex flex-col items-center p-6 max-w-sm text-center">
          {/* Error Icon */}
          <div
            className="w-14 h-14 rounded-2xl flex items-center justify-center mb-4"
            style={{
              background: `${config.iconColor}22`,
              border: `1px solid ${config.iconColor}44`,
              boxShadow: `0 0 30px ${config.iconColor}20`,
            }}
          >
            <Icon size={24} style={{ color: config.iconColor }} />
          </div>

          {/* Title */}
          <h3
            className="text-base font-bold mb-1.5 tracking-wide"
            style={{ color: "oklch(0.9 0 0)" }}
          >
            {config.title}
          </h3>

          {/* Description */}
          <p
            className="text-xs mb-4 leading-relaxed max-w-[280px]"
            style={{ color: "oklch(0.65 0 0)" }}
          >
            {config.description}
          </p>

          {/* Error details (dev mode) */}
          {isDev && error && (
            <div
              className="w-full rounded-lg p-3 mb-4 text-left"
              style={{
                background: "oklch(0.15 0 0 / 0.5)",
                border: "1px solid oklch(0.3 0 0 / 0.3)",
              }}
            >
              <p
                className="text-[10px] font-mono break-all leading-relaxed"
                style={{ color: "oklch(0.7 0.1 25)" }}
              >
                {error.name}: {error.message}
              </p>
            </div>
          )}

          {/* Action Buttons */}
          <div className="flex gap-2 w-full">
            {!exhaustedRetries ? (
              <button
                onClick={this.handleRetry}
                className="flex-1 flex items-center justify-center gap-1.5 px-3 py-2.5 rounded-lg text-xs font-medium transition-all hover:opacity-90 active:scale-[0.97] cursor-pointer"
                style={{
                  background: config.iconColor,
                  color: "oklch(0.15 0 0)",
                  boxShadow: `0 4px 16px ${config.iconColor}40`,
                }}
              >
                <RotateCcw size={13} />
                Retry ({maxRetries - this.state.retryCount} left)
              </button>
            ) : (
              <button
                onClick={this.handleFullReload}
                className="flex-1 flex items-center justify-center gap-1.5 px-3 py-2.5 rounded-lg text-xs font-medium transition-all hover:opacity-90 active:scale-[0.97] cursor-pointer"
                style={{
                  background: "oklch(0.65 0.18 25)",
                  color: "oklch(0.98 0 0)",
                  boxShadow: "0 4px 16px oklch(0.65 0.18 25 / 0.4)",
                }}
              >
                <RefreshCw size={13} />
                Reload App
              </button>
            )}

            {variant === "map" && (
              <button
                onClick={this.handleGoHome}
                className="flex items-center justify-center gap-1.5 px-3 py-2.5 rounded-lg text-xs font-medium transition-all hover:opacity-80 active:scale-[0.97] cursor-pointer"
                style={{
                  background: "oklch(0.25 0.03 240)",
                  color: "oklch(0.7 0 0)",
                  border: "1px solid oklch(0.35 0.03 240)",
                }}
              >
                <Home size={13} />
              </button>
            )}
          </div>

          {/* Retry exhaustion message */}
          {exhaustedRetries && (
            <p
              className="text-[10px] mt-3"
              style={{ color: "oklch(0.5 0 0)" }}
            >
              Multiple retries failed. A full reload may resolve the issue.
            </p>
          )}
        </div>
      </div>
    );
  }
}

export default ComponentErrorBoundary;
