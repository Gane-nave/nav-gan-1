/**
 * G.A.N.E — Professional Voice & Audio System
 * Uses Web Speech API with advanced controls: speed, pitch, voice selection, queue management
 * Supports all browser-available languages with automatic voice matching
 */
import { createContext, useContext, useState, useCallback, useRef, useEffect, type ReactNode } from 'react';

interface VoiceState {
  isPlaying: boolean;
  isPaused: boolean;
  currentText: string;
  progress: number;
  speed: number;
  pitch: number;
  volume: number;
  selectedVoiceLang: string;
  availableVoices: SpeechSynthesisVoice[];
}

interface VoiceContextType extends VoiceState {
  speak: (text: string, lang?: string) => void;
  pause: () => void;
  resume: () => void;
  stop: () => void;
  setSpeed: (speed: number) => void;
  setPitch: (pitch: number) => void;
  setVolume: (volume: number) => void;
  setVoiceLang: (lang: string) => void;
  isSupported: boolean;
}

const VoiceContext = createContext<VoiceContextType | null>(null);

export function VoiceProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<VoiceState>({
    isPlaying: false,
    isPaused: false,
    currentText: '',
    progress: 0,
    speed: 1,
    pitch: 1,
    volume: 1,
    selectedVoiceLang: 'en',
    availableVoices: [] });

  const utteranceRef = useRef<SpeechSynthesisUtterance | null>(null);
  const chunksRef = useRef<string[]>([]);
  const currentChunkRef = useRef(0);
  const isSupported = typeof window !== 'undefined' && 'speechSynthesis' in window;

  // Load available voices
  useEffect(() => {
    if (!isSupported) return;
    const loadVoices = () => {
      const voices = window.speechSynthesis.getVoices();
      if (voices.length > 0) {
        setState(prev => ({ ...prev, availableVoices: voices }));
      }
    };
    loadVoices();
    window.speechSynthesis.onvoiceschanged = loadVoices;
    return () => { window.speechSynthesis.onvoiceschanged = null; };
  }, [isSupported]);

  // Find best voice for language
  const findVoice = useCallback((lang: string): SpeechSynthesisVoice | null => {
    const voices = state.availableVoices;
    // Try exact match first
    let voice = voices.find(v => v.lang.startsWith(lang) && v.localService === false);
    if (!voice) voice = voices.find(v => v.lang.startsWith(lang));
    if (!voice) voice = voices.find(v => v.lang.startsWith('en'));
    return voice || voices[0] || null;
  }, [state.availableVoices]);

  // Split text into manageable chunks for better TTS performance
  const splitText = useCallback((text: string): string[] => {
    const maxLen = 200;
    const sentences = text.split(/(?<=[.!?。！？])\s+/);
    const chunks: string[] = [];
    let current = '';
    for (const sentence of sentences) {
      if ((current + ' ' + sentence).length > maxLen && current) {
        chunks.push(current.trim());
        current = sentence;
      } else {
        current = current ? current + ' ' + sentence : sentence;
      }
    }
    if (current.trim()) chunks.push(current.trim());
    return chunks.length > 0 ? chunks : [text];
  }, []);

  const speakChunk = useCallback((index: number) => {
    if (index >= chunksRef.current.length) {
      setState(prev => ({ ...prev, isPlaying: false, isPaused: false, progress: 100 }));
      return;
    }
    const chunk = chunksRef.current[index];
    const utterance = new SpeechSynthesisUtterance(chunk);
    const voice = findVoice(state.selectedVoiceLang);
    if (voice) utterance.voice = voice;
    utterance.rate = state.speed;
    utterance.pitch = state.pitch;
    utterance.volume = state.volume;
    utterance.lang = state.selectedVoiceLang;

    utterance.onend = () => {
      currentChunkRef.current = index + 1;
      const progress = Math.round(((index + 1) / chunksRef.current.length) * 100);
      setState(prev => ({ ...prev, progress }));
      speakChunk(index + 1);
    };

    utterance.onerror = () => {
      setState(prev => ({ ...prev, isPlaying: false, isPaused: false }));
    };

    utteranceRef.current = utterance;
    window.speechSynthesis.speak(utterance);
  }, [findVoice, state.selectedVoiceLang, state.speed, state.pitch, state.volume]);

  const speak = useCallback((text: string, lang?: string) => {
    if (!isSupported) return;
    window.speechSynthesis.cancel();
    const cleanText = text.replace(/<[^>]*>/g, '').replace(/[#*`_~]/g, '').replace(/\s+/g, ' ').trim();
    if (!cleanText) return;

    if (lang) {
      setState(prev => ({ ...prev, selectedVoiceLang: lang }));
    }

    chunksRef.current = splitText(cleanText);
    currentChunkRef.current = 0;
    setState(prev => ({
      ...prev,
      isPlaying: true,
      isPaused: false,
      currentText: cleanText.slice(0, 100) + (cleanText.length > 100 ? '...' : ''),
      progress: 0,
      selectedVoiceLang: lang || prev.selectedVoiceLang }));
    speakChunk(0);
  }, [isSupported, splitText, speakChunk]);

  const pause = useCallback(() => {
    if (!isSupported) return;
    window.speechSynthesis.pause();
    setState(prev => ({ ...prev, isPaused: true }));
  }, [isSupported]);

  const resume = useCallback(() => {
    if (!isSupported) return;
    window.speechSynthesis.resume();
    setState(prev => ({ ...prev, isPaused: false }));
  }, [isSupported]);

  const stop = useCallback(() => {
    if (!isSupported) return;
    window.speechSynthesis.cancel();
    chunksRef.current = [];
    currentChunkRef.current = 0;
    setState(prev => ({ ...prev, isPlaying: false, isPaused: false, progress: 0, currentText: '' }));
  }, [isSupported]);

  const setSpeed = useCallback((speed: number) => {
    setState(prev => ({ ...prev, speed: Math.max(0.25, Math.min(4, speed)) }));
  }, []);

  const setPitch = useCallback((pitch: number) => {
    setState(prev => ({ ...prev, pitch: Math.max(0, Math.min(2, pitch)) }));
  }, []);

  const setVolume = useCallback((volume: number) => {
    setState(prev => ({ ...prev, volume: Math.max(0, Math.min(1, volume)) }));
  }, []);

  const setVoiceLang = useCallback((lang: string) => {
    setState(prev => ({ ...prev, selectedVoiceLang: lang }));
  }, []);

  return (
    <VoiceContext.Provider value={{
      ...state, speak, pause, resume, stop,
      setSpeed, setPitch, setVolume, setVoiceLang, isSupported }}>
      {children}
    </VoiceContext.Provider>
  );
}

export function useVoice() {
  const ctx = useContext(VoiceContext);
  if (!ctx) throw new Error('useVoice must be used within VoiceProvider');
  return ctx;
}
