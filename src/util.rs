use argon2::{
    password_hash::{Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{Duration, Utc};
use color_eyre::eyre::eyre;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use std::{env, fs::read};

use crate::graphql::schema::Context;
use crate::schema::auth::Claims;

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), salt.as_ref())?;

    Ok(hash.to_string())
}

pub fn verify_password(password_hash: &str, password: &str) -> Result<(), Error> {
    let argon = Argon2::default();
    let hash = PasswordHash::new(password_hash).expect("Failed to parse");

    argon.verify_password(password.as_bytes(), &hash)
}

pub fn generate_jwt(user_id: &str, expires_in_minute: i64) -> color_eyre::Result<String> {
    let private_key = read("private.pem")?;

    let key = EncodingKey::from_ec_pem(&private_key)?;
    let exp = Utc::now() + Duration::minutes(expires_in_minute);

    let claims = Claims::new(user_id, exp)?;
    let header = Header::new(Algorithm::ES256);

    encode(&header, &claims, &key).map_err(|e| e.into())
}

pub fn get_token(ctx: &Context) -> color_eyre::Result<&str> {
    let auth_header = match &ctx.auth {
        Some(header) => header,
        None => return Err(eyre!("Unauthorized")),
    };

    let token: Vec<&str> = auth_header.split_ascii_whitespace().collect();

    if *token.get(0).unwrap() != "Bearer" {
        return Err(eyre!("Unauthorized"));
    }

    match token.get(1) {
        Some(token) => Ok(*token),
        None => Err(eyre!("Unauthorized")),
    }
}

pub fn authenticate(ctx: &Context) -> color_eyre::Result<String> {
    let token = get_token(ctx)?;

    let public_key = read("public.pem")?;
    let key = DecodingKey::from_ec_pem(&public_key)?;

    let validation = Validation {
        iss: env::var("HOST").ok(),
        algorithms: vec![Algorithm::ES256],
        ..Default::default()
    };

    match decode::<Claims>(token, &key, &validation) {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err(eyre!("Unauthorized")),
    }
}
