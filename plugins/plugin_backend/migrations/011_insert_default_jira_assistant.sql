-- 011_insert_default_jira_assistant.sql
-- Seed a default system user with user_id = 0 for Jira assistant integrations.

INSERT INTO tihc_users (
    user_id,
    username,
    password_hash,
    email,
    nick_name,
    github_name,
    status,
    created_at,
    updated_at
) VALUES (
    0,
    'jira-assistant',
    '$2b$12$e0blZpuXTkLRZan2QVU9oO0RYWndPz8sAf/Qjh4n9MFLyimRKNDk6',
    'jira-assistant@system.local',
    'Jira Assistant',
    NULL,
    1,
    NOW(),
    NOW()
)
ON DUPLICATE KEY UPDATE
    password_hash = VALUES(password_hash),
    email = VALUES(email),
    nick_name = VALUES(nick_name),
    github_name = VALUES(github_name),
    status = VALUES(status),
    updated_at = VALUES(updated_at);
