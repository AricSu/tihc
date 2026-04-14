CREATE TABLE `llm_usage_events` (
  `id` varchar(36) NOT NULL,
  `request_id` varchar(128) NOT NULL,
  `principal_id` varchar(36) NULL,
  `case_id` varchar(128) NULL,
  `session_id` varchar(128) NULL,
  `provider` varchar(128) NOT NULL,
  `model` varchar(255) NOT NULL,
  `route` varchar(64) NOT NULL,
  `stream` boolean NOT NULL,
  `success` boolean NOT NULL,
  `started_at` datetime NOT NULL,
  `finished_at` datetime NOT NULL,
  `latency_ms` int NOT NULL,
  `input_tokens` int NULL,
  `output_tokens` int NULL,
  `total_tokens` int NULL,
  `cached_input_tokens` int NULL,
  `reasoning_tokens` int NULL,
  `source` varchar(32) NOT NULL,
  `cost_usd` decimal(12,6) NULL,
  `raw_usage` json NULL,
  `error_code` varchar(128) NULL,
  CONSTRAINT `llm_usage_events_pk` PRIMARY KEY(`id`)
);

--> statement-breakpoint

CREATE INDEX `llm_usage_events_request_idx`
  ON `llm_usage_events` (`request_id`);

--> statement-breakpoint

CREATE INDEX `llm_usage_events_principal_finished_idx`
  ON `llm_usage_events` (`principal_id`, `finished_at`);
