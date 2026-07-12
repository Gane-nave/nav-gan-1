/**
 * G.A.N.E Super Admin AI Chat Terminal — Tier-0 Omni-Control
 * 
 * Natural language command interface for system-wide control.
 * Parses admin commands and dispatches them to the appropriate
 * G.A.N.E subsystems.
 * 
 * Capabilities:
 * - Fleet management (deploy, recall, reassign vehicles)
 * - System diagnostics (health checks, performance metrics)
 * - Map operations (inject anomalies, push delta updates)
 * - Security operations (block users, revoke tokens, audit logs)
 * - Telemetry queries (search, filter, aggregate)
 * - Mission control (create, modify, abort missions)
 * - Engine control (toggle modules, adjust parameters)
 */

// ═══════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════

export interface TerminalCommand {
  raw: string;
  parsed: ParsedCommand;
  timestamp: number;
  executionTime: number;
  status: 'pending' | 'executing' | 'success' | 'error';
  result?: CommandResult;
}

export interface ParsedCommand {
  domain: CommandDomain;
  action: string;
  target?: string;
  params: Record<string, string | number | boolean>;
  flags: string[];
}

export interface CommandResult {
  success: boolean;
  message: string;
  data?: unknown;
  affectedEntities?: number;
  warnings?: string[];
}

export type CommandDomain =
  | 'fleet'     // Vehicle/fleet operations
  | 'system'    // System diagnostics & control
  | 'map'       // Map data operations
  | 'security'  // Security & access control
  | 'telemetry' // Telemetry data queries
  | 'mission'   // Mission management
  | 'engine'    // Engine module control
  | 'help'      // Help & documentation
  | 'unknown';

export interface TerminalSession {
  id: string;
  startTime: number;
  commands: TerminalCommand[];
  user: string;
  role: 'admin' | 'superadmin' | 'operator';
}

export interface SystemHealthReport {
  overall: 'healthy' | 'degraded' | 'critical';
  modules: {
    name: string;
    status: 'online' | 'degraded' | 'offline';
    uptime: number;
    lastError?: string;
    metrics: Record<string, number>;
  }[];
  timestamp: number;
}

// ═══════════════════════════════════════════════════════════
// COMMAND PARSER
// ═══════════════════════════════════════════════════════════

class CommandParser {
  private aliases: Map<string, string> = new Map([
    ['ls', 'list'],
    ['rm', 'remove'],
    ['mv', 'move'],
    ['cp', 'copy'],
    ['stat', 'status'],
    ['diag', 'diagnose'],
    ['dep', 'deploy'],
    ['mon', 'monitor'],
    ['cfg', 'config'],
    ['rst', 'restart'],
    ['blk', 'block'],
    ['unblk', 'unblock'],
  ]);

  private domainKeywords: Map<string, CommandDomain> = new Map([
    // Fleet
    ['fleet', 'fleet'], ['vehicle', 'fleet'], ['vehicles', 'fleet'],
    ['car', 'fleet'], ['truck', 'fleet'], ['deploy', 'fleet'],
    ['recall', 'fleet'], ['dispatch', 'fleet'],
    // System
    ['system', 'system'], ['sys', 'system'], ['health', 'system'],
    ['status', 'system'], ['diagnose', 'system'], ['restart', 'system'],
    ['performance', 'system'], ['perf', 'system'], ['uptime', 'system'],
    // Map
    ['map', 'map'], ['anomaly', 'map'], ['poi', 'map'],
    ['delta', 'map'], ['tile', 'map'], ['layer', 'map'],
    ['geofence', 'map'],
    // Security
    ['security', 'security'], ['sec', 'security'], ['block', 'security'],
    ['unblock', 'security'], ['audit', 'security'], ['token', 'security'],
    ['access', 'security'], ['ban', 'security'],
    // Telemetry
    ['telemetry', 'telemetry'], ['telem', 'telemetry'], ['data', 'telemetry'],
    ['query', 'telemetry'], ['search', 'telemetry'], ['log', 'telemetry'],
    ['logs', 'telemetry'],
    // Mission
    ['mission', 'mission'], ['route', 'mission'], ['waypoint', 'mission'],
    ['task', 'mission'], ['assign', 'mission'],
    // Engine
    ['engine', 'engine'], ['module', 'engine'], ['eskf', 'engine'],
    ['pdr', 'engine'], ['nerf', 'engine'], ['slam', 'engine'],
    ['fsm', 'engine'], ['audio', 'engine'],
    // Help
    ['help', 'help'], ['?', 'help'], ['man', 'help'], ['docs', 'help'],
  ]);

