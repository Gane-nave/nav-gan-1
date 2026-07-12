CREATE TABLE `pois` (
	`id` int AUTO_INCREMENT NOT NULL,
	`external_id` varchar(255),
	`source` enum('osm','google','user','import') NOT NULL,
	`name` varchar(500) NOT NULL,
	`name_local` varchar(500),
	`category` varchar(100) NOT NULL,
	`subcategory` varchar(100),
	`lat` text NOT NULL,
	`lon` text NOT NULL,
	`address` text,
	`phone` varchar(50),
	`website` varchar(500),
	`rating` text,
	`rating_count` int DEFAULT 0,
	`price_level` int,
	`open_now` boolean,
	`hours` json,
	`photos` json,
	`tags` json,
	`metadata` json,
	`bounding_box` varchar(100),
	`last_updated` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	`created_at` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `pois_id` PRIMARY KEY(`id`)
);
--> statement-breakpoint
CREATE INDEX `idx_pois_category` ON `pois` (`category`);--> statement-breakpoint
CREATE INDEX `idx_pois_source` ON `pois` (`source`);--> statement-breakpoint
CREATE INDEX `idx_pois_external` ON `pois` (`external_id`);--> statement-breakpoint
CREATE INDEX `idx_pois_bbox` ON `pois` (`bounding_box`);