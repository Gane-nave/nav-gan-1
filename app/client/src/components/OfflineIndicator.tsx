/**
 * G.A.N.E — Offline Indicator
 * 
 * Shows a subtle banner when the app loses network connectivity.
 * Auto-hides when connection is restored.
 */
import { motion, AnimatePresence } from 'framer-motion';
import { WifiOff, Wifi } from 'lucide-react';
import { useOnlineStatus } from '@/hooks/useOnlineStatus';
import { useState, useEffect } from 'react';

export default function OfflineIndicator() {
  const isOnline = useOnlineStatus();
  const [showReconnected, setShowReconnected] = useState(false);
  const [wasOffline, setWasOffline] = useState(false);

  useEffect(() => {
    if (!isOnline) {
      setWasOffline(true);
    } else if (wasOffline) {
      setShowReconnected(true);
      const timer = setTimeout(() => {
        setShowReconnected(false);
        setWasOffline(false);
      }, 3000);
      return () => clearTimeout(timer);
    }
  }, [isOnline, wasOffline]);

  return (
    <AnimatePresence>
      {!isOnline && (
        <motion.div
          initial={{ y: -60, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          exit={{ y: -60, opacity: 0 }}
          transition={{ type: 'spring', stiffness: 300, damping: 30 }}
          className="fixed top-4 left-1/2 -translate-x-1/2 z-[300] flex items-center gap-2 px-4 py-2.5 rounded-xl shadow-lg"
          style={{
            background: 'linear-gradient(135deg, oklch(0.45 0.15 25), oklch(0.40 0.12 30))',
            border: '1px solid oklch(0.55 0.15 25 / 40%)',
            backdropFilter: 'blur(12px)',
          }}
        >
          <WifiOff className="w-4 h-4 text-white" />
          <span className="text-sm font-semibold text-white tracking-wide">
            Offline — Using cached data
          </span>
          <motion.div
            className="w-2 h-2 rounded-full bg-orange-300"
            animate={{ opacity: [1, 0.3, 1] }}
            transition={{ duration: 1.5, repeat: Infinity }}
          />
        </motion.div>
      )}

      {showReconnected && isOnline && (
        <motion.div
          initial={{ y: -60, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          exit={{ y: -60, opacity: 0 }}
          transition={{ type: 'spring', stiffness: 300, damping: 30 }}
          className="fixed top-4 left-1/2 -translate-x-1/2 z-[300] flex items-center gap-2 px-4 py-2.5 rounded-xl shadow-lg"
          style={{
            background: 'linear-gradient(135deg, oklch(0.55 0.15 145), oklch(0.50 0.12 150))',
            border: '1px solid oklch(0.65 0.15 145 / 40%)',
            backdropFilter: 'blur(12px)',
          }}
        >
          <Wifi className="w-4 h-4 text-white" />
          <span className="text-sm font-semibold text-white tracking-wide">
            Back online
          </span>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
