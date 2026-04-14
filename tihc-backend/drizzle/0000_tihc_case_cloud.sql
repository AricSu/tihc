CREATE TABLE `principals` (
  `id` varchar(36) NOT NULL,
  `google_sub` varchar(255) NOT NULL,
  `email` varchar(320) NOT NULL,
  `hosted_domain` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL,
  `last_seen_at` datetime NOT NULL,
  CONSTRAINT `principals_google_sub_unique` UNIQUE (`google_sub`),
  CONSTRAINT `principals_pk` PRIMARY KEY(`id`)
);

--> statement-breakpoint

CREATE TABLE `cases` (
  `principal_id` varchar(36) NOT NULL,
  `id` varchar(128) NOT NULL,
  `plugin_id` varchar(128) NOT NULL,
  `title` varchar(256) NOT NULL,
  `activity_state` varchar(32) NOT NULL,
  `resolved_at` datetime NULL,
  `archived_at` datetime NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  `summary` varchar(4000) NOT NULL,
  `signals_json` json NOT NULL,
  `messages_preview_json` json NOT NULL,
  CONSTRAINT `cases_pk` PRIMARY KEY(`principal_id`, `id`),
  CONSTRAINT `cases_principal_fk` FOREIGN KEY (`principal_id`) REFERENCES `principals`(`id`) ON DELETE CASCADE
);

--> statement-breakpoint

CREATE INDEX `cases_principal_updated_idx` ON `cases` (`principal_id`, `updated_at`);

--> statement-breakpoint

CREATE TABLE `case_histories` (
  `principal_id` varchar(36) NOT NULL,
  `case_id` varchar(128) NOT NULL,
  `repository_json` json NOT NULL,
  `updated_at` datetime NOT NULL,
  CONSTRAINT `case_histories_pk` PRIMARY KEY(`principal_id`, `case_id`),
  CONSTRAINT `case_histories_case_fk` FOREIGN KEY (`principal_id`, `case_id`) REFERENCES `cases`(`principal_id`, `id`) ON DELETE CASCADE
);

--> statement-breakpoint

CREATE TABLE `client_import_receipts` (
  `principal_id` varchar(36) NOT NULL,
  `client_id` varchar(128) NOT NULL,
  `imported_at` datetime NOT NULL,
  CONSTRAINT `client_import_receipts_pk` PRIMARY KEY(`principal_id`, `client_id`),
  CONSTRAINT `client_import_receipts_principal_fk` FOREIGN KEY (`principal_id`) REFERENCES `principals`(`id`) ON DELETE CASCADE
);

--> statement-breakpoint

CREATE TABLE `principal_settings` (
  `principal_id` varchar(36) NOT NULL,
  `active_case_id` varchar(128) NULL,
  `analytics_consent` varchar(32) NOT NULL,
  `installed_plugins_json` json NOT NULL,
  `updated_at` datetime NOT NULL,
  CONSTRAINT `principal_settings_pk` PRIMARY KEY(`principal_id`),
  CONSTRAINT `principal_settings_principal_fk` FOREIGN KEY (`principal_id`) REFERENCES `principals`(`id`) ON DELETE CASCADE
);
