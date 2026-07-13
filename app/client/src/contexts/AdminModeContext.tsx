/**
 * G.A.N.E — Admin Mode Context
 * =============================
 * Provides a global toggle between admin mode and user mode.
 * Only available to users with admin role.
 * Persists the mode preference in localStorage.
 */
import { createContext, useContext, useState, useCallback, type ReactNode } from "react";
import { useAuth } from "@/_core/hooks/useAuth";

interface AdminModeContextType {
  /** Whether admin mode is currently active */
  isAdminMode: boolean;
  /** Whether the current user is an admin (can toggle) */
  isAdmin: boolean;
  /** Toggle between admin and user mode */
  toggleMode: () => void;
  /** Explicitly set admin mode */
  setAdminMode: (enabled: boolean) => void;
}

const AdminModeContext = createContext<AdminModeContextType>({
  isAdminMode: false,
  isAdmin: false,
  toggleMode: () => {},
  setAdminMode: () => {},
});

export function AdminModeProvider({ children }: { children: ReactNode }) {
  const { user } = useAuth();
  const isAdmin = user?.role === "admin";

  const [isAdminMode, setIsAdminMode] = useState(() => {
    if (typeof window === "undefined") return false;
    return localStorage.getItem("gane-admin-mode") === "true";
  });

  const setAdminMode = useCallback((enabled: boolean) => {
    setIsAdminMode(enabled);
    localStorage.setItem("gane-admin-mode", String(enabled));
  }, []);

  const toggleMode = useCallback(() => {
    setAdminMode(!isAdminMode);
  }, [isAdminMode, setAdminMode]);

  return (
    <AdminModeContext.Provider
      value={{
        isAdminMode: isAdmin && isAdminMode,
        isAdmin,
        toggleMode,
        setAdminMode,
      }}
    >
      {children}
    </AdminModeContext.Provider>
  );
}

export function useAdminMode() {
  return useContext(AdminModeContext);
}
