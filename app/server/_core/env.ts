export const ENV = {
  appId: process.env.VITE_APP_ID ?? "",
  cookieSecret: process.env.JWT_SECRET ?? "",
  databaseUrl: process.env.DATABASE_URL ?? "",
  oAuthServerUrl: process.env.OAUTH_SERVER_URL ?? "",
  ownerOpenId: process.env.OWNER_OPEN_ID ?? "",
  isProduction: process.env.NODE_ENV === "production",
  forgeApiUrl: process.env.BUILT_IN_FORGE_API_URL ?? "",
  forgeApiKey: process.env.BUILT_IN_FORGE_API_KEY ?? "",
  redisUrl: process.env.REDIS_URL ?? "",
  sentryDsn: process.env.SENTRY_DSN ?? "",
  stripeSecretKey: process.env.STRIPE_SECRET_KEY || process.env.STRIPE_SK || "",
  stripeWebhookSecret: process.env.STRIPE_WEBHOOK_SECRET ?? "",
  stripePublishableKey: process.env.VITE_STRIPE_PUBLISHABLE_KEY || process.env.VITE_STRIPE_PK || "",
  // Map provider API keys
  mapboxApiKey: process.env.MAPBOX_API_KEY ?? "",
  hereApiKey: process.env.HERE_API_KEY ?? "",
  tomtomApiKey: process.env.TOMTOM_API_KEY ?? "",
};
