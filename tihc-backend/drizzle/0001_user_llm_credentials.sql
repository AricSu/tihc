ALTER TABLE `principal_settings`
  ADD COLUMN `llm_runtime_json` json NOT NULL;

--> statement-breakpoint

CREATE TABLE `principal_llm_credentials` (
  `principal_id` varchar(36) NOT NULL,
  `provider_id` varchar(128) NOT NULL,
  `api_key` varchar(4096) NOT NULL,
  `updated_at` datetime NOT NULL,
  CONSTRAINT `principal_llm_credentials_pk` PRIMARY KEY(`principal_id`, `provider_id`),
  CONSTRAINT `principal_llm_credentials_principal_fk` FOREIGN KEY (`principal_id`) REFERENCES `principals`(`id`) ON DELETE CASCADE
);

--> statement-breakpoint

CREATE INDEX `principal_llm_credentials_principal_idx`
  ON `principal_llm_credentials` (`principal_id`, `provider_id`);
