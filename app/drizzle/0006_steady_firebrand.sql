CREATE TABLE `notification_preferences` (
	`id` int AUTO_INCREMENT NOT NULL,
	`user_id` int NOT NULL,
	`enable_info` boolean NOT NULL DEFAULT true,
	`enable_success` boolean NOT NULL DEFAULT true,
	`enable_warning` boolean NOT NULL DEFAULT true,
	`enable_error` boolean NOT NULL DEFAULT true,
	`enable_system` boolean NOT NULL DEFAULT true,
	`enable_collaboration` boolean NOT NULL DEFAULT true,
	`enable_admin` boolean NOT NULL DEFAULT true,
	`enable_sound` boolean NOT NULL DEFAULT true,
	`enable_toast` boolean NOT NULL DEFAULT true,
	`updated_at` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `notification_preferences_id` PRIMARY KEY(`id`),
	CONSTRAINT `notification_preferences_user_id_unique` UNIQUE(`user_id`)
);
--> statement-breakpoint
CREATE TABLE `user_notifications` (
	`id` int AUTO_INCREMENT NOT NULL,
	`notification_id` varchar(64) NOT NULL,
	`user_id` int NOT NULL,
	`type` enum('info','success','warning','error','system','collaboration','admin') NOT NULL DEFAULT 'info',
	`title` varchar(256) NOT NULL,
	`message` text NOT NULL,
	`is_read` boolean NOT NULL DEFAULT false,
	`metadata` json,
	`created_at` timestamp NOT NULL DEFAULT (now()),
	`read_at` timestamp,
	`expires_at` timestamp,
	CONSTRAINT `user_notifications_id` PRIMARY KEY(`id`),
	CONSTRAINT `user_notifications_notification_id_unique` UNIQUE(`notification_id`)
);
--> statement-breakpoint
CREATE INDEX `idx_notif_prefs_user` ON `notification_preferences` (`user_id`);--> statement-breakpoint
CREATE INDEX `idx_user_notif_user` ON `user_notifications` (`user_id`);--> statement-breakpoint
CREATE INDEX `idx_user_notif_read` ON `user_notifications` (`user_id`,`is_read`);--> statement-breakpoint
CREATE INDEX `idx_user_notif_created` ON `user_notifications` (`created_at`);--> statement-breakpoint
CREATE INDEX `idx_user_notif_type` ON `user_notifications` (`type`);