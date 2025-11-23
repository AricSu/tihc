-- 012_add_chat_session_is_closed.sql
-- Introduce an is_closed flag for chat sessions.

ALTER TABLE chat_sessions
    ADD COLUMN is_closed TINYINT(1) NOT NULL DEFAULT 0 AFTER title;

UPDATE chat_sessions
SET is_closed = 0
WHERE is_closed IS NULL;

CREATE INDEX idx_chat_sessions_closed_updated_at ON chat_sessions (is_closed, updated_at DESC);
