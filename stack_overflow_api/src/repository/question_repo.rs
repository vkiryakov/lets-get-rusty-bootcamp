use uuid::Uuid;

use crate::dto::question_dto::QuestionResponse;

pub struct PgQuestionRepo {
    pg_pool: sqlx::PgPool,
}

impl PgQuestionRepo {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    /// Creates a new question in the database and returns its ID.
    pub async fn create_question(&self, title: &str, body: &str) -> Result<Uuid, sqlx::Error> {
        let rec = sqlx::query!(
            r#"
                INSERT INTO questions (title, body)
                VALUES ($1, $2)
                RETURNING id
            "#,
            title,
            body
        )
        .fetch_one(&self.pg_pool)
        .await?;

        Ok(rec.id)
    }

    /// Retrieves the total count of questions in the database.
    pub async fn get_total_questions_count(&self) -> Result<i64, sqlx::Error> {
        let rec = sqlx::query!(
            r#"
                SELECT COUNT(*) as count
                FROM questions
            "#
        )
        .fetch_one(&self.pg_pool)
        .await?;

        Ok(rec.count.unwrap_or(0))
    }

    /// Retrieves a paginated list of questions from the database.
    pub async fn get_questions(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<QuestionResponse>, sqlx::Error> {
        let recs = sqlx::query_as!(
            QuestionResponse,
            r#"
                SELECT id, title, body, created_at
                FROM questions
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pg_pool)
        .await?;

        Ok(recs)
    }
}