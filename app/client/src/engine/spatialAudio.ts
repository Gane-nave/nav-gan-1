/**
 * G.A.N.E — Spatial Audio Engine
 * ================================
 * 3D directional audio for navigation cues.
 * Uses Web Audio API with HRTF (Head-Related Transfer Function)
 * to create the illusion that navigation instructions come from
 * the physical direction of the turn/destination.
 *
 * "Turn right" sounds like it's coming from the right intersection.
 */

export interface SpatialAudioConfig {
  enabled: boolean;
  volume: number;              // 0-1
  hrtfEnabled: boolean;        // use HRTF panning model
  maxDistance: number;          // meters — beyond this, sound is ambient
  refDistance: number;          // meters — distance for full volume
  rolloffFactor: number;       // how quickly sound fades with distance
}

export interface AudioCue {
  id: string;
  type: 'turn' | 'alert' | 'poi' | 'hazard' | 'arrival' | 'recalculating';
  bearing: number;             // degrees from current heading (0=ahead, 90=right)
  distance: number;            // meters from user
  priority: number;            // 1=highest
  message?: string;            // TTS text
  soundFile?: string;          // custom sound URL
}

interface ActiveSource {
  source: AudioBufferSourceNode | null;
  panner: PannerNode;
  gain: GainNode;
  cue: AudioCue;
}

// ─── Pre-generated notification tones (synthesized) ───
function generateTone(ctx: AudioContext, freq: number, duration: number, type: OscillatorType = 'sine'): AudioBuffer {
  const sampleRate = ctx.sampleRate;
  const length = Math.floor(sampleRate * duration);
  const buffer = ctx.createBuffer(1, length, sampleRate);
  const data = buffer.getChannelData(0);

  for (let i = 0; i < length; i++) {
    const t = i / sampleRate;
    const envelope = Math.min(1, t * 20) * Math.max(0, 1 - (t / duration) * 1.5);
    let sample = 0;
    switch (type) {
      case 'sine':
        sample = Math.sin(2 * Math.PI * freq * t);
        break;
      case 'triangle':
        sample = 2 * Math.abs(2 * (t * freq - Math.floor(t * freq + 0.5))) - 1;
        break;
      case 'square':
        sample = Math.sign(Math.sin(2 * Math.PI * freq * t));
        break;
      default:
        sample = Math.sin(2 * Math.PI * freq * t);
    }
    data[i] = sample * envelope * 0.3;
  }
  return buffer;
}

function generateChime(ctx: AudioContext): AudioBuffer {
  const sampleRate = ctx.sampleRate;
  const duration = 0.6;
  const length = Math.floor(sampleRate * duration);
  const buffer = ctx.createBuffer(1, length, sampleRate);
  const data = buffer.getChannelData(0);

  const freqs = [880, 1108.73, 1318.51]; // A5, C#6, E6 (A major chord)
  for (let i = 0; i < length; i++) {
    const t = i / sampleRate;
    const envelope = Math.exp(-t * 5) * Math.min(1, t * 50);
    let sample = 0;
    for (const f of freqs) {
      sample += Math.sin(2 * Math.PI * f * t) / freqs.length;
    }
    data[i] = sample * envelope * 0.25;
  }
  return buffer;
}

function generateAlert(ctx: AudioContext): AudioBuffer {
  const sampleRate = ctx.sampleRate;
  const duration = 0.4;
  const length = Math.floor(sampleRate * duration);
  const buffer = ctx.createBuffer(1, length, sampleRate);
  const data = buffer.getChannelData(0);

  for (let i = 0; i < length; i++) {
    const t = i / sampleRate;
    const freq = 600 + 400 * Math.sin(2 * Math.PI * 8 * t); // warbling
    const envelope = Math.min(1, t * 30) * Math.max(0, 1 - t / duration);
    data[i] = Math.sin(2 * Math.PI * freq * t) * envelope * 0.35;
  }
  return buffer;
}

// ─── Spatial Audio Engine ───
const DEFAULT_CONFIG: SpatialAudioConfig = {
  enabled: true,
  volume: 0.7,
  hrtfEnabled: true,
  maxDistance: 500,
  refDistance: 10,
  rolloffFactor: 1.5,
};

export class SpatialAudioEngine {
  private ctx: AudioContext | null = null;
  private config: SpatialAudioConfig;
  private masterGain: GainNode | null = null;
  private activeSources: Map<string, ActiveSource> = new Map();
  private soundBuffers: Map<string, AudioBuffer> = new Map();
  private userHeading: number = 0;
  private initialized = false;
  private ttsQueue: string[] = [];
  private isSpeaking = false;

  constructor(config?: Partial<SpatialAudioConfig>) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /** Initialize audio context (must be called from user gesture) */
  async init(): Promise<void> {
    if (this.initialized) return;

    try {
      this.ctx = new AudioContext();
      this.masterGain = this.ctx.createGain();
      this.masterGain.gain.value = this.config.volume;
      this.masterGain.connect(this.ctx.destination);

      // Pre-generate sound buffers
      this.soundBuffers.set('turn', generateTone(this.ctx, 660, 0.3, 'sine'));
      this.soundBuffers.set('alert', generateAlert(this.ctx));
      this.soundBuffers.set('poi', generateChime(this.ctx));
      this.soundBuffers.set('hazard', generateAlert(this.ctx));
      this.soundBuffers.set('arrival', generateChime(this.ctx));
      this.soundBuffers.set('recalculating', generateTone(this.ctx, 440, 0.5, 'triangle'));

      this.initialized = true;
    } catch (e) {
      console.warn('[SpatialAudio] Failed to initialize:', e);
    }
  }

