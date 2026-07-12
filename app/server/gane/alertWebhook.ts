/**
 * G.A.N.E — Alert Webhook Handler
 * ==================================
 * Receives Alertmanager webhook notifications and processes them.
 * Logs alerts, notifies the owner, and stores alert history.
 *
 * Endpoint: POST /api/webhooks/alerts
 */
import type { Request, Response } from "express";
import { notifyOwner } from "../_core/notification";

// ─── Types ───
interface AlertmanagerPayload {
  version: string;
  groupKey: string;
  status: "firing" | "resolved";
  receiver: string;
  groupLabels: Record<string, string>;
  commonLabels: Record<string, string>;
  commonAnnotations: Record<string, string>;
  externalURL: string;
  alerts: AlertmanagerAlert[];
}

interface AlertmanagerAlert {
  status: "firing" | "resolved";
  labels: Record<string, string>;
  annotations: Record<string, string>;
  startsAt: string;
  endsAt: string;
  generatorURL: string;
  fingerprint: string;
}

// ─── In-Memory Alert History (last 100 alerts) ───
const alertHistory: {
  timestamp: string;
  alertName: string;
  severity: string;
  status: string;
  summary: string;
  component: string;
}[] = [];

const MAX_ALERT_HISTORY = 100;

// ─── Webhook Handler ───
export async function alertWebhookHandler(req: Request, res: Response): Promise<void> {
  try {
    const payload = req.body as AlertmanagerPayload;
    const priority = req.query.priority as string | undefined;
    const team = req.query.team as string | undefined;

    if (!payload || !payload.alerts) {
      res.status(400).json({ error: "Invalid Alertmanager payload" });
      return;
    }

    console.log(
      `[AlertWebhook] Received ${payload.alerts.length} alert(s) — status: ${payload.status}, receiver: ${payload.receiver}`
    );

    for (const alert of payload.alerts) {
      const alertName = alert.labels.alertname || "Unknown";
      const severity = alert.labels.severity || "unknown";
      const component = alert.labels.component || "unknown";
      const summary = alert.annotations.summary || "No summary";
      const description = alert.annotations.description || "";

      // Store in history
      alertHistory.unshift({
        timestamp: new Date().toISOString(),
        alertName,
        severity,
        status: alert.status,
        summary,
        component,
      });

      // Trim history
      if (alertHistory.length > MAX_ALERT_HISTORY) {
        alertHistory.length = MAX_ALERT_HISTORY;
      }

      // Log the alert
      const emoji = alert.status === "firing" ? "🔴" : "✅";
      const severityTag = severity === "critical" ? "[CRITICAL]" : "[WARNING]";
      console.log(
        `[AlertWebhook] ${emoji} ${severityTag} ${alertName} (${component}): ${summary}`
      );

      // Notify owner for critical alerts or all firing alerts
      if (alert.status === "firing" && (severity === "critical" || priority === "critical")) {
        try {
          await notifyOwner({
            title: `${severityTag} ${alertName}`,
            content: `**Component:** ${component}\n\n${summary}\n\n${description}\n\n**Status:** ${alert.status}\n**Started:** ${alert.startsAt}`,
          });
        } catch (err) {
          console.warn("[AlertWebhook] Failed to notify owner:", (err as Error).message);
        }
      }
    }

    res.status(200).json({
      status: "ok",
      processed: payload.alerts.length,
    });
  } catch (err) {
    console.error("[AlertWebhook] Error processing alert:", err);
    res.status(500).json({ error: "Internal error processing alert" });
  }
}

// ─── Alert History Endpoint ───
export function alertHistoryHandler(_req: Request, res: Response): void {
  res.json({
    total: alertHistory.length,
    alerts: alertHistory,
  });
}
