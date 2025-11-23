use crate::domain::chat::{ChatHistory, ChatSession};
use sqlx::{MySqlPool, Row};

pub struct ChatHistoryRepository {
    pool: MySqlPool,
}

impl ChatHistoryRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: i64,
        session_id: Option<i64>,
        user_message: String,
        assistant_message: String,
    ) -> anyhow::Result<ChatHistory> {
        let session_id = match session_id {
            Some(id) => id,
            None => self.ensure_default_session(user_id).await?,
        };

        let mut tx = self.pool.begin().await?;

        let result = sqlx::query(
            r#"
            INSERT INTO chat_history (session_id, user_id, user_message, assistant_message)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(session_id)
        .bind(user_id)
        .bind(&user_message)
        .bind(&assistant_message)
        .execute(&mut *tx)
        .await?;

        let inserted_id = result.last_insert_id() as i64;

        sqlx::query(
            r#"
            UPDATE chat_sessions
            SET updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(session_id)
        .execute(&mut *tx)
        .await?;

        let row = sqlx::query(
            r#"
            SELECT id, session_id, user_id, user_message, assistant_message, created_at
            FROM chat_history
            WHERE id = ? AND session_id = ?
            "#,
        )
        .bind(inserted_id)
        .bind(session_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ChatHistory {
            id: row.get("id"),
            session_id: row.get("session_id"),
            user_id: row.get::<i64, _>("user_id"),
            user_message: row.get("user_message"),
            assistant_message: row.get("assistant_message"),
            created_at: row.get("created_at"),
        })
    }

    pub async fn find_by_session_id(
        &self,
        session_id: i64,
        limit: i32,
    ) -> anyhow::Result<Vec<ChatHistory>> {
        let rows = sqlx::query(
            r#"
            SELECT id, session_id, user_id, user_message, assistant_message, created_at
            FROM chat_history
            WHERE session_id = ?
            ORDER BY created_at ASC
            LIMIT ?
            "#,
        )
        .bind(session_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ChatHistory {
                id: row.get("id"),
                session_id: row.get("session_id"),
                user_id: row.get("user_id"),
                user_message: row.get("user_message"),
                assistant_message: row.get("assistant_message"),
                created_at: row.get("created_at"),
            })
            .collect())
    }

    pub async fn find_latest_by_user_id(
        &self,
        user_id: i64,
        limit: i32,
    ) -> anyhow::Result<Vec<ChatHistory>> {
        if let Some(session_id) = self.find_latest_session_id(user_id).await? {
            self.find_by_session_id(session_id, limit).await
        } else {
            Ok(Vec::new())
        }
    }

    async fn find_latest_session_id(&self, user_id: i64) -> anyhow::Result<Option<i64>> {
        let session_id = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT id
            FROM chat_sessions
            WHERE user_id = ? AND is_closed = 0
            ORDER BY updated_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session_id)
    }

    async fn ensure_default_session(&self, user_id: i64) -> anyhow::Result<i64> {
        if let Some(session_id) = self.find_latest_session_id(user_id).await? {
            return Ok(session_id);
        }

        let default_session = self
            .create_session(user_id, Some("default".to_string()))
            .await?;

        Ok(default_session.id)
    }

    pub async fn create_session(
        &self,
        user_id: i64,
        title: Option<String>,
    ) -> anyhow::Result<ChatSession> {
        let title = title.unwrap_or_else(|| "default".to_string());

        let mut tx = self.pool.begin().await?;

        let insert_result = sqlx::query(
            r#"
            INSERT INTO chat_sessions (user_id, title, is_closed)
            VALUES (?, ?, 0)
            ON DUPLICATE KEY UPDATE updated_at = CURRENT_TIMESTAMP, is_closed = 0
            "#,
        )
        .bind(user_id)
        .bind(&title)
        .execute(&mut *tx)
        .await?;

        let session_id = if insert_result.last_insert_id() == 0 {
            sqlx::query_scalar::<_, i64>(
                r#"
                SELECT id FROM chat_sessions
                WHERE user_id = ? AND title = ? AND is_closed = 0
                LIMIT 1
                "#,
            )
            .bind(user_id)
            .bind(&title)
            .fetch_one(&mut *tx)
            .await?
        } else {
            insert_result.last_insert_id() as i64
        };

        let row = sqlx::query(
            r#"
            SELECT id, user_id, title, is_closed, created_at, updated_at
            FROM chat_sessions
            WHERE id = ?
            "#,
        )
        .bind(session_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ChatSession {
            id: row.get("id"),
            user_id: row.get::<i64, _>("user_id"),
            title: row.get("title"),
            is_closed: row.get::<i8, _>("is_closed") != 0,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn list_sessions_by_user(
        &self,
        user_id: i64,
        limit: Option<i32>,
    ) -> anyhow::Result<Vec<ChatSession>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, title, is_closed, created_at, updated_at
            FROM chat_sessions
            WHERE user_id = ? AND is_closed = 0
            ORDER BY updated_at DESC
            LIMIT ?
            "#,
        )
        .bind(user_id)
        .bind(limit.unwrap_or(20))
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ChatSession {
                id: row.get("id"),
                user_id: row.get::<i64, _>("user_id"),
                title: row.get("title"),
                is_closed: row.get::<i8, _>("is_closed") != 0,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect())
    }
}