  /** Update user's heading for spatial positioning */
  updateHeading(headingDegrees: number): void {
    this.userHeading = headingDegrees;

    // Update listener orientation
    if (this.ctx?.listener) {
      const headingRad = headingDegrees * Math.PI / 180;
      const listener = this.ctx.listener;
      if (listener.forwardX) {
        listener.forwardX.value = Math.sin(headingRad);
        listener.forwardY.value = 0;
        listener.forwardZ.value = -Math.cos(headingRad);
        listener.upX.value = 0;
        listener.upY.value = 1;
        listener.upZ.value = 0;
      }
    }
  }

  /** Play a spatial audio cue at a specific bearing and distance */
  playCue(cue: AudioCue): void {
    if (!this.initialized || !this.ctx || !this.masterGain) return;
    if (!this.config.enabled) return;

    // Stop existing cue with same ID
    this.stopCue(cue.id);

    // Create panner for 3D positioning
    const panner = this.ctx.createPanner();
    panner.panningModel = this.config.hrtfEnabled ? 'HRTF' : 'equalpower';
    panner.distanceModel = 'inverse';
    panner.refDistance = this.config.refDistance;
    panner.maxDistance = this.config.maxDistance;
    panner.rolloffFactor = this.config.rolloffFactor;
    panner.coneInnerAngle = 360;
    panner.coneOuterAngle = 360;

    // Convert bearing + distance to 3D position
    const bearingRad = (cue.bearing - this.userHeading) * Math.PI / 180;
    const dist = Math.min(cue.distance, this.config.maxDistance);
    panner.positionX.value = Math.sin(bearingRad) * dist;
    panner.positionY.value = 0;
    panner.positionZ.value = -Math.cos(bearingRad) * dist;

    // Create gain node for this cue
    const gain = this.ctx.createGain();
    gain.gain.value = 1.0 / cue.priority; // higher priority = louder

    // Get or create sound buffer
    const buffer = this.soundBuffers.get(cue.type);
    let source: AudioBufferSourceNode | null = null;

    if (buffer) {
      source = this.ctx.createBufferSource();
      source.buffer = buffer;
      source.connect(gain);
      gain.connect(panner);
      panner.connect(this.masterGain);
      source.start();

      source.onended = () => {
        this.activeSources.delete(cue.id);
      };
    }

    this.activeSources.set(cue.id, { source, panner, gain, cue });

    // If there's a TTS message, queue it
    if (cue.message) {
      this.speakDirectional(cue.message, cue.bearing);
    }
  }

  /** Speak a message with directional panning */
  private speakDirectional(text: string, bearing: number): void {
    if (!('speechSynthesis' in window)) return;

    this.ttsQueue.push(text);
    if (!this.isSpeaking) this.processNextTTS();
  }

  private processNextTTS(): void {
    if (this.ttsQueue.length === 0) {
      this.isSpeaking = false;
      return;
    }

    this.isSpeaking = true;
    const text = this.ttsQueue.shift()!;
    const utterance = new SpeechSynthesisUtterance(text);
    utterance.lang = 'he-IL'; // Hebrew by default
    utterance.rate = 1.1;
    utterance.pitch = 1.0;
    utterance.volume = this.config.volume;

    utterance.onend = () => {
      this.processNextTTS();
    };

    utterance.onerror = () => {
      this.processNextTTS();
    };

    speechSynthesis.speak(utterance);
  }

  /** Stop a specific cue */
  stopCue(id: string): void {
    const active = this.activeSources.get(id);
    if (active) {
      try {
        active.source?.stop();
      } catch { /* already stopped */ }
      active.panner.disconnect();
      active.gain.disconnect();
      this.activeSources.delete(id);
    }
  }

  /** Stop all cues */
  stopAll(): void {
    this.activeSources.forEach((_val, id) => {
      this.stopCue(id);
    });
    speechSynthesis.cancel();
    this.ttsQueue = [];
    this.isSpeaking = false;
  }

  /** Update volume */
  setVolume(volume: number): void {
    this.config.volume = Math.max(0, Math.min(1, volume));
    if (this.masterGain) {
      this.masterGain.gain.value = this.config.volume;
    }
  }

  /** Enable/disable spatial audio */
  setEnabled(enabled: boolean): void {
    this.config.enabled = enabled;
    if (!enabled) this.stopAll();
  }

  /** Get current state */
  getState(): { enabled: boolean; volume: number; activeCues: number; initialized: boolean } {
    return {
      enabled: this.config.enabled,
      volume: this.config.volume,
      activeCues: this.activeSources.size,
      initialized: this.initialized,
    };
  }

  /** Cleanup */
  destroy(): void {
    this.stopAll();
    this.ctx?.close();
    this.ctx = null;
    this.initialized = false;
  }
}
