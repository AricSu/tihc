-- 001_create_user_providers.sql
-- 用户与第三方账号的绑定信息

CREATE TABLE IF NOT EXISTS tihc_user_providers (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    provider VARCHAR(100) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    provider_email VARCHAR(255),
    provider_raw JSON,
    refresh_token_encrypted TEXT,
    scope TEXT,
    token_expires_at TIMESTAMP NULL,
    last_synced_at TIMESTAMP NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY ux_user_providers_provider_user (provider, provider_user_id),
    INDEX idx_user_providers_user (user_id)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;