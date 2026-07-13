CREATE TABLE `alerts` (
	`id` int AUTO_INCREMENT NOT NULL,
	`alertId` varchar(64) NOT NULL,
	`userId` int,
	`deviceId` varchar(64),
	`tripId` varchar(64),
	`type` enum('traffic','accident','weather','geofence_enter','geofence_exit','speed_violation','gnss_degradation','gnss_spoofing','gnss_jamming','route_deviation','sos','maintenance','payment','system') NOT NULL,
	`severity` enum('info','warning','critical','emergency') NOT NULL DEFAULT 'info',
	`title` varchar(256) NOT NULL,
	`message` text,
	`lat` double,
	`lon` double,
	`channels` json,
	`deliveryStatus` json,
	`isRead` boolean DEFAULT false,
	`isAcknowledged` boolean DEFAULT false,
	`expiresAt` timestamp,
	`metadata` json,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `alerts_id` PRIMARY KEY(`id`),
	CONSTRAINT `alerts_alertId_unique` UNIQUE(`alertId`)
);
--> statement-breakpoint
CREATE TABLE `devices` (
	`id` int AUTO_INCREMENT NOT NULL,
	`deviceId` varchar(64) NOT NULL,
	`userId` int NOT NULL,
	`name` varchar(128),
	`platform` enum('ios','android','web','embedded','obd2') DEFAULT 'android',
	`osVersion` varchar(32),
	`appVersion` varchar(32),
	`capabilities` json,
	`pushToken` text,
	`isActive` boolean DEFAULT true,
	`lastSeenAt` timestamp,
	`lastLat` double,
	`lastLon` double,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `devices_id` PRIMARY KEY(`id`),
	CONSTRAINT `devices_deviceId_unique` UNIQUE(`deviceId`)
);
--> statement-breakpoint
CREATE TABLE `geofences` (
	`id` int AUTO_INCREMENT NOT NULL,
	`fenceId` varchar(64) NOT NULL,
	`userId` int,
	`fleetId` int,
	`name` varchar(128) NOT NULL,
	`type` enum('circle','polygon') DEFAULT 'circle',
	`centerLat` double,
	`centerLon` double,
	`radiusMeters` float,
	`polygon` json,
	`triggerOn` enum('enter','exit','both') DEFAULT 'both',
	`alertChannels` json,
	`isActive` boolean DEFAULT true,
	`metadata` json,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `geofences_id` PRIMARY KEY(`id`),
	CONSTRAINT `geofences_fenceId_unique` UNIQUE(`fenceId`)
);
--> statement-breakpoint
CREATE TABLE `integration_channels` (
	`id` int AUTO_INCREMENT NOT NULL,
	`userId` int NOT NULL,
	`channelType` enum('whatsapp','gmail','google_drive','sms','push','webhook') NOT NULL,
	`isEnabled` boolean DEFAULT true,
	`config` json,
	`lastUsedAt` timestamp,
	`errorCount` int DEFAULT 0,
	`lastError` text,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `integration_channels_id` PRIMARY KEY(`id`),
	CONSTRAINT `idx_channels_unique` UNIQUE(`userId`,`channelType`)
);
--> statement-breakpoint
CREATE TABLE `log_entries` (
	`id` bigint AUTO_INCREMENT NOT NULL,
	`level` enum('debug','info','warn','error','critical') NOT NULL DEFAULT 'info',
	`source` varchar(64) NOT NULL,
	`action` varchar(128) NOT NULL,
	`userId` int,
	`deviceId` varchar(64),
	`ipAddress` varchar(45),
	`userAgent` text,
	`details` json,
	`errorStack` text,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `log_entries_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `payment_events` (
	`id` int AUTO_INCREMENT NOT NULL,
	`eventId` varchar(64) NOT NULL,
	`userId` int NOT NULL,
	`tripId` varchar(64),
	`type` enum('toll','parking','fuel','charging','subscription','fine','refund') NOT NULL,
	`amount` float NOT NULL,
	`currency` varchar(3) DEFAULT 'ILS',
	`status` enum('pending','completed','failed','refunded') DEFAULT 'pending',
	`provider` varchar(64),
	`externalRef` varchar(128),
	`lat` double,
	`lon` double,
	`metadata` json,
	`alertSent` boolean DEFAULT false,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `payment_events_id` PRIMARY KEY(`id`),
	CONSTRAINT `payment_events_eventId_unique` UNIQUE(`eventId`)
);
--> statement-breakpoint
CREATE TABLE `routes` (
	`id` int AUTO_INCREMENT NOT NULL,
	`routeId` varchar(64) NOT NULL,
	`tripId` varchar(64),
	`userId` int,
	`originLat` double NOT NULL,
	`originLon` double NOT NULL,
	`destinationLat` double NOT NULL,
	`destinationLon` double NOT NULL,
	`polyline` text,
	`distanceMeters` float,
	`durationSeconds` float,
	`trafficDelaySeconds` float,
	`tollCost` float,
	`routeType` enum('fastest','shortest','eco','avoid_tolls','avoid_highways','scenic') DEFAULT 'fastest',
	`constraints` json,
	`alternatives` json,
	`computeTimeMs` int,
	`isActive` boolean DEFAULT true,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `routes_id` PRIMARY KEY(`id`),
	CONSTRAINT `routes_routeId_unique` UNIQUE(`routeId`)
);
--> statement-breakpoint
CREATE TABLE `trip_events` (
	`id` bigint AUTO_INCREMENT NOT NULL,
	`eventId` varchar(64) NOT NULL,
	`tripId` varchar(64) NOT NULL,
	`deviceId` varchar(64) NOT NULL,
	`userId` int,
	`eventType` enum('trip_start','trip_end','trip_pause','trip_resume','route_compute','reroute','arrival','failure','gnss_degradation','gnss_recovery','gnss_spoofing_detected','sync_event','notification_dispatch','payment_alert','geofence_enter','geofence_exit','speed_violation','waypoint_reached','eta_update','traffic_update') NOT NULL,
	`timestampDevice` bigint NOT NULL,
	`timestampServer` bigint NOT NULL,
	`lat` double,
	`lon` double,
	`accuracy` float,
	`confidence` float,
	`latencyMs` int,
	`outcome` enum('success','failure','degraded','timeout') DEFAULT 'success',
	`payload` json,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `trip_events_id` PRIMARY KEY(`id`),
	CONSTRAINT `trip_events_eventId_unique` UNIQUE(`eventId`)
);
--> statement-breakpoint
CREATE TABLE `trips` (
	`id` int AUTO_INCREMENT NOT NULL,
	`tripId` varchar(64) NOT NULL,
	`userId` int NOT NULL,
	`deviceId` varchar(64) NOT NULL,
	`status` enum('planned','active','paused','completed','cancelled','failed') NOT NULL DEFAULT 'planned',
	`originLat` double NOT NULL,
	`originLon` double NOT NULL,
	`originAddress` text,
	`destinationLat` double,
	`destinationLon` double,
	`destinationAddress` text,
	`routeId` varchar(64),
	`distanceMeters` float,
	`durationSeconds` float,
	`avgSpeed` float,
	`maxSpeed` float,
	`rerouteCount` int DEFAULT 0,
	`gnssQualityAvg` float,
	`startedAt` timestamp,
	`completedAt` timestamp,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `trips_id` PRIMARY KEY(`id`),
	CONSTRAINT `trips_tripId_unique` UNIQUE(`tripId`)
);
--> statement-breakpoint
CREATE TABLE `waypoints` (
	`id` int AUTO_INCREMENT NOT NULL,
	`routeId` varchar(64) NOT NULL,
	`tripId` varchar(64),
	`sequence` int NOT NULL,
	`lat` double NOT NULL,
	`lon` double NOT NULL,
	`address` text,
	`name` varchar(256),
	`type` enum('origin','destination','via','stop','fuel','rest','charging') DEFAULT 'via',
	`arrivalTime` timestamp,
	`departureTime` timestamp,
	`dwellSeconds` int,
	`isReached` boolean DEFAULT false,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `waypoints_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
ALTER TABLE `raw_telemetry` ADD `tripId` varchar(64);--> statement-breakpoint
ALTER TABLE `raw_telemetry` ADD `retentionTier` enum('hot','warm','cold','archive') DEFAULT 'hot';--> statement-breakpoint
ALTER TABLE `users` ADD `preferences` json;--> statement-breakpoint
CREATE INDEX `idx_alerts_user` ON `alerts` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_alerts_type` ON `alerts` (`type`);--> statement-breakpoint
CREATE INDEX `idx_alerts_severity` ON `alerts` (`severity`);--> statement-breakpoint
CREATE INDEX `idx_alerts_created` ON `alerts` (`createdAt`);--> statement-breakpoint
CREATE INDEX `idx_alerts_trip` ON `alerts` (`tripId`);--> statement-breakpoint
CREATE INDEX `idx_devices_user` ON `devices` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_devices_active` ON `devices` (`isActive`);--> statement-breakpoint
CREATE INDEX `idx_geofences_user` ON `geofences` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_geofences_fleet` ON `geofences` (`fleetId`);--> statement-breakpoint
CREATE INDEX `idx_geofences_center` ON `geofences` (`centerLat`,`centerLon`);--> statement-breakpoint
CREATE INDEX `idx_geofences_active` ON `geofences` (`isActive`);--> statement-breakpoint
CREATE INDEX `idx_channels_user` ON `integration_channels` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_channels_type` ON `integration_channels` (`channelType`);--> statement-breakpoint
CREATE INDEX `idx_logs_level` ON `log_entries` (`level`);--> statement-breakpoint
CREATE INDEX `idx_logs_source` ON `log_entries` (`source`);--> statement-breakpoint
CREATE INDEX `idx_logs_user` ON `log_entries` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_logs_created` ON `log_entries` (`createdAt`);--> statement-breakpoint
CREATE INDEX `idx_payment_user` ON `payment_events` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_payment_trip` ON `payment_events` (`tripId`);--> statement-breakpoint
CREATE INDEX `idx_payment_type` ON `payment_events` (`type`);--> statement-breakpoint
CREATE INDEX `idx_payment_status` ON `payment_events` (`status`);--> statement-breakpoint
CREATE INDEX `idx_routes_trip` ON `routes` (`tripId`);--> statement-breakpoint
CREATE INDEX `idx_routes_user` ON `routes` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_trip_events_trip` ON `trip_events` (`tripId`);--> statement-breakpoint
CREATE INDEX `idx_trip_events_type` ON `trip_events` (`eventType`);--> statement-breakpoint
CREATE INDEX `idx_trip_events_device` ON `trip_events` (`deviceId`);--> statement-breakpoint
CREATE INDEX `idx_trip_events_ts` ON `trip_events` (`timestampServer`);--> statement-breakpoint
CREATE INDEX `idx_trips_user` ON `trips` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_trips_device` ON `trips` (`deviceId`);--> statement-breakpoint
CREATE INDEX `idx_trips_status` ON `trips` (`status`);--> statement-breakpoint
CREATE INDEX `idx_trips_started` ON `trips` (`startedAt`);--> statement-breakpoint
CREATE INDEX `idx_waypoints_route` ON `waypoints` (`routeId`);--> statement-breakpoint
CREATE INDEX `idx_waypoints_trip` ON `waypoints` (`tripId`);--> statement-breakpoint
CREATE INDEX `idx_telemetry_trip` ON `raw_telemetry` (`tripId`);--> statement-breakpoint
CREATE INDEX `idx_telemetry_retention` ON `raw_telemetry` (`retentionTier`);