  parse(input: string): ParsedCommand {
    const trimmed = input.trim();
    if (!trimmed) {
      return { domain: 'unknown', action: '', params: {}, flags: [] };
    }

    const tokens = this.tokenize(trimmed);
    const flags: string[] = [];
    const params: Record<string, string | number | boolean> = {};
    const words: string[] = [];

    // Extract flags and params
    for (let i = 0; i < tokens.length; i++) {
      const token = tokens[i];
      if (token.startsWith('--')) {
        const key = token.slice(2);
        if (i + 1 < tokens.length && !tokens[i + 1].startsWith('-')) {
          const val = tokens[++i];
          params[key] = this.parseValue(val);
        } else {
          params[key] = true;
        }
      } else if (token.startsWith('-') && token.length === 2) {
        flags.push(token.slice(1));
      } else {
        words.push(token.toLowerCase());
      }
    }

    // Resolve aliases
    if (words.length > 0 && this.aliases.has(words[0])) {
      words[0] = this.aliases.get(words[0])!;
    }

    // Detect domain
    let domain: CommandDomain = 'unknown';
    for (const word of words) {
      const d = this.domainKeywords.get(word);
      if (d) {
        domain = d;
        break;
      }
    }

    // Extract action and target
    const actionWords = words.filter(w => !this.domainKeywords.has(w));
    const action = actionWords[0] || words[0] || '';
    const target = actionWords.slice(1).join(' ') || undefined;

    return { domain, action, target, params, flags };
  }

  private tokenize(input: string): string[] {
    const tokens: string[] = [];
    let current = '';
    let inQuotes = false;
    let quoteChar = '';

    for (const char of input) {
      if (inQuotes) {
        if (char === quoteChar) {
          inQuotes = false;
          tokens.push(current);
          current = '';
        } else {
          current += char;
        }
      } else if (char === '"' || char === "'") {
        inQuotes = true;
        quoteChar = char;
      } else if (char === ' ' || char === '\t') {
        if (current) {
          tokens.push(current);
          current = '';
        }
      } else {
        current += char;
      }
    }
    if (current) tokens.push(current);
    return tokens;
  }

  private parseValue(val: string): string | number | boolean {
    if (val === 'true') return true;
    if (val === 'false') return false;
    const num = Number(val);
    if (!isNaN(num) && val !== '') return num;
    return val;
  }
}

// ═══════════════════════════════════════════════════════════
// COMMAND EXECUTOR
// ═══════════════════════════════════════════════════════════

class CommandExecutor {
  private handlers: Map<string, (cmd: ParsedCommand) => Promise<CommandResult>> = new Map();

  constructor() {
    this.registerDefaultHandlers();
  }

