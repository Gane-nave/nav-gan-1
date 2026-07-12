/**
 * G.A.N.E NAV — VHF/UHF/HF Radio Communications Engine
 * =====================================================
 * Full-spectrum radio communications for emergency and tactical use.
 * When all internet/cellular connectivity fails, radio is the last line.
 *
 * Supported Bands:
 * - HF  (3–30 MHz)   — Long-range skywave propagation
 * - VHF (30–300 MHz)  — Line-of-sight, aviation, marine
 * - UHF (300–3000 MHz)— Urban, military, satellite uplink
 *
 * Protocols:
 * - APRS (Automatic Packet Reporting System) — Position beaconing
 * - DMR  (Digital Mobile Radio) — Digital voice + data
 * - D-STAR — Digital voice with GPS data embedding
 * - System Fusion (C4FM) — Wideband digital voice
 * - AIS  — Automatic Identification System (marine)
 * - ADS-B — Aircraft position broadcast
 * - Mesh Networking — Multi-hop relay for extended range
 *
 * Emergency Frequencies:
 * - 121.500 MHz — International aviation distress
 * - 156.800 MHz — Marine channel 16 (distress)
 * - 243.000 MHz — Military aviation distress (UHF)
 * - 406.025 MHz — COSPAS-SARSAT satellite distress
 * - 462.5625 MHz — FRS/GMRS emergency
 */

// ─── Radio Band Definitions ───
export type RadioBand = 'HF' | 'VHF' | 'UHF';
export type ModulationType = 'AM' | 'FM' | 'SSB' | 'DMR' | 'DSTAR' | 'C4FM' | 'APRS' | 'AIS' | 'ADSB';
export type RadioState = 'idle' | 'scanning' | 'transmitting' | 'receiving' | 'emergency' | 'mesh-relay';

export interface RadioFrequency {
  id: string;
  frequency: number;       // MHz
  band: RadioBand;
  modulation: ModulationType;
  label: string;
  description: string;
  isEmergency: boolean;
  power: number;           // Watts
  range: number;           // km estimated
  active: boolean;
}

export interface RadioChannel {
  id: string;
  name: string;
  frequency: RadioFrequency;
  squelch: number;         // 0-9
  ctcss?: number;          // CTCSS tone Hz
  dcs?: number;            // DCS code
  encrypted: boolean;
  members: string[];
}

export interface APRSPacket {
  callsign: string;
  ssid: number;
  lat: number;
  lon: number;
  altitude: number;
  speed: number;
  course: number;
  symbol: string;
  comment: string;
  timestamp: number;
  path: string[];
  digipeaters: string[];
}

export interface MeshNode {
  id: string;
  callsign: string;
  lat: number;
  lon: number;
  lastSeen: number;
  signalStrength: number;  // dBm
  hops: number;
  battery: number;         // %
  role: 'router' | 'client' | 'relay' | 'gateway';
  neighbors: string[];
}

export interface RadioTransmission {
  id: string;
  channelId: string;
  from: string;
  to: string | 'broadcast';
  type: 'voice' | 'data' | 'position' | 'emergency' | 'text';
  timestamp: number;
  duration: number;        // ms
  signalStrength: number;  // dBm
  snr: number;             // Signal-to-noise ratio dB
  data?: string;
  position?: { lat: number; lon: number };
}

export interface RadioStatus {
  state: RadioState;
  activeBand: RadioBand;
  activeFrequency: number;
  activeChannel: RadioChannel | null;
  signalStrength: number;
  noiseFloor: number;
  squelchOpen: boolean;
  ptt: boolean;
  meshConnected: boolean;
  meshNodeCount: number;
  aprsBeaconing: boolean;
  lastTransmission: RadioTransmission | null;
  emergencyActive: boolean;
  txPower: number;
  batteryDrain: number;    // mW estimated
}

