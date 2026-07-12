/**
 * G.A.N.E — Voice Command System
 * ═══════════════════════════════════════
 * Real speech recognition using Web Speech API
 * Natural language command processing
 * Visual waveform feedback with holographic UI
 * 
 * Commands supported:
 * - Navigation: "נווט ל...", "Navigate to..."
 * - Search: "חפש...", "Search for..."
 * - Layers: "הפעל שכבת...", "Toggle layer..."
 * - Mode: "מצב לילה", "Night mode"
 * - Zoom: "התקרב", "Zoom in/out"
 * - Emergency: "חירום", "Emergency"
 */
import { useState, useEffect, useRef, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Mic, MicOff, X, Volume2, Zap, Globe, Navigation,
  Search, Layers, Moon, Sun, ZoomIn, ZoomOut, AlertTriangle,
  CheckCircle2, Loader2, Sparkles
} from 'lucide-react';
import { useNavigation } from '@/contexts/NavigationContext';
import { useLanguage } from "@/contexts/LanguageContext";

interface VoiceCommand {
  pattern: RegExp;
  action: string;
  icon: typeof Mic;
  color: string;
  handler: (match: RegExpMatchArray, dispatch: any) => string;
}

const COMMANDS: VoiceCommand[] = [
  {
    pattern: /(?:נווט|navigate|go|drive)\s+(?:ל|to|towards?)\s+(.+)/i,
    action: 'navigate',
    icon: Navigation,
    color: '#2563EB',
    handler: (match, dispatch) => {
      dispatch({ type: 'SET_SEARCH_QUERY', query: match[1] });
      dispatch({ type: 'SET_VIEW', view: 'search' });
      return `מחפש מסלול ל-${match[1]}`;
    } },
  {
    pattern: /(?:חפש|search|find|look for)\s+(.+)/i,
    action: 'search',
    icon: Search,
    color: '#ffaa00',
    handler: (match, dispatch) => {
      dispatch({ type: 'SET_SEARCH_QUERY', query: match[1] });
      dispatch({ type: 'SET_VIEW', view: 'search' });
      return `מחפש: ${match[1]}`;
    } },
  {
    pattern: /(?:מצב לילה|night mode|dark mode)/i,
    action: 'night-mode',
    icon: Moon,
    color: '#7C3AED',
    handler: (_match, dispatch) => {
      dispatch({ type: 'TOGGLE_LAYER', layer: 'night-vision' });
      return 'מצב לילה הופעל';
    } },
  {
    pattern: /(?:מצב יום|day mode|light mode)/i,
    action: 'day-mode',
    icon: Sun,
    color: '#ffaa00',
    handler: (_match, dispatch) => {
      dispatch({ type: 'TOGGLE_LAYER', layer: 'night-vision' });
      return 'מצב יום הופעל';
    } },
  {
    pattern: /(?:הפעל|toggle|show|enable)\s+(?:שכבת?\s*)?(?:תנועה|traffic)/i,
    action: 'toggle-traffic',
    icon: Layers,
    color: '#F97316',
    handler: (_match, dispatch) => {
      dispatch({ type: 'TOGGLE_LAYER', layer: 'traffic' });
      return 'שכבת תנועה הופעלה';
    } },
  {
    pattern: /(?:הפעל|toggle|show|enable)\s+(?:שכבת?\s*)?(?:לוויין|satellite)/i,
    action: 'toggle-satellite',
    icon: Globe,
    color: '#2563EB',
    handler: (_match, dispatch) => {
      dispatch({ type: 'TOGGLE_LAYER', layer: 'satellite' });
      return 'תצוגת לוויין הופעלה';
    } },
  {
    pattern: /(?:התקרב|zoom in|closer)/i,
    action: 'zoom-in',
    icon: ZoomIn,
    color: '#16A34A',
    handler: (_match, _dispatch) => {
      return 'מתקרב';
    } },
  {
    pattern: /(?:התרחק|zoom out|farther)/i,
    action: 'zoom-out',
    icon: ZoomOut,
    color: '#16A34A',
    handler: (_match, _dispatch) => {
      return 'מתרחק';
    } },
  {
    pattern: /(?:חירום|emergency|sos|help)/i,
    action: 'emergency',
    icon: AlertTriangle,
    color: '#DC2626',
    handler: (_match, dispatch) => {
      dispatch({ type: 'SET_NAV_MODE', mode: 'emergency' });
      return 'מצב חירום הופעל!';
    } },
  {
    pattern: /(?:עצור ניווט|stop navigation|cancel route)/i,
    action: 'stop-nav',
    icon: X,
    color: '#DC2626',
    handler: (_match, dispatch) => {
      dispatch({ type: 'STOP_NAVIGATION' });
      return 'ניווט נעצר';
    } },
];

