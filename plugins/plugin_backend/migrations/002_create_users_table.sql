-- 002_create_users_table.sql
-- 系统用户表与默认管理员账号

CREATE TABLE IF NOT EXISTS tihc_users (
        id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
        user_id BIGINT NULL,
        username VARCHAR(50) NOT NULL,
        password_hash VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL,
        nick_name VARCHAR(100),
        github_name VARCHAR(255),
        avatar VARCHAR(500),
        status TINYINT NOT NULL DEFAULT 1 COMMENT '1: active, 0: inactive',
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        UNIQUE KEY ux_users_username (username),
        UNIQUE KEY ux_users_email (email),
        UNIQUE KEY ux_users_user_id (user_id)
) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci;

-- 默认管理员账号（密码: admin123，对应 bcrypt 哈希）
INSERT INTO tihc_users (user_id, username, password_hash, email, nick_name, github_name, status)
VALUES (
        1,
        'aric',
    '$2b$12$e0blZpuXTkLRZan2QVU9oO0RYWndPz8sAf/Qjh4n9MFLyimRKNDk6',
    'ask.aric.su@gmail.com',
    'aric',
    'AricSu',
    1
)
ON DUPLICATE KEY UPDATE
    password_hash = VALUES(password_hash),
    nick_name = VALUES(nick_name),
    github_name = VALUES(github_name),
    status = VALUES(status);