// ─── Emergency Frequencies Database ───
export const EMERGENCY_FREQUENCIES: RadioFrequency[] = [
  {
    id: 'em-aviation-vhf',
    frequency: 121.500,
    band: 'VHF',
    modulation: 'AM',
    label: 'Aviation Distress',
    description: 'International aviation emergency — monitored by all aircraft and ATC towers worldwide',
    isEmergency: true,
    power: 5,
    range: 200,
    active: false,
  },
  {
    id: 'em-marine-ch16',
    frequency: 156.800,
    band: 'VHF',
    modulation: 'FM',
    label: 'Marine Ch.16',
    description: 'International maritime distress and calling — monitored by coast guard and all vessels',
    isEmergency: true,
    power: 25,
    range: 60,
    active: false,
  },
  {
    id: 'em-military-uhf',
    frequency: 243.000,
    band: 'UHF',
    modulation: 'AM',
    label: 'Military Distress',
    description: 'NATO/military aviation emergency frequency — UHF guard',
    isEmergency: true,
    power: 5,
    range: 150,
    active: false,
  },
  {
    id: 'em-sarsat',
    frequency: 406.025,
    band: 'UHF',
    modulation: 'FM',
    label: 'COSPAS-SARSAT',
    description: 'Satellite-based search and rescue beacon — global coverage via LEO/MEO satellites',
    isEmergency: true,
    power: 5,
    range: 0, // satellite — global
    active: false,
  },
  {
    id: 'em-frs',
    frequency: 462.5625,
    band: 'UHF',
    modulation: 'FM',
    label: 'FRS/GMRS Emergency',
    description: 'Family Radio Service emergency channel — no license required',
    isEmergency: true,
    power: 2,
    range: 5,
    active: false,
  },
  {
    id: 'em-pmr446',
    frequency: 446.00625,
    band: 'UHF',
    modulation: 'FM',
    label: 'PMR446 Ch.1',
    description: 'European license-free PMR — common emergency/calling channel',
    isEmergency: true,
    power: 0.5,
    range: 3,
    active: false,
  },
];

// ─── Standard Frequency Presets ───
export const STANDARD_FREQUENCIES: RadioFrequency[] = [
  // VHF Marine
  { id: 'marine-ch9', frequency: 156.450, band: 'VHF', modulation: 'FM', label: 'Marine Ch.9', description: 'Boater calling channel', isEmergency: false, power: 25, range: 40, active: false },
  { id: 'marine-ch13', frequency: 156.650, band: 'VHF', modulation: 'FM', label: 'Marine Ch.13', description: 'Bridge-to-bridge navigation safety', isEmergency: false, power: 1, range: 15, active: false },
  { id: 'marine-ch70', frequency: 156.525, band: 'VHF', modulation: 'FM', label: 'Marine DSC', description: 'Digital Selective Calling — automated distress', isEmergency: false, power: 25, range: 40, active: false },
  // Aviation
  { id: 'atis', frequency: 127.800, band: 'VHF', modulation: 'AM', label: 'ATIS', description: 'Automatic Terminal Information Service', isEmergency: false, power: 5, range: 100, active: false },
  { id: 'unicom', frequency: 122.800, band: 'VHF', modulation: 'AM', label: 'UNICOM', description: 'Uncontrolled airport advisory', isEmergency: false, power: 5, range: 80, active: false },
  // APRS
  { id: 'aprs-na', frequency: 144.390, band: 'VHF', modulation: 'APRS', label: 'APRS NA', description: 'APRS North America — position beaconing', isEmergency: false, power: 5, range: 50, active: false },
  { id: 'aprs-eu', frequency: 144.800, band: 'VHF', modulation: 'APRS', label: 'APRS EU', description: 'APRS Europe — position beaconing', isEmergency: false, power: 5, range: 50, active: false },
  // DMR
  { id: 'dmr-call', frequency: 438.500, band: 'UHF', modulation: 'DMR', label: 'DMR Calling', description: 'Digital Mobile Radio calling channel', isEmergency: false, power: 5, range: 20, active: false },
  // D-STAR
  { id: 'dstar-call', frequency: 145.670, band: 'VHF', modulation: 'DSTAR', label: 'D-STAR Call', description: 'D-STAR digital voice calling', isEmergency: false, power: 5, range: 30, active: false },
  // AIS
  { id: 'ais-1', frequency: 161.975, band: 'VHF', modulation: 'AIS', label: 'AIS Ch.1', description: 'Automatic Identification System — vessel tracking', isEmergency: false, power: 12.5, range: 40, active: false },
  { id: 'ais-2', frequency: 162.025, band: 'VHF', modulation: 'AIS', label: 'AIS Ch.2', description: 'AIS secondary channel', isEmergency: false, power: 12.5, range: 40, active: false },
  // HF
  { id: 'hf-20m', frequency: 14.300, band: 'HF', modulation: 'SSB', label: 'HF 20m Net', description: 'International emergency/traffic net — skywave propagation', isEmergency: false, power: 100, range: 5000, active: false },
  { id: 'hf-40m', frequency: 7.290, band: 'HF', modulation: 'SSB', label: 'HF 40m Net', description: 'Regional emergency net — NVIS propagation', isEmergency: false, power: 100, range: 800, active: false },
];