// ═══ Waveform Visualizer ═══
function WaveformVisualizer({ active, analyser }: { active: boolean; analyser: AnalyserNode | null }) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!active || !analyser) return;
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const w = 200;
    const h = 60;
    canvas.width = w * 2;
    canvas.height = h * 2;
    canvas.style.width = w + 'px';
    canvas.style.height = h + 'px';
    ctx.scale(2, 2);

    const bufferLength = analyser.frequencyBinCount;
    const dataArray = new Uint8Array(bufferLength);
    let animId: number;

    const draw = () => {
      animId = requestAnimationFrame(draw);
      if (document.hidden) return;
      analyser.getByteTimeDomainData(dataArray);
      ctx.clearRect(0, 0, w, h);

      // Main waveform
      ctx.lineWidth = 2;
      ctx.strokeStyle = '#2563EB';
      ctx.beginPath();
      const sliceWidth = w / bufferLength;
      let x = 0;
      for (let i = 0; i < bufferLength; i++) {
        const v = dataArray[i] / 128.0;
        const y = (v * h) / 2;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
        x += sliceWidth;
      }
      ctx.stroke();

      // Glow layer
      ctx.lineWidth = 6;
      ctx.strokeStyle = 'rgba(37,99,235,0.15)';
      ctx.beginPath();
      x = 0;
      for (let i = 0; i < bufferLength; i++) {
        const v = dataArray[i] / 128.0;
        const y = (v * h) / 2;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
        x += sliceWidth;
      }
      ctx.stroke();

    };
    animId = requestAnimationFrame(draw);
    return () => cancelAnimationFrame(animId);
  }, [active, analyser]);

  if (!active) return null;

  return <canvas ref={canvasRef} className="w-[200px] h-[60px]" />;
}

