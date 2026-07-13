/**
 * G.A.N.E — Error Boundary with Sentry Integration
 * ===================================================
 * Catches React rendering errors, reports to Sentry,
 * and shows a user-friendly crash recovery screen.
 */
import { cn } from "@/lib/utils";
import { captureError, addBreadcrumb } from "@/lib/sentry";
import { AlertTriangle, RotateCcw, Home, Bug, ChevronDown, ChevronUp } from "lucide-react";
import { Component, type ReactNode, type ErrorInfo } from "react";

interface Props {
  children: ReactNode;
  /** Optional fallback component */
  fallback?: ReactNode;
  /** Optional boundary name for Sentry context */
  boundaryName?: string;
  /** Whether to show error details (dev mode) */
  showDetails?: boolean;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
  eventId: string | null;
  showStack: boolean;
}

class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null,
      eventId: null,
      showStack: false,
    };
  }

  static getDerivedStateFromError(error: Error): Partial<State> {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    this.setState({ errorInfo });

    // Report to Sentry with full context
    captureError(error, {
      componentStack: errorInfo.componentStack,
      boundaryName: this.props.boundaryName || "root",
      url: window.location.href,
      timestamp: new Date().toISOString(),
    });

    // Add breadcrumb for debugging trail
    addBreadcrumb("error-boundary", `Error caught in ${this.props.boundaryName || "root"}`, {
      errorMessage: error.message,
      errorName: error.name,
    });

    // Log for development
    console.error("[ErrorBoundary] Caught error:", error);
    console.error("[ErrorBoundary] Component stack:", errorInfo.componentStack);
  }

  handleReload = (): void => {
    window.location.reload();
  };

  handleGoHome = (): void => {
    window.location.href = "/";
  };

  handleRetry = (): void => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
      eventId: null,
    });
  };

  toggleStack = (): void => {
    this.setState((prev) => ({ showStack: !prev.showStack }));
  };

  render(): ReactNode {
    if (this.state.hasError) {
      // Use custom fallback if provided
      if (this.props.fallback) {
        return this.props.fallback;
      }

      const { error, errorInfo, showStack } = this.state;
      const isDev = import.meta.env.DEV;

      return (
        <div className="flex items-center justify-center min-h-screen p-6"
          style={{
            background: "linear-gradient(135deg, #f8fafc 0%, #e2e8f0 50%, #f1f5f9 100%)",
          }}
        >
          <div className="flex flex-col items-center w-full max-w-lg">
            {/* Error Icon with pulse animation */}
            <div className="relative mb-8">
              <div
                className="absolute inset-0 rounded-full animate-ping opacity-20"
                style={{ background: "oklch(0.65 0.2 25)" }}
              />
              <div
                className="relative w-20 h-20 rounded-full flex items-center justify-center"
                style={{
                  background: "linear-gradient(135deg, oklch(0.65 0.2 25), oklch(0.55 0.18 15))",
                  boxShadow: "0 8px 32px oklch(0.65 0.2 25 / 0.3)",
                }}
              >
                <AlertTriangle size={36} className="text-white" />
              </div>
            </div>

            {/* Error Message */}
            <h2 className="text-2xl font-bold text-gray-900 mb-2 text-center">
              Something went wrong
            </h2>
            <p className="text-gray-500 text-center mb-2 text-sm max-w-sm">
              An unexpected error occurred. The error has been automatically reported to our team.
            </p>

            {/* Error name/message summary */}
            {error && (
              <div className="w-full rounded-xl border border-red-200 bg-red-50 p-4 mb-6">
                <p className="text-sm font-mono text-red-700 break-all">
                  {error.name}: {error.message}
                </p>
              </div>
            )}

            {/* Action Buttons */}
            <div className="flex gap-3 mb-6 w-full">
              <button
                onClick={this.handleRetry}
                className={cn(
                  "flex-1 flex items-center justify-center gap-2 px-4 py-3 rounded-xl",
                  "text-white font-medium text-sm",
                  "hover:opacity-90 cursor-pointer transition-all",
                  "active:scale-[0.98]"
                )}
                style={{
                  background: "linear-gradient(135deg, oklch(0.65 0.15 220), oklch(0.55 0.18 260))",
                  boxShadow: "0 4px 16px oklch(0.65 0.15 220 / 0.3)",
                }}
              >
                <RotateCcw size={16} />
                Try Again
              </button>

              <button
                onClick={this.handleGoHome}
                className={cn(
                  "flex-1 flex items-center justify-center gap-2 px-4 py-3 rounded-xl",
                  "bg-white text-gray-700 font-medium text-sm border border-gray-200",
                  "hover:bg-gray-50 cursor-pointer transition-all",
                  "active:scale-[0.98]"
                )}
                style={{ boxShadow: "0 2px 8px rgba(0,0,0,0.06)" }}
              >
                <Home size={16} />
                Go Home
              </button>
            </div>

            {/* Reload fallback */}
            <button
              onClick={this.handleReload}
              className="text-xs text-gray-400 hover:text-gray-600 transition-colors cursor-pointer underline underline-offset-2"
            >
              Or reload the entire page
            </button>

            {/* Stack trace (dev mode or togglable) */}
            {(isDev || this.props.showDetails) && error?.stack && (
              <div className="w-full mt-6">
                <button
                  onClick={this.toggleStack}
                  className="flex items-center gap-1.5 text-xs text-gray-400 hover:text-gray-600 transition-colors cursor-pointer mb-2"
                >
                  <Bug size={12} />
                  <span>Technical Details</span>
                  {showStack ? <ChevronUp size={12} /> : <ChevronDown size={12} />}
                </button>

                {showStack && (
                  <div className="rounded-xl border border-gray-200 bg-gray-900 p-4 overflow-auto max-h-64">
                    <pre className="text-xs text-gray-300 whitespace-pre-wrap font-mono leading-relaxed">
                      {error.stack}
                    </pre>
                    {errorInfo?.componentStack && (
                      <>
                        <hr className="border-gray-700 my-3" />
                        <p className="text-xs text-gray-500 mb-1 font-medium">Component Stack:</p>
                        <pre className="text-xs text-gray-400 whitespace-pre-wrap font-mono leading-relaxed">
                          {errorInfo.componentStack}
                        </pre>
                      </>
                    )}
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;
