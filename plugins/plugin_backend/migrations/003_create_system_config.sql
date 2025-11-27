-- 003_create_system_config.sql
-- 系统运行期配置项存储

CREATE TABLE IF NOT EXISTS tihc_system_config (
        id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
        config_key VARCHAR(100) NOT NULL,
        config_value TEXT NOT NULL,
        config_type ENUM('string', 'number', 'boolean', 'json') NOT NULL DEFAULT 'string',
        is_encrypted BOOLEAN NOT NULL DEFAULT FALSE,
        description TEXT,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        UNIQUE KEY ux_system_config_key (config_key)
) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci;