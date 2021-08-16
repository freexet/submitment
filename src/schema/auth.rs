use chrono::{DateTime, Utc};
use serde::Serialize;
use std::env;

use crate::util::verify_password;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn verify_password(&self, password: &str) -> Result<(), argon2::password_hash::Error> {
        verify_password(&self.password_hash, password)
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
