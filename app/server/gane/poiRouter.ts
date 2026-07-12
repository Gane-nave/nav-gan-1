/**
 * POI Router — Points of Interest tRPC procedures
 * 
 * Provides:
 *  - Geo-radius search with category filtering
 *  - POI details with photos and hours
 *  - User-submitted POI creation
 *  - POI ingestion from OSM (batch)
 */

import { z } from "zod";
import { publicProcedure, protectedProcedure, router } from "../_core/trpc";
import { getDb } from "../db";
import { pois } from "../../drizzle/schema";
import { eq, and, like, sql, desc } from "drizzle-orm";

const POI_CATEGORIES = [
  'restaurant', 'cafe', 'bar', 'fast_food',
  'gas_station', 'ev_charging', 'parking',
  'hospital', 'pharmacy', 'clinic',
  'hotel', 'motel', 'hostel',
  'supermarket', 'convenience', 'mall',
  'bank', 'atm',
  'police', 'fire_station', 'embassy',
  'school', 'university', 'library',
  'park', 'beach', 'playground',
  'museum', 'cinema', 'theater',
  'bus_station', 'train_station', 'airport', 'ferry',
  'car_repair', 'car_wash', 'car_rental',
  'mosque', 'church', 'synagogue', 'temple',
  'gym', 'swimming_pool', 'stadium',
  'post_office', 'government',
  'tourist_attraction', 'viewpoint',
  'other',
] as const;

