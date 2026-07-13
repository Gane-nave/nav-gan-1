CREATE TABLE `collaboration_invites` (
	`id` int AUTO_INCREMENT NOT NULL,
	`invite_token` varchar(64) NOT NULL,
	`session_id` varchar(64) NOT NULL,
	`created_by` int NOT NULL,
	`max_uses` int DEFAULT 0,
	`used_count` int NOT NULL DEFAULT 0,
	`expires_at` timestamp NOT NULL,
	`created_at` timestamp NOT NULL DEFAULT (now()),
	`is_active` boolean NOT NULL DEFAULT true,
	CONSTRAINT `collaboration_invites_id` PRIMARY KEY(`id`),
	CONSTRAINT `collaboration_invites_invite_token_unique` UNIQUE(`invite_token`)
);
--> statement-breakpoint
CREATE INDEX `idx_invite_token` ON `collaboration_invites` (`invite_token`);--> statement-breakpoint
CREATE INDEX `idx_invite_session` ON `collaboration_invites` (`session_id`);