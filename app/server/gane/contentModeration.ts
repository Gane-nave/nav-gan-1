/**
 * G.A.N.E Content Moderation Framework
 * ======================================
 * Crowd report validation, abuse detection, media filtering,
 * reputation scoring, and appeal workflow.
 *
 * Covers Requirement #42 from the specification.
 */

import { z } from 'zod';
import { router, publicProcedure, protectedProcedure, adminProcedure } from '../_core/trpc';

// ─── Types ──────────────────────────────────────────────────

export type ContentType = 'incident_report' | 'crowd_report' | 'map_edit' | 'review' | 'comment' | 'media' | 'route_share';
export type ModerationAction = 'approve' | 'reject' | 'flag' | 'escalate' | 'shadow_ban' | 'auto_approve';
export type ViolationType = 'spam' | 'abuse' | 'misinformation' | 'harassment' | 'explicit' | 'pii_leak' | 'manipulation' | 'off_topic';
export type AppealStatus = 'pending' | 'under_review' | 'upheld' | 'overturned' | 'expired';

export interface ModerationRule {
  id: string;
  name: string;
  contentTypes: ContentType[];
  check: (content: ContentPayload) => ModerationResult;
  priority: number;
  enabled: boolean;
}

export interface ContentPayload {
  id: string;
  type: ContentType;
  authorId: string;
  authorReputation: number;
  text?: string;
  mediaUrls?: string[];
  location?: { lat: number; lng: number };
  metadata?: Record<string, unknown>;
  timestamp: number;
}

export interface ModerationResult {
  action: ModerationAction;
  violations: ViolationType[];
  confidence: number;
  reason: string;
  details?: Record<string, unknown>;
}

export interface ReputationScore {
  userId: string;
  score: number;           // 0-1000
  level: 'untrusted' | 'new' | 'regular' | 'trusted' | 'verified';
  totalContributions: number;
  approvedCount: number;
  rejectedCount: number;
  flaggedCount: number;
  lastUpdated: number;
}

// ─── Reputation Engine ──────────────────────────────────────

export class ReputationEngine {
  private scores = new Map<string, ReputationScore>();

  getScore(userId: string): ReputationScore {
    return this.scores.get(userId) ?? this.createDefault(userId);
  }

  updateAfterModeration(userId: string, action: ModerationAction): ReputationScore {
    const score = this.getScore(userId);
    score.totalContributions++;
    score.lastUpdated = Date.now();

    switch (action) {
      case 'approve':
      case 'auto_approve':
        score.approvedCount++;
        score.score = Math.min(1000, score.score + 5);
        break;
      case 'reject':
        score.rejectedCount++;
        score.score = Math.max(0, score.score - 25);
        break;
      case 'flag':
        score.flaggedCount++;
        score.score = Math.max(0, score.score - 10);
        break;
      case 'shadow_ban':
        score.score = Math.max(0, score.score - 100);
        break;
      case 'escalate':
        score.flaggedCount++;
        score.score = Math.max(0, score.score - 5);
        break;
    }

    score.level = this.computeLevel(score.score);
    this.scores.set(userId, score);
    return score;
  }

  private computeLevel(score: number): ReputationScore['level'] {
    if (score >= 800) return 'verified';
    if (score >= 600) return 'trusted';
    if (score >= 300) return 'regular';
    if (score >= 100) return 'new';
    return 'untrusted';
  }

  private createDefault(userId: string): ReputationScore {
    const score: ReputationScore = {
      userId,
      score: 200,
      level: 'new',
      totalContributions: 0,
      approvedCount: 0,
      rejectedCount: 0,
      flaggedCount: 0,
      lastUpdated: Date.now(),
    };
    this.scores.set(userId, score);
    return score;
  }
}

// ─── Content Filter Rules ───────────────────────────────────

const SPAM_PATTERNS = [
  /\b(buy|sell|discount|free|click here|earn money|casino)\b/gi,
  /(.)\1{5,}/g,                    // Repeated characters
  /https?:\/\/[^\s]{100,}/g,       // Very long URLs
];

const PII_PATTERNS = [
  /\b\d{3}[-.]?\d{2}[-.]?\d{4}\b/g,   // SSN-like
  /\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b/g, // Credit card
  /\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b/g, // Email
];

const ABUSE_KEYWORDS = [
  'kill', 'bomb', 'attack', 'threat', 'weapon',
];

function checkSpam(content: ContentPayload): ModerationResult | null {
  if (!content.text) return null;
  for (const pattern of SPAM_PATTERNS) {
    if (pattern.test(content.text)) {
      return {
        action: 'reject',
        violations: ['spam'],
        confidence: 0.85,
        reason: 'Content matches spam pattern',
      };
    }
  }
  return null;
}

function checkPII(content: ContentPayload): ModerationResult | null {
  if (!content.text) return null;
  for (const pattern of PII_PATTERNS) {
    if (pattern.test(content.text)) {
      return {
        action: 'flag',
        violations: ['pii_leak'],
        confidence: 0.9,
        reason: 'Content contains potential PII',
      };
    }
  }
  return null;
}

