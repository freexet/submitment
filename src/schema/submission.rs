use chrono::{DateTime, Utc};

use crate::schema::auth::User;

#[derive(juniper::GraphQLObject, sqlx::FromRow)]
pub struct Submission {
    pub id: String,
    pub user: User,
    pub reviewer_id: Option<String>,
    pub question: String,
    pub answer: String,
    pub score: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