// ─── Radio Communications Engine ───
export class RadioCommsEngine {
  private state: RadioState = 'idle';
  private channels: Map<string, RadioChannel> = new Map();
  private meshNodes: Map<string, MeshNode> = new Map();
  private transmissionLog: RadioTransmission[] = [];
  private aprsBeaconInterval: ReturnType<typeof setInterval> | null = null;
  private meshDiscoveryInterval: ReturnType<typeof setInterval> | null = null;
  private listeners: Map<string, Set<(data: any) => void>> = new Map();
  private myCallsign: string = 'GANE-01';
  private myPosition: { lat: number; lon: number; alt: number } = { lat: 32.0853, lon: 34.7818, alt: 0 };
  private txPower: number = 5; // Watts
  private activeBand: RadioBand = 'VHF';
  private activeFrequency: number = 145.500;
  private squelchLevel: number = 3;
  private emergencyActive: boolean = false;
  private pttActive: boolean = false;

  constructor(callsign?: string) {
    if (callsign) this.myCallsign = callsign;
    this.initializeDefaultChannels();
  }

  private initializeDefaultChannels(): void {
    // Emergency channels
    for (const freq of EMERGENCY_FREQUENCIES) {
      this.channels.set(freq.id, {
        id: freq.id,
        name: freq.label,
        frequency: freq,
        squelch: 0, // Always open for emergency
        encrypted: false,
        members: [],
      });
    }

    // Standard channels
    for (const freq of STANDARD_FREQUENCIES) {
      this.channels.set(freq.id, {
        id: freq.id,
        name: freq.label,
        frequency: freq,
        squelch: this.squelchLevel,
        encrypted: false,
        members: [],
      });
    }
  }

  // ─── APRS Position Beaconing ───
  startAPRSBeacon(intervalMs: number = 120000): void {
    if (this.aprsBeaconInterval) return;
    
    const beacon = (): APRSPacket => ({
      callsign: this.myCallsign,
      ssid: 9, // Mobile station
      lat: this.myPosition.lat,
      lon: this.myPosition.lon,
      altitude: this.myPosition.alt,
      speed: 0,
      course: 0,
      symbol: '/>', // Car symbol
      comment: 'G.A.N.E NAV Mobile',
      timestamp: Date.now(),
      path: ['WIDE1-1', 'WIDE2-1'],
      digipeaters: [],
    });

    // Send initial beacon
    this.emit('aprs-beacon', beacon());
    
    this.aprsBeaconInterval = setInterval(() => {
      this.emit('aprs-beacon', beacon());
    }, intervalMs);
  }

  stopAPRSBeacon(): void {
    if (this.aprsBeaconInterval) {
      clearInterval(this.aprsBeaconInterval);
      this.aprsBeaconInterval = null;
    }
  }

  // ─── Mesh Networking ───
  startMeshDiscovery(): void {
    if (this.meshDiscoveryInterval) return;

    this.meshDiscoveryInterval = setInterval(() => {
      // Simulate mesh node discovery
      this.emit('mesh-discovery', {
        nodeCount: this.meshNodes.size,
        nodes: Array.from(this.meshNodes.values()),
      });
    }, 30000);
  }

  stopMeshDiscovery(): void {
    if (this.meshDiscoveryInterval) {
      clearInterval(this.meshDiscoveryInterval);
      this.meshDiscoveryInterval = null;
    }
  }

  addMeshNode(node: MeshNode): void {
    this.meshNodes.set(node.id, node);
    this.emit('mesh-node-added', node);
  }

  removeMeshNode(nodeId: string): void {
    this.meshNodes.delete(nodeId);
    this.emit('mesh-node-removed', nodeId);
  }

  getMeshTopology(): { nodes: MeshNode[]; links: { from: string; to: string; strength: number }[] } {
    const nodes = Array.from(this.meshNodes.values());
    const links: { from: string; to: string; strength: number }[] = [];
    
    for (const node of nodes) {
      for (const neighborId of node.neighbors) {
        const neighbor = this.meshNodes.get(neighborId);
        if (neighbor) {
          links.push({
            from: node.id,
            to: neighborId,
            strength: Math.min(node.signalStrength, neighbor.signalStrength),
          });
        }
      }
    }

    return { nodes, links };
  }

  // ─── Emergency Operations ───
  activateEmergency(frequency?: number): void {
    this.emergencyActive = true;
    this.state = 'emergency';
    
    // Default to 121.5 MHz aviation distress
    const emergencyFreq = frequency || 121.500;
    this.activeFrequency = emergencyFreq;
    
    // Enable all emergency channels
    for (const freq of EMERGENCY_FREQUENCIES) {
      const channel = this.channels.get(freq.id);
      if (channel) {
        channel.frequency.active = true;
      }
    }

    // Start APRS emergency beacon (every 30 seconds)
    this.startAPRSBeacon(30000);

    this.emit('emergency-activated', {
      frequency: emergencyFreq,
      position: this.myPosition,
      timestamp: Date.now(),
    });
  }