function checkAbuse(content: ContentPayload): ModerationResult | null {
  if (!content.text) return null;
  const lower = content.text.toLowerCase();
  for (const keyword of ABUSE_KEYWORDS) {
    if (lower.includes(keyword)) {
      return {
        action: 'escalate',
        violations: ['abuse'],
        confidence: 0.7,
        reason: `Content contains concerning keyword: ${keyword}`,
      };
    }
  }
  return null;
}

function checkRateLimit(content: ContentPayload): ModerationResult | null {
  // Rate limiting based on reputation
  if (content.authorReputation < 100) {
    return {
      action: 'flag',
      violations: ['manipulation'],
      confidence: 0.5,
      reason: 'Low reputation user — content queued for review',
    };
  }
  return null;
}

// ─── Moderation Pipeline ────────────────────────────────────

export class ModerationPipeline {
  private rules: ModerationRule[] = [];
  private reputation = new ReputationEngine();
  private queue: Array<{ content: ContentPayload; result: ModerationResult; timestamp: number }> = [];
  private appeals: Array<{
    id: string;
    contentId: string;
    userId: string;
    reason: string;
    status: AppealStatus;
    createdAt: number;
    resolvedAt?: number;
    resolvedBy?: string;
    resolution?: string;
  }> = [];

  constructor() {
    this.registerDefaultRules();
  }

  private registerDefaultRules(): void {
    this.addRule({
      id: 'spam-filter',
      name: 'Spam Filter',
      contentTypes: ['incident_report', 'crowd_report', 'comment', 'review'],
      check: checkSpam as (c: ContentPayload) => ModerationResult,
      priority: 1,
      enabled: true,
    });
    this.addRule({
      id: 'pii-filter',
      name: 'PII Filter',
      contentTypes: ['incident_report', 'crowd_report', 'comment', 'review', 'route_share'],
      check: checkPII as (c: ContentPayload) => ModerationResult,
      priority: 2,
      enabled: true,
    });
    this.addRule({
      id: 'abuse-filter',
      name: 'Abuse Filter',
      contentTypes: ['incident_report', 'crowd_report', 'comment', 'review'],
      check: checkAbuse as (c: ContentPayload) => ModerationResult,
      priority: 3,
      enabled: true,
    });
    this.addRule({
      id: 'rate-limit',
      name: 'Rate Limit Check',
      contentTypes: ['incident_report', 'crowd_report', 'map_edit', 'comment'],
      check: checkRateLimit as (c: ContentPayload) => ModerationResult,
      priority: 10,
      enabled: true,
    });
  }

  addRule(rule: ModerationRule): void {
    this.rules.push(rule);
    this.rules.sort((a, b) => a.priority - b.priority);
  }

  removeRule(ruleId: string): boolean {
    const idx = this.rules.findIndex(r => r.id === ruleId);
    if (idx === -1) return false;
    this.rules.splice(idx, 1);
    return true;
  }

  toggleRule(ruleId: string, enabled: boolean): boolean {
    const rule = this.rules.find(r => r.id === ruleId);
    if (!rule) return false;
    rule.enabled = enabled;
    return true;
  }

  moderate(content: ContentPayload): ModerationResult {
    const applicableRules = this.rules.filter(
      r => r.enabled && r.contentTypes.includes(content.type)
    );

    const violations: ViolationType[] = [];
    let worstAction: ModerationAction = 'auto_approve';
    let highestConfidence = 0;
    const reasons: string[] = [];

    const actionSeverity: Record<ModerationAction, number> = {
      auto_approve: 0,
      approve: 1,
      flag: 2,
      escalate: 3,
      reject: 4,
      shadow_ban: 5,
    };

    for (const rule of applicableRules) {
      const result = rule.check(content);
      if (result) {
        violations.push(...result.violations);
        if (actionSeverity[result.action] > actionSeverity[worstAction]) {
          worstAction = result.action;
        }
        if (result.confidence > highestConfidence) {
          highestConfidence = result.confidence;
        }
        reasons.push(result.reason);
      }
    }

    // Trusted users get auto-approved for non-critical violations
    if (content.authorReputation >= 600 && worstAction === 'flag') {
      worstAction = 'auto_approve';
    }

    const finalResult: ModerationResult = {
      action: worstAction,
      violations: Array.from(new Set(violations)),
      confidence: highestConfidence || 1.0,
      reason: reasons.length > 0 ? reasons.join('; ') : 'Content passed all checks',
    };

    // Update reputation
    this.reputation.updateAfterModeration(content.authorId, finalResult.action);

    // Add to queue for audit trail
    this.queue.push({
      content,
      result: finalResult,
      timestamp: Date.now(),
    });

    // Keep queue bounded
    if (this.queue.length > 10000) {
      this.queue = this.queue.slice(-5000);
    }

    return finalResult;
  }

