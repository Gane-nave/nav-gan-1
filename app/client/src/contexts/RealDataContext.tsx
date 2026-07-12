/**
 * RealDataContext — Global provider for REAL live data
 * =====================================================
 * Wraps the entire app so every panel can access
 * real weather, location, air quality, earthquakes, etc.
 */

import React, { createContext, useContext } from 'react';
import { useRealData } from '../hooks/useRealData';
import type { RealDataState } from '../lib/realDataService';

interface RealDataContextType extends RealDataState {
  refresh: () => Promise<void>;
}

const RealDataCtx = createContext<RealDataContextType | null>(null);

export function RealDataProvider({ children }: { children: React.ReactNode }) {
  const realData = useRealData();

  return (
    <RealDataCtx.Provider value={realData}>
      {children}
    </RealDataCtx.Provider>
  );
}

export function useRealDataContext(): RealDataContextType {
  const ctx = useContext(RealDataCtx);
  if (!ctx) {
    throw new Error('useRealDataContext must be used within RealDataProvider');
  }
  return ctx;
}
