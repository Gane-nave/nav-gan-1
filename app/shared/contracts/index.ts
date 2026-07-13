/**
 * G.A.N.E — CONTRACTS BARREL EXPORT
 * 14 formal contract modules
 */

// ─── Core Contracts ─────────────────────────────────────
export * from './apiEnvelope';
export * from './stateMachines';
export * from './schemaRegistry';
export * from './eventCatalog';
export * from './interfaceContracts';
export * from './mathModels';

// ─── Production Infrastructure ──────────────────────────
export * from './productionInfra';
export * from './dataEngineering';

// ─── Reliability & Security ─────────────────────────────
export * from './sloCatalog';
export * from './securityModel';
export * from './failureMatrix';

// ─── Data & ML ──────────────────────────────────────────
export * from './dataLineage';
export * from './configSystem';
export * from './modelRegistry';
export * from './evidenceChain';

// ─── System Engineering ─────────────────────────────────
export * from './fieldTestProgram';
export * from './systemEngineering';

// ─── Audit & Verification ─────────────────────────────
export * from './coreSystemAudit';
export * from './securityRedTeam';
export * from './formalVerification';
export * from './advancedTesting';
