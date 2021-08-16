use argon2::{
    password_hash::{Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand_core::OsRng;
use std::fs::read;

use crate::schema::auth::Claims;

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password_simple(password.as_bytes(), salt.as_ref())?;

    Ok(hash.to_string())
}

pub fn verify_password(password_hash: &str, password: &str) -> Result<(), Error> {
    let argon = Argon2::default();
    let hash = PasswordHash::new(&password_hash).expect("Failed to parse");

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
