CREATE TABLE `admin_actions` (
	`id` int AUTO_INCREMENT NOT NULL,
	`actionId` varchar(64) NOT NULL,
	`adminUserId` int NOT NULL,
	`actionType` enum('block_user','unblock_user','promote_user','demote_user','hide_content','show_content','update_content','delete_content','toggle_feature','update_config','maintenance_mode','send_notification','reset_user','force_logout','ai_command','bulk_action') NOT NULL,
	`targetScope` enum('individual','group','all') NOT NULL DEFAULT 'individual',
	`targetUserIds` json,
	`description` text,
	`previousValue` json,
	`newValue` json,
	`aiPrompt` text,
	`status` enum('pending','completed','failed','rolled_back') NOT NULL DEFAULT 'completed',
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `admin_actions_id` PRIMARY KEY(`id`),
	CONSTRAINT `admin_actions_actionId_unique` UNIQUE(`actionId`)
);
--> statement-breakpoint
CREATE TABLE `admin_notifications` (
	`id` int AUTO_INCREMENT NOT NULL,
	`notificationId` varchar(64) NOT NULL,
	`title` varchar(256) NOT NULL,
	`message` text NOT NULL,
	`type` enum('info','warning','success','error','announcement') NOT NULL DEFAULT 'info',
	`targetScope` enum('individual','group','all') NOT NULL DEFAULT 'all',
	`targetUserIds` json,
	`sentBy` int NOT NULL,
	`isActive` boolean NOT NULL DEFAULT true,
	`expiresAt` timestamp,
	`readBy` json,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `admin_notifications_id` PRIMARY KEY(`id`),
	CONSTRAINT `admin_notifications_notificationId_unique` UNIQUE(`notificationId`)
);
--> statement-breakpoint
CREATE TABLE `app_config` (
	`id` int AUTO_INCREMENT NOT NULL,
	`key` varchar(128) NOT NULL,
	`value` json NOT NULL,
	`label` varchar(256),
	`category` enum('general','navigation','notifications','security','performance','ui','integrations','limits') NOT NULL DEFAULT 'general',
	`updatedBy` int,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `app_config_id` PRIMARY KEY(`id`),
	CONSTRAINT `app_config_key_unique` UNIQUE(`key`)
);
--> statement-breakpoint
CREATE TABLE `feature_flags` (
	`id` int AUTO_INCREMENT NOT NULL,
	`key` varchar(128) NOT NULL,
	`label` varchar(256) NOT NULL,
	`description` text,
	`isEnabled` boolean NOT NULL DEFAULT true,
	`scope` enum('global','group','individual') NOT NULL DEFAULT 'global',
	`targetUserIds` json,
	`metadata` json,
	`updatedBy` int,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	`updatedAt` timestamp NOT NULL DEFAULT (now()) ON UPDATE CURRENT_TIMESTAMP,
	CONSTRAINT `feature_flags_id` PRIMARY KEY(`id`),
	CONSTRAINT `feature_flags_key_unique` UNIQUE(`key`)
);
--> statement-breakpoint
CREATE TABLE `user_blocks` (
	`id` int AUTO_INCREMENT NOT NULL,
	`userId` int NOT NULL,
	`blockedBy` int NOT NULL,
	`reason` text,
	`expiresAt` timestamp,
	`isActive` boolean NOT NULL DEFAULT true,
	`createdAt` timestamp NOT NULL DEFAULT (now()),
	CONSTRAINT `user_blocks_id` PRIMARY KEY(`id`),
	CONSTRAINT `idx_blocks_unique` UNIQUE(`userId`,`blockedBy`)
);
--> statement-breakpoint
CREATE INDEX `idx_admin_actions_admin` ON `admin_actions` (`adminUserId`);--> statement-breakpoint
CREATE INDEX `idx_admin_actions_type` ON `admin_actions` (`actionType`);--> statement-breakpoint
CREATE INDEX `idx_admin_actions_scope` ON `admin_actions` (`targetScope`);--> statement-breakpoint
CREATE INDEX `idx_admin_actions_created` ON `admin_actions` (`createdAt`);--> statement-breakpoint
CREATE INDEX `idx_admin_notif_scope` ON `admin_notifications` (`targetScope`);--> statement-breakpoint
CREATE INDEX `idx_admin_notif_active` ON `admin_notifications` (`isActive`);--> statement-breakpoint
CREATE INDEX `idx_admin_notif_created` ON `admin_notifications` (`createdAt`);--> statement-breakpoint
CREATE INDEX `idx_blocks_user` ON `user_blocks` (`userId`);--> statement-breakpoint
CREATE INDEX `idx_blocks_active` ON `user_blocks` (`isActive`);