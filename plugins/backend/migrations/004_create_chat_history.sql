-- 004_create_chat_history.sql
-- 会话表与聊天记录表（按 session_id 分区）

CREATE TABLE IF NOT EXISTS chat_sessions (
        id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
        user_id BIGINT NOT NULL,
        title VARCHAR(255) NOT NULL DEFAULT 'default',
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        UNIQUE KEY uk_chat_sessions_user_title (user_id, title),
        INDEX idx_chat_sessions_user (user_id),
        INDEX idx_chat_sessions_updated_at (updated_at)
) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci;

CREATE TABLE IF NOT EXISTS chat_history (
        id BIGINT NOT NULL AUTO_INCREMENT,
        session_id BIGINT NOT NULL,
        user_id BIGINT NOT NULL,
        user_message TEXT NOT NULL,
        assistant_message TEXT NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY (id, session_id),
        INDEX idx_chat_history_session_created_at (session_id, created_at),
        INDEX idx_chat_history_user_created_at (user_id, created_at)
) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci
    PARTITION BY HASH(session_id) PARTITIONS 16;