  deactivateEmergency(): void {
    this.emergencyActive = false;
    this.state = 'idle';
    
    for (const freq of EMERGENCY_FREQUENCIES) {
      const channel = this.channels.get(freq.id);
      if (channel) {
        channel.frequency.active = false;
      }
    }

    this.stopAPRSBeacon();
    this.emit('emergency-deactivated', { timestamp: Date.now() });
  }

  // ─── PTT (Push-to-Talk) ───
  startTransmit(): void {
    if (this.state === 'transmitting') return;
    this.pttActive = true;
    this.state = 'transmitting';
    this.emit('ptt-start', { frequency: this.activeFrequency, band: this.activeBand });
  }

  stopTransmit(): void {
    this.pttActive = false;
    this.state = 'idle';
    this.emit('ptt-stop', { frequency: this.activeFrequency });
  }

  // ─── Frequency Management ───
  tune(frequency: number, band?: RadioBand): void {
    this.activeFrequency = frequency;
    if (band) this.activeBand = band;
    else if (frequency < 30) this.activeBand = 'HF';
    else if (frequency < 300) this.activeBand = 'VHF';
    else this.activeBand = 'UHF';
    
    this.emit('frequency-changed', { frequency, band: this.activeBand });
  }

  scan(bandFilter?: RadioBand): void {
    this.state = 'scanning';
    this.emit('scan-start', { band: bandFilter || 'all' });
  }

  stopScan(): void {
    this.state = 'idle';
    this.emit('scan-stop', {});
  }

  // ─── Configuration ───
  setCallsign(callsign: string): void {
    this.myCallsign = callsign;
  }

  setPosition(lat: number, lon: number, alt: number = 0): void {
    this.myPosition = { lat, lon, alt };
  }

  setTxPower(watts: number): void {
    this.txPower = Math.max(0.5, Math.min(100, watts));
  }

  setSquelch(level: number): void {
    this.squelchLevel = Math.max(0, Math.min(9, level));
  }

  // ─── Status ───
  getStatus(): RadioStatus {
    return {
      state: this.state,
      activeBand: this.activeBand,
      activeFrequency: this.activeFrequency,
      activeChannel: this.getActiveChannel(),
      signalStrength: -65 + Math.random() * 30, // Simulated
      noiseFloor: -120 + Math.random() * 10,
      squelchOpen: this.state === 'receiving',
      ptt: this.pttActive,
      meshConnected: this.meshNodes.size > 0,
      meshNodeCount: this.meshNodes.size,
      aprsBeaconing: this.aprsBeaconInterval !== null,
      lastTransmission: this.transmissionLog[this.transmissionLog.length - 1] || null,
      emergencyActive: this.emergencyActive,
      txPower: this.txPower,
      batteryDrain: this.calculateBatteryDrain(),
    };
  }

  private getActiveChannel(): RadioChannel | null {
    for (const channel of Array.from(this.channels.values())) {
      if (Math.abs(channel.frequency.frequency - this.activeFrequency) < 0.001) {
        return channel;
      }
    }
    return null;
  }

  private calculateBatteryDrain(): number {
    let drain = 50; // Base receiver drain mW
    if (this.pttActive) drain += this.txPower * 1000; // TX power
    if (this.aprsBeaconInterval) drain += 100;
    if (this.meshDiscoveryInterval) drain += 200;
    if (this.state === 'scanning') drain += 150;
    return drain;
  }

  getAllChannels(): RadioChannel[] {
    return Array.from(this.channels.values());
  }

  getEmergencyChannels(): RadioChannel[] {
    return Array.from(this.channels.values()).filter(ch => ch.frequency.isEmergency);
  }

  getTransmissionLog(): RadioTransmission[] {
    return [...this.transmissionLog];
  }

  // ─── Event System ───
  on(event: string, callback: (data: any) => void): () => void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(callback);
    return () => this.listeners.get(event)?.delete(callback);
  }

  private emit(event: string, data: any): void {
    const callbacks = this.listeners.get(event);
    if (callbacks) {
      for (const cb of Array.from(callbacks)) {
        try { cb(data); } catch (e) { console.error(`[Radio] Event handler error:`, e); }
      }
    }
  }

  // ─── Cleanup ───
  destroy(): void {
    this.stopAPRSBeacon();
    this.stopMeshDiscovery();
    this.listeners.clear();
    this.channels.clear();
    this.meshNodes.clear();
  }
}

// ─── Singleton ───
let instance: RadioCommsEngine | null = null;
export function getRadioEngine(callsign?: string): RadioCommsEngine {
  if (!instance) instance = new RadioCommsEngine(callsign);
  return instance;
}

export default RadioCommsEngine;
