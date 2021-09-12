use chrono::{DateTime, Utc};

use crate::schema::auth::ResponseUser;

#[derive(juniper::GraphQLObject, sqlx::FromRow)]
pub struct Submission {
    pub id: String,
    pub user: ResponseUser,
    pub reviewer_id: Option<String>,
    pub question: String,
    pub answer: String,
    pub score: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub struct SubmissionForm<'a> {
    pub id: String,
    pub user_id: &'a str,
    pub question: &'a str,
    pub answer: &'a str,
}