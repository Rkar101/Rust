use chrono::{DateTime, Utc};
use sqlx::{PgPool, query, query_scalar};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user_repo(
        &self,
        username: &str,
        email: &str,
        pass_word: &str,
    ) -> Result<(), sqlx::Error> {
        query(
            r#"
            INSERT INTO users (username, email, password_hash)
             Values ($1,$2,$3)
        "#,
        )
        .bind(username)
        .bind(email)
        .bind(pass_word)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn user_data(
        &self,
        user_id: i32,
    ) -> Result<(String, String, DateTime<Utc>), sqlx::Error> {
        let user_data = sqlx::query_as::<_, (String, String, DateTime<Utc>)>(
            r#"
            SELECT username, email, created_at
            FROM users
            WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(user_data)
    }

    pub async fn get_hash_pw(
        &self,
        user_email: &str,
    ) -> Result<Option<(i32, String, String)>, sqlx::Error> {
        sqlx::query_as::<_, (i32, String, String)>(
            r#"
                    SELECT user_id, password_hash, role
                    FROM users
                    WHERE email = $1
        "#,
        )
        .bind(user_email)
        .fetch_optional(&self.pool)
        .await
    }
}
