/**
 * G.A.N.E — Live Status Bar
 * 
 * Top-right real-time status display showing FPS, weather,
 * GNSS accuracy, network status, and current time.
 */
import { useState, useEffect, useRef } from "react";
import { motion } from "framer-motion";
import { Wifi } from "lucide-react";
import { useRealDataContext } from "@/contexts/RealDataContext";
import useAnimationPerformance from "@/hooks/useAnimationPerformance";
import { COLORS } from "./homeConstants";
import NotificationBell from "@/components/NotificationBell";

export default function LiveStatusBar({ color }: { color: string }) {
  const [time, setTime] = useState(new Date());
  const realData = useRealDataContext();
  const [fps, setFps] = useState(60);
  const { shouldAnimateLight } = useAnimationPerformance();

  // CONSOLIDATED: Single interval for time + FPS measurement
  useEffect(() => {
    let animId = 0;
    let tick = 0;
    const interval = setInterval(() => {
      tick++;
      setTime(new Date());
      // FPS measurement every 5 seconds (brief rAF burst, samples for 1s)
      if (tick % 5 === 0 && !document.hidden) {
        let count = 0;
        const start = performance.now();
        const measure = () => {
          if (document.hidden) return;
          count++;
          if (performance.now() - start < 1000) {
            animId = requestAnimationFrame(measure);
          } else {
            setFps(count);
          }
        };
        animId = requestAnimationFrame(measure);
      }
    }, 1000);
    return () => { cancelAnimationFrame(animId); clearInterval(interval); };
  }, []);

  const gnssColor = realData.gnss?.accuracy && realData.gnss.accuracy < 20
    ? COLORS.green
    : realData.gnss?.accuracy && realData.gnss.accuracy < 50
      ? COLORS.orange
      : COLORS.red;

  return (
    <motion.div
      className="fixed top-3 right-3 z-30 flex items-center gap-3"
      initial={{ opacity: 0, y: -10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: 0.5 }}
    >
      {/* FPS */}
      <div className="flex items-center gap-1.5 px-3 py-2 rounded-lg"
        style={{
          background: '#FFFFFF',
          border: '1px solid #E5E7EB',
          boxShadow: '0 1px 3px rgba(0,0,0,0.06)',
        }}>
        <span className="text-[10px] font-mono" style={{ color: fps > 50 ? COLORS.green : COLORS.orange }}>
          {fps}fps
        </span>
      </div>

      {/* Weather pill */}
      {realData.weather && (
        <div className="flex items-center gap-2 px-3 py-2 rounded-lg"
          style={{
            background: '#FFFFFF',
            border: '1px solid #E5E7EB',
            boxShadow: '0 1px 3px rgba(0,0,0,0.06)',
          }}>
          <span className="text-xs">{realData.weather.weatherIcon}</span>
          <span className="text-[11px] font-mono text-gray-500">{realData.weather.temperature}°</span>
        </div>
      )}

      {/* GNSS status */}
      <div className="flex items-center gap-2 px-3 py-2 rounded-lg"
        style={{
          background: '#FFFFFF',
          border: '1px solid #E5E7EB',
          boxShadow: '0 1px 3px rgba(0,0,0,0.06)',
        }}>
        <motion.div
          className="w-2 h-2 rounded-full"
          style={{ background: gnssColor, boxShadow: `0 0 6px ${gnssColor}` }}
          animate={shouldAnimateLight ? { opacity: [1, 0.4, 1] } : { opacity: 0.7 }}
          transition={shouldAnimateLight ? { duration: 2, repeat: Infinity } : { duration: 0 }}
        />
        <span className="text-[10px] font-mono font-bold" style={{ color: gnssColor }}>
          {realData.gnss ? `±${realData.gnss.accuracy.toFixed(0)}m` : 'GPS'}
        </span>
        {realData.gnss && (
          <span className="text-[8px] font-mono text-gray-400">
            {realData.gnss.satellitesUsed}sat
          </span>
        )}
      </div>

      {/* Network */}
      <div className="flex items-center gap-2 px-3 py-2 rounded-lg"
        style={{
          background: '#FFFFFF',
          border: '1px solid #E5E7EB',
          boxShadow: '0 1px 3px rgba(0,0,0,0.06)',
        }}>
        <Wifi className="w-3.5 h-3.5" style={{ color: COLORS.cyan }} />
        <span className="text-[10px] font-mono text-gray-400">5G</span>
      </div>

      {/* Notification Bell */}
      <NotificationBell />

      {/* Time */}
      <div className="px-3.5 py-2 rounded-lg"
        style={{
          background: 'rgba(6,10,20,0.85)',
          border: '1px solid rgba(255,255,255,0.06)',
        }}>
        <span className="text-[12px] font-mono font-bold text-white/70">
          {time.toLocaleTimeString('he-IL', { hour: '2-digit', minute: '2-digit', second: '2-digit' })}
        </span>
      </div>
    </motion.div>
  );
}
