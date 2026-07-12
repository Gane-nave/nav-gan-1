CREATE TABLE `collaboration_events` (
	`id` int AUTO_INCREMENT NOT NULL,
	`sessionId` varchar(64) NOT NULL,
	`userId` int NOT NULL,
	`type` enum('user_joined','user_left','marker_added','marker_moved','marker_deleted','annotation_added','annotation_deleted','cursor_moved','session_created','session_ended') NOT NULL,
	`payload` json,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `collaboration_events_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `collaboration_participants` (
	`id` int AUTO_INCREMENT NOT NULL,
	`sessionId` varchar(64) NOT NULL,
	`userId` int NOT NULL,
	`displayName` varchar(128),
	`color` varchar(7) NOT NULL,
	`cursorLat` double,
	`cursorLon` double,
	`isOnline` boolean NOT NULL DEFAULT true,
	`lastHeartbeat` timestamp NOT NULL DEFAULT (now()),
	`joinedAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `collaboration_participants_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE TABLE `collaboration_sessions` (
	`id` int AUTO_INCREMENT NOT NULL,
	`sessionId` varchar(64) NOT NULL,
	`name` varchar(256) NOT NULL,
	`createdBy` int NOT NULL,
	`isActive` boolean NOT NULL DEFAULT true,
	`centerLat` double,
	`centerLon` double,
	`zoomLevel` int DEFAULT 12,
	`maxParticipants` int DEFAULT 20,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `collaboration_sessions_id` PRIMARY KEY(`id`),
	CONSTRAINT `collaboration_sessions_sessionId_unique` UNIQUE(`sessionId`)
);
--> statement-breakpoint
CREATE TABLE `shared_annotations` (
	`id` int AUTO_INCREMENT NOT NULL,
	`annotationId` varchar(64) NOT NULL,
	`sessionId` varchar(64) NOT NULL,
	`userId` int NOT NULL,
	`type` enum('text','route','area','measurement','arrow') NOT NULL,
	`data` json NOT NULL,
	`color` varchar(7) DEFAULT '#00e5ff',
	`isActive` boolean NOT NULL DEFAULT true,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `shared_annotations_id` PRIMARY KEY(`id`),
	CONSTRAINT `shared_annotations_annotationId_unique` UNIQUE(`annotationId`)
);
--> statement-breakpoint
CREATE TABLE `shared_markers` (
	`id` int AUTO_INCREMENT NOT NULL,
	`markerId` varchar(64) NOT NULL,
	`sessionId` varchar(64) NOT NULL,
	`userId` int NOT NULL,
	`lat` double NOT NULL,
	`lon` double NOT NULL,
	`label` varchar(256),
	`description` text,
	`icon` varchar(64) DEFAULT 'pin',
	`color` varchar(7) DEFAULT '#00e5ff',
	`isActive` boolean NOT NULL DEFAULT true,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `shared_markers_id` PRIMARY KEY(`id`),
	CONSTRAINT `shared_markers_markerId_unique` UNIQUE(`markerId`)
);
--> statement-breakpoint
CREATE INDEX `idx_collab_events_session` ON `collaboration_events` (`sessionId`);--> statement-breakpoint
CREATE INDEX `idx_collab_events_user` ON `collaboration_events` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_collab_events_created` ON `collaboration_events` (`createdAt`);--> statement-breakpoint
CREATE INDEX `idx_collab_part_session` ON `collaboration_participants` (`sessionId`);--> statement-breakpoint
CREATE INDEX `idx_collab_part_user` ON `collaboration_participants` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_collab_part_online` ON `collaboration_participants` (`isOnline`);--> statement-breakpoint
CREATE INDEX `idx_collab_session_creator` ON `collaboration_sessions` (`createdBy`);--> statement-breakpoint
CREATE INDEX `idx_collab_session_active` ON `collaboration_sessions` (`isActive`);--> statement-breakpoint
CREATE INDEX `idx_shared_annot_session` ON `shared_annotations` (`sessionId`);--> statement-breakpoint
CREATE INDEX `idx_shared_annot_user` ON `shared_annotations` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_shared_annot_type` ON `shared_annotations` (`type`);--> statement-breakpoint
CREATE INDEX `idx_shared_markers_session` ON `shared_markers` (`sessionId`);--> statement-breakpoint
CREATE INDEX `idx_shared_markers_user` ON `shared_markers` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_shared_markers_active` ON `shared_markers` (`isActive`);