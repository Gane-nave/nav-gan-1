/**
 * VoicePlayer — Professional floating audio control bar
 * Appears when TTS is active with play/pause/stop, speed control, progress bar
 */
import { useVoice } from '@/contexts/VoiceContext';
import { useLanguage } from '@/contexts/LanguageContext';
import { Play, Pause, Square, Volume2, Minus, Plus, X } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

export default function VoicePlayer() {
  const voice = useVoice();
  const { t } = useLanguage();

  if (!voice.isPlaying && !voice.isPaused) return null;

  return (
    <AnimatePresence>
      <motion.div
        initial={{ y: 80, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        exit={{ y: 80, opacity: 0 }}
        className="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 w-[min(90vw,520px)]"
      >
        <div className="bg-[oklch(0.12_0.02_264)] border border-white/10 rounded-xl shadow-2xl shadow-black/40 backdrop-blur-xl px-4 py-3">
          {/* Progress bar */}
          <div className="w-full h-1 bg-white/5 rounded-full mb-3 overflow-hidden">
            <motion.div
              className="h-full bg-gradient-to-r from-indigo-500 to-teal-400 rounded-full"
              style={{ width: `${voice.progress}%` }}
              transition={{ duration: 0.3 }}
            />
          </div>

          <div className="flex items-center gap-3">
            {/* Play/Pause */}
            <button
              onClick={() => voice.isPaused ? voice.resume() : voice.pause()}
              className="w-9 h-9 flex items-center justify-center rounded-full bg-indigo-500/20 border border-indigo-500/30 text-indigo-300 hover:bg-indigo-500/30 transition-all"
            >
              {voice.isPaused ? <Play className="w-4 h-4 ml-0.5" /> : <Pause className="w-4 h-4" />}
            </button>

            {/* Stop */}
            <button
              onClick={voice.stop}
              className="w-9 h-9 flex items-center justify-center rounded-full bg-white/5 border border-white/10 text-white/50 hover:text-white/80 hover:bg-white/10 transition-all"
            >
              <Square className="w-3.5 h-3.5" />
            </button>

            {/* Current text preview */}
            <div className="flex-1 min-w-0">
              <div className="text-xs text-white/60 truncate font-['JetBrains_Mono']">
                {voice.currentText}
              </div>
              <div className="text-[10px] text-white/25 mt-0.5">
                {voice.progress}% complete
              </div>
            </div>

            {/* Speed control */}
            <div className="flex items-center gap-1">
              <button
                onClick={() => voice.setSpeed(Math.max(0.5, voice.speed - 0.25))}
                className="w-6 h-6 flex items-center justify-center rounded text-white/30 hover:text-white/60 transition-colors"
              >
                <Minus className="w-3 h-3" />
              </button>
              <span className="font-['JetBrains_Mono'] text-[10px] text-teal-400 w-8 text-center">
                {voice.speed.toFixed(2)}x
              </span>
              <button
                onClick={() => voice.setSpeed(Math.min(3, voice.speed + 0.25))}
                className="w-6 h-6 flex items-center justify-center rounded text-white/30 hover:text-white/60 transition-colors"
              >
                <Plus className="w-3 h-3" />
              </button>
            </div>

            {/* Volume */}
            <Volume2 className="w-3.5 h-3.5 text-white/25" />

            {/* Close */}
            <button
              onClick={voice.stop}
              className="w-6 h-6 flex items-center justify-center rounded text-white/25 hover:text-white/60 transition-colors"
            >
              <X className="w-3.5 h-3.5" />
            </button>
          </div>
        </div>
      </motion.div>
    </AnimatePresence>
  );
}
