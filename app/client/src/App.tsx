import React, { Suspense } from "react";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import NotFound from "@/pages/NotFound";
import { Route, Switch, useLocation } from "wouter";
import ErrorBoundary from "./components/ErrorBoundary";
import ComponentErrorBoundary from "./components/ComponentErrorBoundary";
import { ThemeProvider } from "./contexts/ThemeContext";
import { NavigationProvider } from "./contexts/NavigationContext";
import { RealDataProvider } from "./contexts/RealDataContext";
import { LanguageProvider } from "./contexts/LanguageContext";
import { VoiceProvider } from "./contexts/VoiceContext";
import { AccessibilityProvider } from "./components/AccessibilityProvider";
import { GANEProvider } from "./contexts/GANEContext";
import { AdminModeProvider } from "./contexts/AdminModeContext";
import Home from "./pages/Home";
import { useAuth } from "./_core/hooks/useAuth";

// ─── Route-Level Code Splitting ───
// AdminPanel is 906 lines — lazy-load to reduce initial bundle
const AdminPanel = React.lazy(() => import("./pages/AdminPanel"));
const JoinByInvite = React.lazy(() => import("./pages/JoinByInvite"));
const NotificationCenter = React.lazy(() => import("./pages/NotificationCenter"));

// ─── Loading Skeleton for Lazy Routes ───
function RouteLoadingSkeleton() {
  return (
    <div className="h-screen w-screen flex items-center justify-center" style={{ background: '#F9FAFB' }}>
      <div className="flex flex-col items-center gap-4">
        <div className="w-12 h-12 rounded-xl animate-pulse" style={{
          background: 'linear-gradient(135deg, oklch(0.65 0.15 220), oklch(0.55 0.18 260))',
        }} />
        <div className="flex gap-1.5">
          {[0, 1, 2].map(i => (
            <div
              key={i}
              className="w-2 h-2 rounded-full animate-pulse"
              style={{
                background: 'oklch(0.65 0.15 220)',
                animationDelay: `${i * 150}ms`,
              }}
            />
          ))}
        </div>
        <p className="text-sm text-gray-400 font-medium tracking-wide">Loading module...</p>
      </div>
    </div>
  );
}

function AdminRoute() {
  const { user } = useAuth();
  const [, setLocation] = useLocation();

  // Only admin users can access admin panel
  if (!user || user.role !== "admin") {
    return <NotFound />;
  }

  return (
    <ComponentErrorBoundary componentName="AdminPanel" variant="admin">
      <Suspense fallback={<RouteLoadingSkeleton />}>
        <AdminPanel
          onSwitchToUser={() => setLocation("/")}
        />
      </Suspense>
    </ComponentErrorBoundary>
  );
}

function Router() {
  // make sure to consider if you need authentication for certain routes
  return (
    <Switch>
      <Route path={"/"} component={Home} />
      <Route path={"/admin"} component={AdminRoute} />
      <Route path="/notifications">
        {() => (
          <ComponentErrorBoundary componentName="NotificationCenter">
            <Suspense fallback={<RouteLoadingSkeleton />}>
              <NotificationCenter />
            </Suspense>
          </ComponentErrorBoundary>
        )}
      </Route>
      <Route path={"/collab/join/:token"}>
        {() => (
          <ComponentErrorBoundary componentName="JoinByInvite">
            <Suspense fallback={<RouteLoadingSkeleton />}>
              <JoinByInvite />
            </Suspense>
          </ComponentErrorBoundary>
        )}
      </Route>
      <Route path={"/404"} component={NotFound} />
      <Route component={NotFound} />
    </Switch>
  );
}

function App() {
  return (
    <ErrorBoundary>
      <ThemeProvider defaultTheme="light">
        <LanguageProvider>
          <VoiceProvider>
            <AccessibilityProvider>
              <TooltipProvider>
                <AdminModeProvider>
                  <NavigationProvider>
                    <RealDataProvider>
                      <GANEProvider>
                        <Toaster />
                        <Router />
                      </GANEProvider>
                    </RealDataProvider>
                  </NavigationProvider>
                </AdminModeProvider>
              </TooltipProvider>
            </AccessibilityProvider>
          </VoiceProvider>
        </LanguageProvider>
      </ThemeProvider>
    </ErrorBoundary>
  );
}

export default App;
