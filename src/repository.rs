use sqlx::{PgPool, Result, postgres::{PgPoolOptions}, query_file};
use std::env;

use crate::schema::{
    auth::{UserForm, User, ResponseUser},
    submission::{Submission, SubmissionForm},
};

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
        
        sqlx::migrate!().run(&pool).await?;

        Ok(Repository { pool })
    }

    pub async fn insert_new_user(&self, form: UserForm) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (id, username, password_hash)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
        )
        .bind(&form.id)
        .bind(&form.username)
        .bind(&form.password_hash)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn insert_new_submission<'a>(&self, form: SubmissionForm<'a>) -> Result<Submission> {
        let mut trx = self.pool.begin().await?;

        sqlx::query!(
            r#"
                INSERT INTO submissions (id, user_id, question, answer)
                VALUES ($1, $2, $3, $4)
            "#,
            form.id,
            form.user_id,
            form.question,
            form.answer
        )
        .execute(&mut trx)
        .await?;

        let sub = query_file!("sql/get_submission_by_id.sql", &form.id).fetch_one(&mut trx).await?;

        trx.commit().await?;

        let submission = Submission {
            id: sub.id.clone(),
            user: ResponseUser {
                id: sub.user_id.clone(),
                username: sub.username.clone(),
                created_at: sub.user_created_at.unwrap()
            },
            reviewer_id: sub.reviewer_id.clone(),
            question: sub.question.clone(),
            answer: sub.answer.clone(),
            score: sub.score,
            created_at: sub.created_at.unwrap(),
            updated_at: sub.updated_at.unwrap(),
            deleted_at: None
        };

        Ok(submission)
    }

    pub async fn get_submission_by_id(&self, id: &str) -> Result<Submission> {
        let sub = query_file!("sql/get_submission_by_id.sql", id).fetch_one(&self.pool).await?;
        let submission = Submission {
            id: sub.id.clone(),
            user: ResponseUser {
                id: sub.user_id.clone(),
                username: sub.username.clone(),
                created_at: sub.user_created_at.unwrap()
            },
            reviewer_id: sub.reviewer_id.clone(),
            question: sub.question.clone(),
            answer: sub.answer.clone(),
            score: sub.score,
            created_at: sub.created_at.unwrap(),
            updated_at: sub.updated_at.unwrap(),
            deleted_at: None
        };

        Ok(submission)
    }

    pub async fn get_all_submissions(&self) -> Result<Vec<Submission>> {
        let submissions = query_file!("sql/get_all_submissions.sql").fetch_all(&self.pool).await?;
        let submissions: Vec<Submission> = submissions.iter().map(|sub| Submission {
            id: sub.id.clone(),
            user: ResponseUser {
                id: sub.user_id.clone(),
                username: sub.username.clone(),
                created_at: sub.user_created_at.unwrap()
            },
            reviewer_id: sub.reviewer_id.clone(),
            question: sub.question.clone(),
            answer: sub.answer.clone(),
            score: sub.score,
            created_at: sub.created_at.unwrap(),
            updated_at: sub.updated_at.unwrap(),
            deleted_at: None
        }).collect();

        Ok(submissions)
    }
}
