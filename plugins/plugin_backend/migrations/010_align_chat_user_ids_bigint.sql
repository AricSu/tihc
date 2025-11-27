-- 010_align_chat_user_ids_bigint.sql
-- Normalize existing data to BIGINT-based user identifiers and seed default sessions.

-- Ensure tihc_users has aligned user_id values after schema changes.
UPDATE tihc_users
SET user_id = id
WHERE user_id IS NULL;

-- Remove legacy seeded chat data that used non-numeric identifiers.
DELETE FROM chat_history WHERE user_id IS NULL;
DELETE FROM chat_sessions WHERE user_id IS NULL;

-- Align column definitions to BIGINT.
ALTER TABLE chat_sessions
    MODIFY COLUMN user_id BIGINT NOT NULL;

ALTER TABLE chat_history
    MODIFY COLUMN user_id BIGINT NOT NULL;

-- Re-seed demonstration chat data for the default Jira assistant user (user_id = 0).
INSERT INTO chat_sessions (user_id, title)
VALUES (0, 'default seed'),
       (0, 'weekly recap')
ON DUPLICATE KEY UPDATE updated_at = CURRENT_TIMESTAMP;

INSERT INTO chat_history (session_id, user_id, user_message, assistant_message, created_at)
SELECT cs.id,
       cs.user_id,
       m.user_message,
       m.assistant_message,
       m.created_at
FROM chat_sessions AS cs
JOIN (
    SELECT 0 AS user_id,
           'default seed' AS title,
           '今天的巡检报告总结一下？' AS user_message,
           '最新一次巡检覆盖 12 台 TiDB 节点，无告警项，慢查询数量下降 18%。' AS assistant_message,
           NOW() - INTERVAL 2 DAY AS created_at
    UNION ALL
    SELECT 0, 'default seed',
           '那慢查询里有没有需要关注的？',
           '仅剩 2 条慢查询，均来自 batch job，可通过增加索引优化。',
           NOW() - INTERVAL 2 DAY + INTERVAL 10 MINUTE
    UNION ALL
    SELECT 0, 'weekly recap',
           '本周 TiDB 集群有没有容量风险？',
           '当前磁盘利用率 63%，预计安全运行 28 天，无需扩容。',
           NOW() - INTERVAL 5 DAY
    UNION ALL
    SELECT 0, 'weekly recap',
           '那我们要不要提前扩容？',
           '建议观察至下周监控，根据增长趋势调整资源即可。',
           NOW() - INTERVAL 5 DAY + INTERVAL 15 MINUTE
) AS m ON m.user_id = cs.user_id AND m.title = cs.title
WHERE NOT EXISTS (
              SELECT 1
              FROM chat_history h
              WHERE h.session_id = cs.id
                     AND h.user_message = m.user_message
                     AND h.assistant_message = m.assistant_message
);