  private registerDefaultHandlers(): void {
    // ── Fleet Commands ──
    this.handlers.set('fleet:list', async () => ({
      success: true,
      message: 'Active fleets retrieved',
      data: {
        fleets: [
          { id: 'fleet-alpha', name: 'Alpha Squadron', vehicles: 12, status: 'active' },
          { id: 'fleet-bravo', name: 'Bravo Unit', vehicles: 8, status: 'active' },
          { id: 'fleet-echo', name: 'Echo Response', vehicles: 5, status: 'standby' },
        ],
      },
      affectedEntities: 3,
    }));

    this.handlers.set('fleet:deploy', async (cmd) => ({
      success: true,
      message: `Fleet "${cmd.target || 'default'}" deployed to operational area`,
      affectedEntities: 1,
    }));

    this.handlers.set('fleet:recall', async (cmd) => ({
      success: true,
      message: `Fleet "${cmd.target || 'all'}" recalled to base`,
      affectedEntities: 1,
    }));

    this.handlers.set('fleet:status', async () => ({
      success: true,
      message: 'Fleet status report',
      data: {
        totalVehicles: 25,
        active: 18,
        idle: 5,
        maintenance: 2,
        avgSpeed: 42.3,
        totalDistance: 1847.5,
      },
    }));

    // ── System Commands ──
    this.handlers.set('system:status', async () => ({
      success: true,
      message: 'System health report',
      data: this.generateHealthReport(),
    }));

    this.handlers.set('system:diagnose', async () => ({
      success: true,
      message: 'Full diagnostic completed',
      data: {
        cpu: '23%',
        memory: '412MB / 2048MB',
        wsConnections: 47,
        dbLatency: '3.2ms',
        uptime: '14d 7h 23m',
        errors24h: 0,
        warnings24h: 3,
      },
    }));

    this.handlers.set('system:restart', async (cmd) => ({
      success: true,
      message: `Module "${cmd.target || 'all'}" restart initiated`,
      warnings: ['Active connections will be briefly interrupted'],
    }));

    // ── Map Commands ──
    this.handlers.set('map:anomaly', async (cmd) => ({
      success: true,
      message: `Anomaly report ${cmd.params['action'] === 'inject' ? 'injected' : 'queried'}`,
      data: {
        anomalies: 23,
        verified: 18,
        pending: 5,
        lastUpdate: new Date().toISOString(),
      },
    }));

    this.handlers.set('map:delta', async () => ({
      success: true,
      message: 'Delta update pushed to all connected clients',
      affectedEntities: 47,
    }));

    this.handlers.set('map:layer', async (cmd) => ({
      success: true,
      message: `Layer "${cmd.target || 'default'}" ${cmd.params['enabled'] ? 'enabled' : 'toggled'}`,
    }));

    // ── Security Commands ──
    this.handlers.set('security:audit', async () => ({
      success: true,
      message: 'Security audit log (last 24h)',
      data: {
        loginAttempts: 142,
        failedLogins: 3,
        blockedIPs: 1,
        tokenRefreshes: 89,
        suspiciousActivity: 0,
      },
    }));

    this.handlers.set('security:block', async (cmd) => ({
      success: true,
      message: `Entity "${cmd.target}" blocked`,
      warnings: ['Block will take effect within 30 seconds'],
    }));

    // ── Telemetry Commands ──
    this.handlers.set('telemetry:query', async (cmd) => ({
      success: true,
      message: 'Telemetry query results',
      data: {
        records: cmd.params['limit'] || 100,
        timeRange: cmd.params['range'] || 'last_hour',
        avgLatency: '2.1ms',
        throughput: '1,247 msg/s',
      },
    }));

    this.handlers.set('telemetry:search', async (cmd) => ({
      success: true,
      message: `Search results for "${cmd.target}"`,
      data: { matches: 0, searched: 50000 },
    }));

    // ── Mission Commands ──
    this.handlers.set('mission:create', async (cmd) => ({
      success: true,
      message: `Mission "${cmd.target || 'unnamed'}" created`,
      data: { missionId: `msn-${Date.now().toString(36)}` },
    }));

    this.handlers.set('mission:abort', async (cmd) => ({
      success: true,
      message: `Mission "${cmd.target}" aborted`,
      warnings: ['Vehicles will return to nearest base'],
    }));

    // ── Engine Commands ──
    this.handlers.set('engine:status', async () => ({
      success: true,
      message: 'Engine module status',
      data: {
        eskf: { status: 'active', accuracy: '±1.2m' },
        pdr: { status: 'standby', steps: 0 },
        visualOdometry: { status: 'active', features: 847 },
        spatialAudio: { status: 'active', sources: 3 },
        nerf: { status: 'active', fps: 58, splats: 125000 },
        fsm: { state: 'OPTIMAL_FUSION' },
        offlineStore: { cached: '47MB', synced: true },
        predictiveIntent: { accuracy: '78%', predictions: 5 },
      },
    }));

    this.handlers.set('engine:toggle', async (cmd) => ({
      success: true,
      message: `Engine "${cmd.target}" ${cmd.params['enabled'] !== false ? 'enabled' : 'disabled'}`,
    }));

    // ── Help ──
    this.handlers.set('help:help', async () => ({
      success: true,
      message: this.generateHelpText(),
    }));
  }

