/**
 * G.A.N.E — Automated Cleanup Job
 * =================================
 * Periodic maintenance for the collaboration system:
 * - Mark stale participants as offline (no heartbeat for 2 minutes)
 * - Deactivate expired invite tokens
 * - Archive old collaboration events (older than 7 days)
 * - Deactivate sessions with no online participants for 24 hours
 * 
 * Runs every 60 seconds in production, configurable for testing.
 */

import { getDb } from "../db";
import { eq, lt, and, sql } from "drizzle-orm";
import {
  collaborationParticipants,
  collaborationSessions,
  collaborationInvites,
  collaborationEvents,
} from "../../drizzle/schema";

// ─── Configuration ───
const STALE_HEARTBEAT_MS = 2 * 60 * 1000;         // 2 minutes → mark offline
const EXPIRED_SESSION_MS = 24 * 60 * 60 * 1000;   // 24 hours → deactivate session
const OLD_EVENTS_MS = 7 * 24 * 60 * 60 * 1000;    // 7 days → delete old events
const CLEANUP_INTERVAL_MS = 60 * 1000;             // Run every 60 seconds

export interface CleanupResult {
  staleParticipantsMarkedOffline: number;
  expiredInvitesDeactivated: number;
  oldEventsDeleted: number;
  inactiveSessionsDeactivated: number;
  errors: string[];
}

// ─── Run a single cleanup pass ───
export async function runCleanup(): Promise<CleanupResult> {
  const result: CleanupResult = {
    staleParticipantsMarkedOffline: 0,
    expiredInvitesDeactivated: 0,
    oldEventsDeleted: 0,
    inactiveSessionsDeactivated: 0,
    errors: [],
  };

  const db = await getDb();
  if (!db) return result;

  const now = new Date();

  try {
    // 1. Mark stale participants as offline
    const staleThreshold = new Date(now.getTime() - STALE_HEARTBEAT_MS);
    const staleResult = await db
      .update(collaborationParticipants)
      .set({ isOnline: false })
      .where(
        and(
          eq(collaborationParticipants.isOnline, true),
          lt(collaborationParticipants.lastHeartbeat, staleThreshold)
        )
      );
    const staleRows = staleResult as unknown as [{ affectedRows: number }];
    result.staleParticipantsMarkedOffline = staleRows?.[0]?.affectedRows ?? 0;
  } catch (err) {
    result.errors.push(`Stale participants: ${(err as Error).message}`);
  }

  try {
    // 2. Deactivate expired invite tokens
    const expiredResult = await db
      .update(collaborationInvites)
      .set({ isActive: false })
      .where(
        and(
          eq(collaborationInvites.isActive, true),
          lt(collaborationInvites.expiresAt, now)
        )
      );
    const expiredRows = expiredResult as unknown as [{ affectedRows: number }];
    result.expiredInvitesDeactivated = expiredRows?.[0]?.affectedRows ?? 0;
  } catch (err) {
    result.errors.push(`Expired invites: ${(err as Error).message}`);
  }

  try {
    // 3. Delete old collaboration events (older than 7 days)
    const oldEventsThreshold = new Date(now.getTime() - OLD_EVENTS_MS);
    const deleteResult = await db
      .delete(collaborationEvents)
      .where(lt(collaborationEvents.createdAt, oldEventsThreshold));
    const deleteRows = deleteResult as unknown as [{ affectedRows: number }];
    result.oldEventsDeleted = deleteRows?.[0]?.affectedRows ?? 0;
  } catch (err) {
    result.errors.push(`Old events: ${(err as Error).message}`);
  }

  try {
    // 4. Deactivate sessions with no online participants for 24+ hours
    const inactiveThreshold = new Date(now.getTime() - EXPIRED_SESSION_MS);
    // Find active sessions that haven't been updated in 24 hours
    const staleSessions = await db
      .select({ sessionId: collaborationSessions.sessionId })
      .from(collaborationSessions)
      .where(
        and(
          eq(collaborationSessions.isActive, true),
          lt(collaborationSessions.updatedAt, inactiveThreshold)
        )
      );

    for (const session of staleSessions) {
      // Check if any participants are still online
      const onlineCount = await db
        .select({ count: sql<number>`COUNT(*)` })
        .from(collaborationParticipants)
        .where(
          and(
            eq(collaborationParticipants.sessionId, session.sessionId),
            eq(collaborationParticipants.isOnline, true)
          )
        );

      const count = onlineCount[0]?.count ?? 0;
      if (count === 0) {
        await db
          .update(collaborationSessions)
          .set({ isActive: false })
          .where(eq(collaborationSessions.sessionId, session.sessionId));
        result.inactiveSessionsDeactivated++;
      }
    }
  } catch (err) {
    result.errors.push(`Inactive sessions: ${(err as Error).message}`);
  }

  return result;
}

// ─── Start periodic cleanup ───
let cleanupInterval: ReturnType<typeof setInterval> | null = null;

export function startCleanupJob(intervalMs = CLEANUP_INTERVAL_MS): void {
  if (cleanupInterval) return;

  console.log(`[Cleanup] Starting periodic cleanup job (every ${intervalMs / 1000}s)`);

  cleanupInterval = setInterval(async () => {
    try {
      const result = await runCleanup();
      const totalActions =
        result.staleParticipantsMarkedOffline +
        result.expiredInvitesDeactivated +
        result.oldEventsDeleted +
        result.inactiveSessionsDeactivated;

      if (totalActions > 0) {
        console.log(
          `[Cleanup] Pass complete: ${result.staleParticipantsMarkedOffline} stale participants, ` +
          `${result.expiredInvitesDeactivated} expired invites, ` +
          `${result.oldEventsDeleted} old events, ` +
          `${result.inactiveSessionsDeactivated} inactive sessions`
        );
      }

      if (result.errors.length > 0) {
        console.warn(`[Cleanup] Errors:`, result.errors);
      }
    } catch (err) {
      console.error(`[Cleanup] Fatal error:`, err);
    }
  }, intervalMs);

  // Don't block process exit
  if (cleanupInterval && typeof cleanupInterval === "object" && "unref" in cleanupInterval) {
    cleanupInterval.unref();
  }
}

export function stopCleanupJob(): void {
  if (cleanupInterval) {
    clearInterval(cleanupInterval);
    cleanupInterval = null;
    console.log("[Cleanup] Periodic cleanup job stopped");
  }
}
