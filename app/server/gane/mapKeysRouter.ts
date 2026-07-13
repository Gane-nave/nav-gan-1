/**
 * G.A.N.E — Map Provider API Key Management Router
 * =================================================
 * Secure server-side management of map provider API keys.
 * Keys are stored in environment variables and served to the client
 * through a protected tRPC endpoint (never exposed in client bundles).
 *
 * Supports:
 *   - Mapbox Directions API
 *   - HERE Routing API v8
 *   - TomTom Routing API
 *
 * The client MultiProviderRouter calls this endpoint to get keys
 * at runtime, keeping them out of the JavaScript bundle.
 */

import { z } from "zod";
import { protectedProcedure, publicProcedure, router } from "../_core/trpc";
import { ENV } from "../_core/env";

export interface MapProviderStatus {
  id: string;
  name: string;
  configured: boolean;
  keyPrefix?: string; // First 4 chars for verification
}

export const mapKeysRouter = router({
  /** Get status of all map providers (public — no keys exposed) */
  status: publicProcedure.query(() => {
    const providers: MapProviderStatus[] = [
      {
        id: "mapbox",
        name: "Mapbox",
        configured: !!ENV.mapboxApiKey,
        keyPrefix: ENV.mapboxApiKey ? ENV.mapboxApiKey.slice(0, 4) + "..." : undefined,
      },
      {
        id: "here",
        name: "HERE",
        configured: !!ENV.hereApiKey,
        keyPrefix: ENV.hereApiKey ? ENV.hereApiKey.slice(0, 4) + "..." : undefined,
      },
      {
        id: "tomtom",
        name: "TomTom",
        configured: !!ENV.tomtomApiKey,
        keyPrefix: ENV.tomtomApiKey ? ENV.tomtomApiKey.slice(0, 4) + "..." : undefined,
      },
    ];

    return {
      providers,
      configuredCount: providers.filter((p) => p.configured).length,
      totalCount: providers.length,
    };
  }),

  /** Get API keys for authenticated users (protected — keys for client-side routing) */
  getKeys: protectedProcedure.query(() => {
    // Only return keys that are configured
    const keys: Record<string, string> = {};
    if (ENV.mapboxApiKey) keys.mapbox = ENV.mapboxApiKey;
    if (ENV.hereApiKey) keys.here = ENV.hereApiKey;
    if (ENV.tomtomApiKey) keys.tomtom = ENV.tomtomApiKey;

    return {
      keys,
      configuredProviders: Object.keys(keys),
    };
  }),

  /** Proxy a routing request through the server (keeps API keys server-side) */
  proxyRoute: protectedProcedure
    .input(
      z.object({
        provider: z.enum(["mapbox", "here", "tomtom"]),
        url: z.string(), // URL without API key
        method: z.enum(["GET", "POST"]).default("GET"),
        body: z.string().optional(),
      })
    )
    .mutation(async ({ input }) => {
      const keyMap: Record<string, string> = {
        mapbox: ENV.mapboxApiKey,
        here: ENV.hereApiKey,
        tomtom: ENV.tomtomApiKey,
      };

      const apiKey = keyMap[input.provider];
      if (!apiKey) {
        return { error: `${input.provider} API key not configured`, status: 503 };
      }

      // Append API key to URL based on provider
      let fullUrl = input.url;
      const separator = fullUrl.includes("?") ? "&" : "?";

      switch (input.provider) {
        case "mapbox":
          fullUrl += `${separator}access_token=${apiKey}`;
          break;
        case "here":
          fullUrl += `${separator}apikey=${apiKey}`;
          break;
        case "tomtom":
          fullUrl += `${separator}key=${apiKey}`;
          break;
      }

      try {
        const response = await fetch(fullUrl, {
          method: input.method,
          headers: input.body
            ? { "Content-Type": "application/json" }
            : undefined,
          body: input.body || undefined,
        });

        const data = await response.text();

        return {
          status: response.status,
          data: response.ok ? data : null,
          error: response.ok ? null : `Provider returned ${response.status}`,
        };
      } catch (err) {
        return {
          status: 500,
          data: null,
          error: err instanceof Error ? err.message : "Proxy request failed",
        };
      }
    }),
});
