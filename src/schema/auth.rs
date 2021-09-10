use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{postgres::PgRow, Row};
use std::env;

use crate::util::verify_password;

#[derive(juniper::GraphQLObject, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    #[graphql(skip)]
    password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn verify_password(&self, password: &str) -> Result<(), argon2::password_hash::Error> {
        verify_password(&self.password_hash, password)
    }

    pub fn from_row(row: &PgRow) -> Self {
        User {
            id: row.get("users.id"),
            username: row.get("users.username"),
            password_hash: row.get("users.password_hash"),
            created_at: row.get("users.created_at"),
            updated_at: row.get("users.updated_at"),
        }
    }
}

pub struct CreateUserParams {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize)]
pub struct Claims {
    iss: String,
    exp: i64,
    pub sub: String,
}

impl Claims {
    pub fn new(user_id: &str, exp: DateTime<Utc>) -> color_eyre::Result<Self> {
        Ok(Claims {
            iss: env::var("HOST")?,
            exp: exp.timestamp(),
            sub: user_id.to_string(),
        })
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Token {
    access_token: String,
}

impl Token {
    pub fn new(access_token: String) -> Self {
        Token { access_token }
    }
}
