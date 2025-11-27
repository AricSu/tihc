-- Create table to store issued authentication tokens with TiDB cache acceleration
CREATE TABLE IF NOT EXISTS auth_tokens (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    token_hash CHAR(64) NOT NULL,
    expires_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    revoked TINYINT(1) NOT NULL DEFAULT 0,
    revoked_at DATETIME NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uniq_token_hash (token_hash),
    KEY idx_auth_tokens_user (user_id),
    KEY idx_auth_tokens_expires (expires_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4  /* CACHED ON */ ;

