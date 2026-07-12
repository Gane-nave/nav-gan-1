CREATE TABLE `anomaly_reports` (
	`id` int AUTO_INCREMENT NOT NULL,
	`anomalyId` varchar(64) NOT NULL,
	`deviceId` varchar(64) NOT NULL,
	`userId` int,
	`lat` double NOT NULL,
	`lon` double NOT NULL,
	`type` varchar(32) NOT NULL,
	`confidence` float DEFAULT 1,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `anomaly_reports_id` PRIMARY KEY(`id`),
	CONSTRAINT `idx_reports_unique` UNIQUE(`anomalyId`,`deviceId`)
);
--> statement-breakpoint
CREATE TABLE `delta_updates` (
	`id` int AUTO_INCREMENT NOT NULL,
	`anomalyId` varchar(64) NOT NULL,
	`type` varchar(32) NOT NULL,
	`payload` json NOT NULL,
	`geofenceLat` double NOT NULL,
	`geofenceLon` double NOT NULL,
	`geofenceRadiusM` float DEFAULT 5000,
	`broadcastCount` int DEFAULT 0,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `delta_updates_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `fleets` (
	`id` int AUTO_INCREMENT NOT NULL,
	`name` varchar(128) NOT NULL,
	`nameHe` varchar(128),
	`ownerId` int NOT NULL,
	`type` enum('delivery','emergency','logistics','transit','private') DEFAULT 'private',
	`maxVehicles` int DEFAULT 50,
	`isActive` boolean DEFAULT true,
	`settings` json,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `fleets_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `map_anomalies` (
	`id` int AUTO_INCREMENT NOT NULL,
	`anomalyId` varchar(64) NOT NULL,
	`type` enum('roadblock','pothole','construction','accident','signal_jamming','new_road','road_closure','flooding','speed_trap','hazard','other') NOT NULL,
	`lat` double NOT NULL,
	`lon` double NOT NULL,
	`radiusMeters` float DEFAULT 10,
	`severity` int DEFAULT 1,
	`description` text,
	`descriptionHe` text,
	`reportCount` int DEFAULT 1,
	`confirmedAt` timestamp,
	`isConfirmed` boolean DEFAULT false,
	`isActive` boolean DEFAULT true,
	`reporterDeviceId` varchar(64),
	`reporterUserId` int,
	`expiresAt` timestamp,
	`metadata` json,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `map_anomalies_id` PRIMARY KEY(`id`),
	CONSTRAINT `map_anomalies_anomalyId_unique` UNIQUE(`anomalyId`)
);
--> statement-breakpoint
CREATE TABLE `missions` (
	`id` int AUTO_INCREMENT NOT NULL,
	`fleetId` int NOT NULL,
	`vehicleId` int,
	`driverId` int,
	`dispatcherId` int,
	`status` enum('pending','assigned','in_progress','completed','cancelled','failed') DEFAULT 'pending',
	`priority` int DEFAULT 3,
	`type` enum('delivery','pickup','patrol','emergency','custom') DEFAULT 'delivery',
	`originLat` double,
	`originLon` double,
	`originAddress` text,
	`destinationLat` double,
	`destinationLon` double,
	`destinationAddress` text,
	`waypoints` json,
	`routePolyline` text,
	`estimatedDistance` float,
	`estimatedDuration` float,
	`actualDistance` float,
	`actualDuration` float,
	`timeWindowStart` timestamp,
	`timeWindowEnd` timestamp,
	`notes` text,
	`isGhostTailing` boolean DEFAULT false,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	`completedAt` timestamp,
	CONSTRAINT `missions_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `raw_telemetry` (
	`id` bigint AUTO_INCREMENT NOT NULL,
	`deviceId` varchar(64) NOT NULL,
	`timestamp` timestamp NOT NULL,
	`lat` double NOT NULL,
	`lon` double NOT NULL,
	`alt` float DEFAULT 0,
	`velocity` float DEFAULT 0,
	`heading` float DEFAULT 0,
	`confidenceScore` float DEFAULT 1,
	`navigationMode` enum('BOOTING','OPTIMAL_FUSION','DEGRADED_MODE','EMERGENCY_BOUNDED') DEFAULT 'OPTIMAL_FUSION',
	`satellites` int DEFAULT 0,
	`hdop` float,
	`batteryLevel` float,
	`sensorStatusGnss` boolean DEFAULT true,
	`sensorStatusImu` boolean DEFAULT true,
	`sensorStatusVision` boolean DEFAULT false,
	`sensorStatusNetwork` boolean DEFAULT true,
	CONSTRAINT `raw_telemetry_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `vehicles` (
	`id` int AUTO_INCREMENT NOT NULL,
	`fleetId` int NOT NULL,
	`driverId` int,
	`deviceId` varchar(64) NOT NULL,
	`name` varchar(128),
	`licensePlate` varchar(20),
	`type` enum('car','truck','van','motorcycle','bicycle','ambulance','firetruck','police') DEFAULT 'car',
	`status` enum('active','idle','offline','maintenance','emergency') DEFAULT 'idle',
	`lastLat` double,
	`lastLon` double,
	`lastHeading` float,
	`lastSpeed` float,
	`lastSeen` timestamp,
	`batteryLevel` float,
	`fuelLevel` float,
	`dimensions` json,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `vehicles_id` PRIMARY KEY(`id`),
	CONSTRAINT `vehicles_deviceId_unique` UNIQUE(`deviceId`)
);
--> statement-breakpoint
ALTER TABLE `users` MODIFY COLUMN `role` enum('user','admin','dispatcher','driver','ems','sports') NOT NULL DEFAULT 'user';--> statement-breakpoint
ALTER TABLE `users` ADD `uiProfile` enum('private','sports','ems','logistics') DEFAULT 'private' NOT NULL;--> statement-breakpoint
ALTER TABLE `users` ADD `fleetId` int;--> statement-breakpoint
CREATE INDEX `idx_reports_anomaly` ON `anomaly_reports` (`anomalyId`);--> statement-breakpoint
CREATE INDEX `idx_reports_device` ON `anomaly_reports` (`deviceId`);--> statement-breakpoint
CREATE INDEX `idx_delta_anomaly` ON `delta_updates` (`anomalyId`);--> statement-breakpoint
CREATE INDEX `idx_delta_geofence` ON `delta_updates` (`geofenceLat`,`geofenceLon`);--> statement-breakpoint
CREATE INDEX `idx_delta_created` ON `delta_updates` (`createdAt`);--> statement-breakpoint
CREATE INDEX `idx_anomalies_lat_lon` ON `map_anomalies` (`lat`,`lon`);--> statement-breakpoint
CREATE INDEX `idx_anomalies_type` ON `map_anomalies` (`type`);--> statement-breakpoint
CREATE INDEX `idx_anomalies_active` ON `map_anomalies` (`isActive`,`isConfirmed`);--> statement-breakpoint
CREATE INDEX `idx_anomalies_created` ON `map_anomalies` (`createdAt`);--> statement-breakpoint
CREATE INDEX `idx_missions_fleet` ON `missions` (`fleetId`);--> statement-breakpoint
CREATE INDEX `idx_missions_vehicle` ON `missions` (`vehicleId`);--> statement-breakpoint
CREATE INDEX `idx_missions_status` ON `missions` (`status`);--> statement-breakpoint
CREATE INDEX `idx_missions_created` ON `missions` (`createdAt`);--> statement-breakpoint
CREATE INDEX `idx_telemetry_device_time` ON `raw_telemetry` (`deviceId`,`timestamp`);--> statement-breakpoint
CREATE INDEX `idx_telemetry_time` ON `raw_telemetry` (`timestamp`);--> statement-breakpoint
CREATE INDEX `idx_telemetry_lat_lon` ON `raw_telemetry` (`lat`,`lon`);--> statement-breakpoint
CREATE INDEX `idx_vehicles_fleet` ON `vehicles` (`fleetId`);--> statement-breakpoint
CREATE INDEX `idx_vehicles_driver` ON `vehicles` (`driverId`);--> statement-breakpoint
CREATE INDEX `idx_vehicles_status` ON `vehicles` (`status`);--> statement-breakpoint
CREATE INDEX `idx_vehicles_location` ON `vehicles` (`lastLat`,`lastLon`);