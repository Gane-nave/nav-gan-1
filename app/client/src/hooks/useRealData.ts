/**
 * useRealData — React hook for REAL live data
 * =============================================
 * Provides real-time weather, location, air quality,
 * earthquakes, elevation, and GNSS status to all panels.
 * Auto-refreshes every 60 seconds.
 * NO MOCK DATA — everything is live.
 */

import { useState, useEffect, useCallback, useRef } from 'react';
import {
  type RealDataState,
  initialRealDataState,
  fetchAllRealData,
  watchRealLocation,
  deriveGNSSStatus,
  getTimeOfDay,
  getSeason,
  isHoliday,
} from '../lib/realDataService';

const REFRESH_INTERVAL = 60_000; // 60 seconds

export function useRealData() {
  const [state, setState] = useState<RealDataState>(initialRealDataState);
  const watchIdRef = useRef<number | null>(null);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const refresh = useCallback(async () => {
    try {
      const data = await fetchAllRealData(
        state.location?.latitude,
        state.location?.longitude
      );
      setState(prev => ({
        ...prev,
        ...data,
        timeOfDay: getTimeOfDay(),
        season: getSeason(),
        holiday: isHoliday(),
      }));
    } catch (err) {
      setState(prev => ({
        ...prev,
        errors: [...prev.errors, String(err)],
        isLoading: false,
      }));
    }
  }, [state.location?.latitude, state.location?.longitude]);

  // Initial fetch
  useEffect(() => {
    refresh();
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  // Watch GPS location in real-time
  useEffect(() => {
    watchIdRef.current = watchRealLocation(
      (location) => {
        setState(prev => ({
          ...prev,
          location,
          gnss: deriveGNSSStatus(location),
        }));

        // Bridge to GANE engines — dispatch location for SensorBridge.injectGNSS
        window.dispatchEvent(new CustomEvent('gane:location', {
          detail: {
            latitude: location.latitude,
            longitude: location.longitude,
            altitude: location.altitude,
            accuracy: location.accuracy,
            speed: location.speed,
            heading: location.heading,
          },
        }));
      },
      (error) => {
        console.warn('GPS error:', error.message);
      }
    );

    return () => {
      if (watchIdRef.current !== null) {
        navigator.geolocation.clearWatch(watchIdRef.current);
      }
    };
  }, []);

  // Auto-refresh weather/air quality every 60s
  useEffect(() => {
    intervalRef.current = setInterval(refresh, REFRESH_INTERVAL);
    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, [refresh]);

  return {
    ...state,
    refresh,
  };
}