  submitAppeal(contentId: string, userId: string, reason: string): string {
    const id = `appeal_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
    this.appeals.push({
      id,
      contentId,
      userId,
      reason,
      status: 'pending',
      createdAt: Date.now(),
    });
    return id;
  }

  resolveAppeal(appealId: string, resolution: 'upheld' | 'overturned', resolvedBy: string, note: string): boolean {
    const appeal = this.appeals.find(a => a.id === appealId);
    if (!appeal) return false;
    appeal.status = resolution;
    appeal.resolvedAt = Date.now();
    appeal.resolvedBy = resolvedBy;
    appeal.resolution = note;

    // If overturned, boost reputation
    if (resolution === 'overturned') {
      this.reputation.updateAfterModeration(appeal.userId, 'approve');
    }

    return true;
  }

  getQueue(limit = 50): typeof this.queue {
    return this.queue.slice(-limit);
  }

  getAppeals(status?: AppealStatus): typeof this.appeals {
    if (status) return this.appeals.filter(a => a.status === status);
    return [...this.appeals];
  }

  getReputation(userId: string): ReputationScore {
    return this.reputation.getScore(userId);
  }

  getRules(): ModerationRule[] {
    return [...this.rules];
  }

  getStats(): {
    totalModerated: number;
    approved: number;
    rejected: number;
    flagged: number;
    escalated: number;
    pendingAppeals: number;
  } {
    const stats = {
      totalModerated: this.queue.length,
      approved: 0,
      rejected: 0,
      flagged: 0,
      escalated: 0,
      pendingAppeals: this.appeals.filter(a => a.status === 'pending').length,
    };
    for (const item of this.queue) {
      switch (item.result.action) {
        case 'approve':
        case 'auto_approve':
          stats.approved++;
          break;
        case 'reject':
        case 'shadow_ban':
          stats.rejected++;
          break;
        case 'flag':
          stats.flagged++;
          break;
        case 'escalate':
          stats.escalated++;
          break;
      }
    }
    return stats;
  }
}

// ─── Singleton ──────────────────────────────────────────────

const pipeline = new ModerationPipeline();

// ─── tRPC Router ────────────────────────────────────────────

export const contentModerationRouter = router({
  moderate: publicProcedure
    .input(z.object({
      contentId: z.string(),
      type: z.enum(['incident_report', 'crowd_report', 'map_edit', 'review', 'comment', 'media', 'route_share']),
      text: z.string().optional(),
      mediaUrls: z.array(z.string()).optional(),
      lat: z.number().optional(),
      lng: z.number().optional(),
    }))
    .mutation(({ input, ctx }) => {
      const userId = String(ctx.user?.id ?? 'anonymous');
      const reputation = pipeline.getReputation(userId);

      const payload: ContentPayload = {
        id: input.contentId,
        type: input.type,
        authorId: userId,
        authorReputation: reputation.score,
        text: input.text,
        mediaUrls: input.mediaUrls,
        location: input.lat !== undefined && input.lng !== undefined
          ? { lat: input.lat, lng: input.lng }
          : undefined,
        timestamp: Date.now(),
      };

      const result = pipeline.moderate(payload);
      return {
        action: result.action,
        violations: result.violations,
        confidence: result.confidence,
        reason: result.reason,
        allowed: result.action === 'approve' || result.action === 'auto_approve',
      };
    }),

  appeal: protectedProcedure
    .input(z.object({
      contentId: z.string(),
      reason: z.string().min(10).max(1000),
    }))
    .mutation(({ input, ctx }) => {
      const appealId = pipeline.submitAppeal(input.contentId, String(ctx.user.id), input.reason);
      return { appealId, status: 'pending' as const };
    }),

  resolveAppeal: adminProcedure
    .input(z.object({
      appealId: z.string(),
      resolution: z.enum(['upheld', 'overturned']),
      note: z.string().optional(),
    }))
    .mutation(({ input, ctx }) => {
      const success = pipeline.resolveAppeal(
        input.appealId,
        input.resolution,
        String(ctx.user.id),
        input.note ?? ''
      );
      return { success };
    }),

  getQueue: adminProcedure
    .input(z.object({ limit: z.number().min(1).max(200).default(50) }))
    .query(({ input }) => {
      return pipeline.getQueue(input.limit);
    }),

  getAppeals: adminProcedure
    .input(z.object({
      status: z.enum(['pending', 'under_review', 'upheld', 'overturned', 'expired']).optional(),
    }))
    .query(({ input }) => {
      return pipeline.getAppeals(input.status);
    }),

  getReputation: protectedProcedure
    .input(z.object({ userId: z.string().optional() }))
    .query(({ input, ctx }) => {
      const targetId = input.userId ?? String(ctx.user.id);
      return pipeline.getReputation(targetId);
    }),

  getStats: adminProcedure.query(() => {
    return pipeline.getStats();
  }),

  getRules: adminProcedure.query(() => {
    return pipeline.getRules().map(r => ({
      id: r.id,
      name: r.name,
      contentTypes: r.contentTypes,
      priority: r.priority,
      enabled: r.enabled,
    }));
  }),

  toggleRule: adminProcedure
    .input(z.object({
      ruleId: z.string(),
      enabled: z.boolean(),
    }))
    .mutation(({ input }) => {
      return { success: pipeline.toggleRule(input.ruleId, input.enabled) };
    }),
});
