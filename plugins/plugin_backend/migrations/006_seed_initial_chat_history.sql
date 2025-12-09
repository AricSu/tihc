-- 006_seed_initial_chat_history.sql
-- 准备演示用的会话与对话记录

INSERT INTO chat_sessions (user_id, title)
VALUES (0, 'default seed'),
       (0, 'weekly recap')
ON DUPLICATE KEY UPDATE updated_at = CURRENT_TIMESTAMP;