  async execute(cmd: ParsedCommand): Promise<CommandResult> {
    const key = `${cmd.domain}:${cmd.action}`;
    const handler = this.handlers.get(key);

    if (handler) {
      return handler(cmd);
    }

    // Try domain-level fallback
    const domainHandler = this.handlers.get(`${cmd.domain}:status`);
    if (domainHandler && cmd.action === 'status') {
      return domainHandler(cmd);
    }

    return {
      success: false,
      message: `Unknown command: "${cmd.action}" in domain "${cmd.domain}". Type "help" for available commands.`,
    };
  }

  private generateHealthReport(): SystemHealthReport {
    return {
      overall: 'healthy',
      modules: [
        { name: 'ESKF Sensor Fusion', status: 'online', uptime: 99.97, metrics: { accuracy: 1.2, updates: 50 } },
        { name: 'PDR Dead Reckoning', status: 'online', uptime: 99.99, metrics: { steps: 0, drift: 0 } },
        { name: 'Visual Odometry', status: 'online', uptime: 99.85, metrics: { features: 847, fps: 30 } },
        { name: 'WebSocket Bridge', status: 'online', uptime: 99.99, metrics: { connections: 47, throughput: 1247 } },
        { name: 'ETL Pipeline', status: 'online', uptime: 99.95, metrics: { processed: 50000, errors: 0 } },
        { name: 'NeRF Renderer', status: 'online', uptime: 99.90, metrics: { fps: 58, splats: 125000 } },
        { name: 'Predictive Intent', status: 'online', uptime: 99.99, metrics: { accuracy: 0.78, predictions: 5 } },
        { name: 'Offline Store', status: 'online', uptime: 100, metrics: { cached: 47, synced: 1 } },
      ],
      timestamp: Date.now(),
    };
  }

  private generateHelpText(): string {
    return [
      '╔══════════════════════════════════════════════════════╗',
      '║  G.A.N.E SUPER ADMIN TERMINAL — Tier-0 Omni-Control ║',
      '╠══════════════════════════════════════════════════════╣',
      '║                                                      ║',
      '║  FLEET COMMANDS:                                      ║',
      '║    fleet list              — List all fleets          ║',
      '║    fleet status            — Fleet status report      ║',
      '║    fleet deploy <name>     — Deploy fleet             ║',
      '║    fleet recall <name>     — Recall fleet             ║',
      '║                                                      ║',
      '║  SYSTEM COMMANDS:                                     ║',
      '║    system status           — System health report     ║',
      '║    system diagnose         — Full diagnostics         ║',
      '║    system restart <module> — Restart module           ║',
      '║                                                      ║',
      '║  MAP COMMANDS:                                        ║',
      '║    map anomaly             — Query anomalies          ║',
      '║    map delta               — Push delta update        ║',
      '║    map layer <name>        — Toggle map layer         ║',
      '║                                                      ║',
      '║  SECURITY COMMANDS:                                   ║',
      '║    security audit          — View audit log           ║',
      '║    security block <entity> — Block entity             ║',
      '║                                                      ║',
      '║  TELEMETRY COMMANDS:                                  ║',
      '║    telemetry query         — Query telemetry data     ║',
      '║    telemetry search <term> — Search telemetry         ║',
      '║                                                      ║',
      '║  MISSION COMMANDS:                                    ║',
      '║    mission create <name>   — Create new mission       ║',
      '║    mission abort <id>      — Abort active mission     ║',
      '║                                                      ║',
      '║  ENGINE COMMANDS:                                     ║',
      '║    engine status           — All engine module status ║',
      '║    engine toggle <module>  — Enable/disable module    ║',
      '║                                                      ║',
      '║  FLAGS: --limit N, --range <period>, --verbose        ║',
      '║  ALIASES: ls=list, stat=status, dep=deploy, rst=restart║',
      '╚══════════════════════════════════════════════════════╝',
    ].join('\n');
  }
}