export const poiRouter = router({
  /**
   * Search POIs by location, category, and text query
   * Uses Haversine formula for geo-radius filtering in SQL
   */
  search: publicProcedure
    .input(z.object({
      lat: z.number().min(-90).max(90),
      lon: z.number().min(-180).max(180),
      radiusMeters: z.number().min(100).max(50000).default(2000),
      category: z.enum(POI_CATEGORIES).optional(),
      query: z.string().max(200).optional(),
      limit: z.number().min(1).max(100).default(30),
      offset: z.number().min(0).default(0),
    }))
    .query(async ({ input }) => {
      const { lat, lon, radiusMeters, category, query, limit, offset } = input;

      // Bounding box pre-filter (fast index scan)
      const latDelta = radiusMeters / 111320;
      const lonDelta = radiusMeters / (111320 * Math.cos(lat * Math.PI / 180));
      const minLat = lat - latDelta;
      const maxLat = lat + latDelta;
      const minLon = lon - lonDelta;
      const maxLon = lon + lonDelta;

      // Build conditions
      const conditions = [
        sql`CAST(${pois.lat} AS DECIMAL(10,7)) BETWEEN ${minLat} AND ${maxLat}`,
        sql`CAST(${pois.lon} AS DECIMAL(10,7)) BETWEEN ${minLon} AND ${maxLon}`,
      ];

      if (category) {
        conditions.push(eq(pois.category, category));
      }

      if (query) {
        conditions.push(like(pois.name, `%${query}%`));
      }

      // Query with Haversine distance calculation
      const db = (await getDb())!;
      const results = await db
        .select({
          id: pois.id,
          externalId: pois.externalId,
          source: pois.source,
          name: pois.name,
          nameLocal: pois.nameLocal,
          category: pois.category,
          subcategory: pois.subcategory,
          lat: pois.lat,
          lon: pois.lon,
          address: pois.address,
          phone: pois.phone,
          website: pois.website,
          rating: pois.rating,
          ratingCount: pois.ratingCount,
          priceLevel: pois.priceLevel,
          openNow: pois.openNow,
          hours: pois.hours,
          photos: pois.photos,
          tags: pois.tags,
          distance: sql<number>`(
            6371000 * 2 * ASIN(SQRT(
              POW(SIN((RADIANS(CAST(${pois.lat} AS DECIMAL(10,7))) - RADIANS(${lat})) / 2), 2) +
              COS(RADIANS(${lat})) * COS(RADIANS(CAST(${pois.lat} AS DECIMAL(10,7)))) *
              POW(SIN((RADIANS(CAST(${pois.lon} AS DECIMAL(10,7))) - RADIANS(${lon})) / 2), 2)
            ))
          )`.as('distance'),
        })
        .from(pois)
        .where(and(...conditions))
        .orderBy(sql`distance`)
        .limit(limit)
        .offset(offset);

      // Filter by actual radius (Haversine is more accurate than bounding box)
      const filtered = results.filter((r: typeof results[number]) => r.distance <= radiusMeters);

      return {
        pois: filtered,
        total: filtered.length,
        hasMore: results.length === limit,
      };
    }),

  /**
   * Get POI details by ID
   */
  getById: publicProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input }) => {
      const db = (await getDb())!;
      const [poi] = await db
        .select()
        .from(pois)
        .where(eq(pois.id, input.id))
        .limit(1);
      return poi ?? null;
    }),

  /**
   * User-submitted POI
   */
  create: protectedProcedure
    .input(z.object({
      name: z.string().min(1).max(500),
      category: z.enum(POI_CATEGORIES),
      lat: z.number().min(-90).max(90),
      lon: z.number().min(-180).max(180),
      address: z.string().max(500).optional(),
      phone: z.string().max(50).optional(),
      website: z.string().max(500).optional(),
      tags: z.array(z.string()).optional(),
    }))
    .mutation(async ({ input }) => {
      const db = (await getDb())!;
      const [result] = await db.insert(pois).values({
        source: 'user',
        name: input.name,
        category: input.category,
        lat: String(input.lat),
        lon: String(input.lon),
        address: input.address,
        phone: input.phone,
        website: input.website,
        tags: input.tags ?? [],
      });
      return { id: result.insertId };
    }),

  /**
   * Batch ingest POIs from OSM (called by background sync)
   */
  ingestBatch: protectedProcedure
    .input(z.object({
      pois: z.array(z.object({
        externalId: z.string(),
        source: z.enum(['osm', 'google', 'import']),
        name: z.string(),
        nameLocal: z.string().optional(),
        category: z.enum(POI_CATEGORIES),
        subcategory: z.string().optional(),
        lat: z.number(),
        lon: z.number(),
        address: z.string().optional(),
        phone: z.string().optional(),
        website: z.string().optional(),
        rating: z.string().optional(),
        ratingCount: z.number().optional(),
        hours: z.record(z.string(), z.string()).optional(),
        tags: z.array(z.string()).optional(),
      })).min(1),
    }))
    .mutation(async ({ input }) => {
      let inserted = 0;
      let updated = 0;

      const db = (await getDb())!;
      for (const poi of input.pois) {
        // Upsert: check if exists by externalId
        const [existing] = await db
          .select({ id: pois.id })
          .from(pois)
          .where(eq(pois.externalId, poi.externalId))
          .limit(1);

        if (existing) {
          await db.update(pois)
            .set({
              name: poi.name,
              nameLocal: poi.nameLocal,
              category: poi.category,
              lat: String(poi.lat),
              lon: String(poi.lon),
              address: poi.address,
              phone: poi.phone,
              website: poi.website,
              rating: poi.rating,
              ratingCount: poi.ratingCount,
              hours: poi.hours,
              tags: poi.tags,
            })
            .where(eq(pois.id, existing.id));
          updated++;
        } else {
          await db.insert(pois).values({
            externalId: poi.externalId,
            source: poi.source,
            name: poi.name,
            nameLocal: poi.nameLocal,
            category: poi.category,
            subcategory: poi.subcategory,
            lat: String(poi.lat),
            lon: String(poi.lon),
            address: poi.address,
            phone: poi.phone,
            website: poi.website,
            rating: poi.rating,
            ratingCount: poi.ratingCount,
            hours: poi.hours,
            tags: poi.tags,
          });
          inserted++;
        }
      }

      return { inserted, updated, total: input.pois.length };
    }),

  /**
   * Get category statistics
   */
  categoryStats: publicProcedure.query(async () => {
    const db = await getDb();
    const stats = await db!
      .select({
        category: pois.category,
        count: sql<number>`COUNT(*)`.as('count'),
      })
      .from(pois)
      .groupBy(pois.category)
      .orderBy(desc(sql`count`));

    return stats;
  }),
});