// ═══ Main Voice Command Component ═══
export default function VoiceCommandSystem() {
  const { t, dir, lang } = useLanguage();
  const { dispatch, mapRef } = useNavigation();
  const [isListening, setIsListening] = useState(false);
  const [transcript, setTranscript] = useState('');
  const [result, setResult] = useState<{ text: string; icon: typeof Mic; color: string } | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showPanel, setShowPanel] = useState(false);
  const [analyser, setAnalyser] = useState<AnalyserNode | null>(null);
  const recognitionRef = useRef<any>(null);
  const audioContextRef = useRef<AudioContext | null>(null);
  const timeoutRef = useRef<ReturnType<typeof setTimeout>>(undefined);

  const processCommand = useCallback((text: string) => {
    const trimmed = text.trim().toLowerCase();
    
    for (const cmd of COMMANDS) {
      const match = trimmed.match(cmd.pattern);
      if (match) {
        const response = cmd.handler(match, dispatch);
        setResult({ text: response, icon: cmd.icon, color: cmd.color });

        // Handle zoom commands with mapRef
        if (cmd.action === 'zoom-in' && mapRef.current) {
          const currentZoom = mapRef.current.getZoom() || 14;
          mapRef.current.setZoom(currentZoom + 2);
        } else if (cmd.action === 'zoom-out' && mapRef.current) {
          const currentZoom = mapRef.current.getZoom() || 14;
          mapRef.current.setZoom(currentZoom - 2);
        }

        // Clear result after 3 seconds
        setTimeout(() => setResult(null), 3000);
        return;
      }
    }

    // No command matched — treat as search
    if (trimmed.length > 2) {
      dispatch({ type: 'SET_SEARCH_QUERY', query: text });
      dispatch({ type: 'SET_VIEW', view: 'search' });
      setResult({ text: `מחפש: ${text}`, icon: Search, color: '#ffaa00' });
      setTimeout(() => setResult(null), 3000);
    } else {
      setResult({ text: 'לא הבנתי את הפקודה', icon: Sparkles, color: 'rgba(107,114,128,0.8)' });
      setTimeout(() => setResult(null), 2000);
    }
  }, [dispatch, mapRef]);

  const startListening = useCallback(() => {
    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
    if (!SpeechRecognition) {
      setError('הדפדפן לא תומך בזיהוי קולי');
      return;
    }

    setShowPanel(true);
    setTranscript('');
    setResult(null);
    setError(null);

    const recognition = new SpeechRecognition();
    recognition.lang = 'he-IL';
    recognition.interimResults = true;
    recognition.continuous = false;
    recognition.maxAlternatives = 3;

    recognition.onstart = () => {
      setIsListening(true);
      // Setup audio analyser for waveform
      try {
        navigator.mediaDevices.getUserMedia({ audio: true }).then(stream => {
          const audioCtx = new AudioContext();
          const source = audioCtx.createMediaStreamSource(stream);
          const analyserNode = audioCtx.createAnalyser();
          analyserNode.fftSize = 256;
          source.connect(analyserNode);
          audioContextRef.current = audioCtx;
          setAnalyser(analyserNode);
        }).catch(() => {});
      } catch {}
    };

    recognition.onresult = (event: any) => {
      let finalTranscript = '';
      let interimTranscript = '';
      for (let i = event.resultIndex; i < event.results.length; i++) {
        if (event.results[i].isFinal) {
          finalTranscript += event.results[i][0].transcript;
        } else {
          interimTranscript += event.results[i][0].transcript;
        }
      }
      setTranscript(finalTranscript || interimTranscript);
      if (finalTranscript) {
        processCommand(finalTranscript);
      }
    };

    recognition.onerror = (event: any) => {
      if (event.error !== 'no-speech') {
        setError(`שגיאה: ${event.error}`);
      }
      setIsListening(false);
    };

    recognition.onend = () => {
      setIsListening(false);
      audioContextRef.current?.close();
      setAnalyser(null);
      // Auto-close panel after delay
      timeoutRef.current = setTimeout(() => setShowPanel(false), 4000);
    };

    recognitionRef.current = recognition;
    recognition.start();
  }, [processCommand]);

  const stopListening = useCallback(() => {
    recognitionRef.current?.stop();
    setIsListening(false);
  }, []);

  // Cleanup
  useEffect(() => {
    return () => {
      recognitionRef.current?.abort();
      audioContextRef.current?.close();
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, []);

  return (
    <>
      {/* Voice activation button (integrated into the search bar) */}
      {!showPanel && (
        <motion.button
          whileHover={{ scale: 1.1 }}
          whileTap={{ scale: 0.9 }}
          onClick={startListening}
          className="w-10 h-10 rounded-xl flex items-center justify-center cursor-pointer relative overflow-hidden"
          style={{
            background: 'rgba(37,99,235,0.06)',
            border: '1px solid rgba(37,99,235,0.15)' }}
          aria-label="Voice command"
        >
          <Mic className="w-5 h-5" style={{ color: '#2563EB' }} />
          <motion.div
            className="absolute inset-0 rounded-xl"
            animate={{ opacity: [0, 0.15, 0] }}
            transition={{ duration: 2, repeat: Infinity }}
            style={{ background: 'radial-gradient(circle, rgba(37,99,235,0.2), transparent)' }}
          />
        </motion.button>
      )}

      {/* Voice Command Overlay */}
      <AnimatePresence>
        {showPanel && (
          <motion.div
            initial={{ opacity: 0, scale: 0.95, y: 20 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95, y: 20 }}
            transition={{ type: 'spring', stiffness: 300, damping: 25 }}
            className="fixed bottom-24 left-1/2 -translate-x-1/2 z-50 w-[340px]"
          >
            <div className="rounded-2xl overflow-hidden" style={{
              background: 'rgba(4,8,18,0.95)',
              border: '1px solid rgba(37,99,235,0.15)',
              boxShadow: '0 20px 60px rgba(249,250,251,0.85), 0 0 30px rgba(37,99,235,0.08)' }}>
              {/* Header */}
              <div className="flex items-center justify-between px-4 py-3" style={{ borderBottom: '1px solid rgba(229,231,235,0.4)' }}>
                <div className="flex items-center gap-2">
                  <motion.div
                    animate={isListening ? { scale: [1, 1.2, 1], opacity: [1, 0.6, 1] } : {}}
                    transition={{ duration: 1.5, repeat: Infinity }}
                  >
                    <Mic className="w-4 h-4" style={{ color: isListening ? '#2563EB' : 'rgba(156,163,175,0.9)' }} />
                  </motion.div>
                  <span className="text-xs font-bold" style={{ fontFamily: 'Syne, sans-serif', color: isListening ? '#2563EB' : 'rgba(107,114,128,0.9)' }}>
                    {isListening ? 'מקשיב...' : result ? 'פקודה בוצעה' : 'פקודה קולית'}
                  </span>
                </div>
                <motion.button
                  whileHover={{ scale: 1.1 }}
                  whileTap={{ scale: 0.9 }}
                  onClick={() => { stopListening(); setShowPanel(false); }}
                  className="w-6 h-6 rounded flex items-center justify-center cursor-pointer"
                  style={{ background: 'rgba(229,231,235,0.4)' }}
                >
                  <X className="w-3.5 h-3.5 text-white/30" />
                </motion.button>
              </div>

              {/* Waveform */}
              <div className="flex items-center justify-center py-4 px-4">
                {isListening ? (
                  <WaveformVisualizer active={isListening} analyser={analyser} />
                ) : result ? (
                  <motion.div
                    initial={{ scale: 0.8, opacity: 0 }}
                    animate={{ scale: 1, opacity: 1 }}
                    className="flex items-center gap-3"
                  >
                    <div className="w-10 h-10 rounded-xl flex items-center justify-center"
                      style={{ background: `${result.color}15`, border: `1px solid ${result.color}30` }}>
                      <result.icon className="w-5 h-5" style={{ color: result.color }} />
                    </div>
                    <span className="text-sm font-medium" style={{ color: result.color, direction: dir }}>
                      {result.text}
                    </span>
                  </motion.div>
                ) : (
                  <div className="flex items-center gap-2">
                    <Loader2 className="w-4 h-4 animate-spin" style={{ color: 'rgba(156,163,175,0.6)' }} />
                    <span className="text-xs" style={{ color: 'rgba(156,163,175,0.9)' }}>מעבד...</span>
                  </div>
                )}
              </div>

              {/* Transcript */}
              {transcript && (
                <div className="px-4 pb-3">
                  <div className="rounded-lg px-3 py-2" style={{ background: 'rgba(37,99,235,0.04)', border: '1px solid rgba(37,99,235,0.08)' }}>
                    <p className="text-xs text-center" style={{ color: 'rgba(75,85,99,0.9)', direction: dir }}>
                      "{transcript}"
                    </p>
                  </div>
                </div>
              )}

              {/* Error */}
              {error && (
                <div className="px-4 pb-3">
                  <div className="rounded-lg px-3 py-2" style={{ background: 'rgba(255,51,85,0.06)', border: '1px solid rgba(255,51,85,0.1)' }}>
                    <p className="text-[10px] text-center" style={{ color: '#DC2626' }}>{error}</p>
                  </div>
                </div>
              )}

              {/* Quick commands hint */}
              <div className="px-4 pb-3">
                <div className="flex flex-wrap gap-1.5 justify-center">
                  {['נווט ל...', 'חפש...', 'מצב לילה', 'תנועה', 'חירום'].map((hint) => (
                    <span key={hint} className="text-[8px] px-2 py-0.5 rounded-full"
                      style={{ background: 'rgba(243,244,246,0.5)', color: 'rgba(156,163,175,0.6)', border: '1px solid rgba(229,231,235,0.4)' }}>
                      {hint}
                    </span>
                  ))}
                </div>
              </div>

              {/* Mic button */}
              <div className="flex justify-center pb-4">
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.9 }}
                  onClick={isListening ? stopListening : startListening}
                  className="w-14 h-14 rounded-full flex items-center justify-center cursor-pointer relative"
                  style={{
                    background: isListening ? 'rgba(255,51,85,0.15)' : 'rgba(37,99,235,0.1)',
                    border: `2px solid ${isListening ? 'rgba(255,51,85,0.3)' : 'rgba(37,99,235,0.25)'}`,
                    boxShadow: isListening ? '0 0 30px rgba(255,51,85,0.2)' : '0 0 30px rgba(37,99,235,0.15)' }}
                >
                  {isListening ? (
                    <MicOff className="w-6 h-6" style={{ color: '#DC2626' }} />
                  ) : (
                    <Mic className="w-6 h-6" style={{ color: '#2563EB' }} />
                  )}
                  {isListening && (
                    <motion.div
                      className="absolute inset-[-6px] rounded-full"
                      style={{ border: '2px solid rgba(255,51,85,0.2)' }}
                      animate={{ scale: [1, 1.3, 1], opacity: [0.5, 0, 0.5] }}
                      transition={{ duration: 1.5, repeat: Infinity }}
                    />
                  )}
                </motion.button>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </>
  );
}