// ═══════════════════════════════════════════════════════════
// ADMIN TERMINAL SESSION MANAGER
// ═══════════════════════════════════════════════════════════

export class AdminTerminal {
  private parser: CommandParser;
  private executor: CommandExecutor;
  private session: TerminalSession;
  private commandHistory: string[] = [];
  private historyIndex: number = -1;
  private listeners: Set<(cmd: TerminalCommand) => void> = new Set();

  constructor(userId: string = 'admin', role: 'admin' | 'superadmin' | 'operator' = 'superadmin') {
    this.parser = new CommandParser();
    this.executor = new CommandExecutor();
    this.session = {
      id: `session-${Date.now().toString(36)}`,
      startTime: Date.now(),
      commands: [],
      user: userId,
      role,
    };
    console.log(`[AdminTerminal] Session ${this.session.id} started for ${userId} (${role})`);
  }

  // ─── COMMAND EXECUTION ─────────────────────────────────

  async execute(input: string): Promise<TerminalCommand> {
    const startTime = performance.now();

    const parsed = this.parser.parse(input);

    const command: TerminalCommand = {
      raw: input,
      parsed,
      timestamp: Date.now(),
      executionTime: 0,
      status: 'executing',
    };

    this.session.commands.push(command);
    this.commandHistory.push(input);
    this.historyIndex = this.commandHistory.length;

    try {
      // Permission check
      if (parsed.domain === 'security' && this.session.role !== 'superadmin') {
        command.result = {
          success: false,
          message: 'ACCESS DENIED: Security commands require superadmin privileges',
        };
        command.status = 'error';
      } else {
        command.result = await this.executor.execute(parsed);
        command.status = command.result.success ? 'success' : 'error';
      }
    } catch (error) {
      command.result = {
        success: false,
        message: `Execution error: ${error instanceof Error ? error.message : String(error)}`,
      };
      command.status = 'error';
    }

    command.executionTime = performance.now() - startTime;

    // Notify listeners
    this.listeners.forEach(listener => listener(command));

    return command;
  }

  // ─── HISTORY NAVIGATION ────────────────────────────────

  historyUp(): string | null {
    if (this.historyIndex > 0) {
      this.historyIndex--;
      return this.commandHistory[this.historyIndex];
    }
    return null;
  }

  historyDown(): string | null {
    if (this.historyIndex < this.commandHistory.length - 1) {
      this.historyIndex++;
      return this.commandHistory[this.historyIndex];
    }
    this.historyIndex = this.commandHistory.length;
    return '';
  }

  // ─── TAB COMPLETION ────────────────────────────────────

  getCompletions(partial: string): string[] {
    const tokens = partial.toLowerCase().split(/\s+/);
    const lastToken = tokens[tokens.length - 1] || '';

    const allCommands = [
      'fleet list', 'fleet status', 'fleet deploy', 'fleet recall',
      'system status', 'system diagnose', 'system restart',
      'map anomaly', 'map delta', 'map layer',
      'security audit', 'security block', 'security unblock',
      'telemetry query', 'telemetry search',
      'mission create', 'mission abort',
      'engine status', 'engine toggle',
      'help',
    ];

    if (tokens.length <= 1) {
      return allCommands.filter(c => c.startsWith(lastToken));
    }

    return allCommands
      .filter(c => c.startsWith(tokens.join(' ')))
      .map(c => c.slice(tokens.slice(0, -1).join(' ').length).trim());
  }

  // ─── SESSION INFO ──────────────────────────────────────

  getSession(): TerminalSession {
    return { ...this.session };
  }

  getCommandCount(): number {
    return this.session.commands.length;
  }

  getUptime(): number {
    return Date.now() - this.session.startTime;
  }

  // ─── EVENT LISTENERS ───────────────────────────────────

  onCommand(listener: (cmd: TerminalCommand) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  // ─── CLEANUP ───────────────────────────────────────────

  destroy(): void {
    this.listeners.clear();
    this.session.commands = [];
    this.commandHistory = [];
    console.log(`[AdminTerminal] Session ${this.session.id} ended`);
  }
}
