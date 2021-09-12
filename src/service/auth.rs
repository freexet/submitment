use color_eyre::{eyre::bail, Result};
use nanoid::nanoid;

use crate::error::ServerError;
use crate::repository::Repository;
use crate::schema::auth::{UserForm, Token};
use crate::util::{generate_jwt, hash_password};

#[derive(Clone)]
pub struct AuthService {
    repository: Repository,
}

impl AuthService {
    pub fn new(repository: Repository) -> Self {
        AuthService { repository }
    }

    pub async fn register(&self, username: String, password: String) -> Result<Token> {
        let id = nanoid!();
        let password_hash = match hash_password(&password) {
            Ok(hash) => hash,
            Err(_) => bail!(ServerError::Internal),
        };

        let params = UserForm {
            id,
            username,
            password_hash,
        };

        let user = self.repository.insert_new_user(params).await?;

        Ok(Token::new(generate_jwt(&user.id, 30)?))
    }

    pub async fn login(&self, username: String, password: String) -> Result<Token> {
        let user = self.repository.get_user_by_username(&username).await?;

        if user.verify_password(&password).is_err() {
            bail!(ServerError::Internal)
        };

        Ok(Token::new(generate_jwt(&user.id, 30)?))
    }
}
