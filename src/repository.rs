use sqlx::{postgres::PgPoolOptions, PgPool, Result};
use std::env;

use crate::schema::auth::{CreateUserParams, User};

#[derive(Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new() -> color_eyre::Result<Self> {
        let db_url = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(5))
            .connect(&db_url)
            .await?;

        Ok(Repository { pool })
    }

    pub async fn insert_new_user(&self, params: CreateUserParams) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (id, username, password_hash)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
        )
        .bind(&params.id)
        .bind(&params.username)
        .bind(&params.password_hash)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await
    }
}
