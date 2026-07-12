import { COOKIE_NAME } from "@shared/const";
import { getSessionCookieOptions } from "./_core/cookies";
import { systemRouter } from "./_core/systemRouter";
import { publicProcedure, router } from "./_core/trpc";
import { telemetryRouter } from "./gane/telemetryRouter";
import { anomalyRouter } from "./gane/anomalyRouter";
import { fleetRouter } from "./gane/fleetRouter";
import { incidentRouter } from "./gane/incidentEngine";
import { liveSharingRouter } from "./gane/liveSharing";
import { crowdRouter } from "./gane/crowdIntelligence";
import { analyticsRouter } from "./gane/analyticsPipeline";
import { observabilityRouter } from "./gane/observabilityMiddleware";
import { masterAdminRouter } from "./gane/masterAdmin";
import { contentModerationRouter } from "./gane/contentModeration";
import { collaborationRouter } from "./gane/collaborationRouter";
import { notificationRouter } from "./gane/notificationRouter";
import { paymentRouter } from "./gane/paymentRouter";
import { mapKeysRouter } from "./gane/mapKeysRouter";
import { poiRouter } from "./gane/poiRouter";

export const appRouter = router({
  system: systemRouter,
  auth: router({
    me: publicProcedure.query(opts => opts.ctx.user),
    logout: publicProcedure.mutation(({ ctx }) => {
      const cookieOptions = getSessionCookieOptions(ctx.req);
      ctx.res.clearCookie(COOKIE_NAME, { ...cookieOptions, maxAge: -1 });
      return {
        success: true,
      } as const;
    }),
  }),

  // G.A.N.E — Planetary Data Matrix
  telemetry: telemetryRouter,
  anomaly: anomalyRouter,
  fleet: fleetRouter,
  incident: incidentRouter,
  liveSharing: liveSharingRouter,
  crowd: crowdRouter,
  analytics: analyticsRouter,
  observability: observabilityRouter,
  admin: masterAdminRouter,
  moderation: contentModerationRouter,
  collaboration: collaborationRouter,
  notifications: notificationRouter,
  payments: paymentRouter,
  mapKeys: mapKeysRouter,
  poi: poiRouter,
});

export type AppRouter = typeof appRouter;
