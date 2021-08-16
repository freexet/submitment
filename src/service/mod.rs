pub mod auth;

use auth::AuthService;

#[derive(Clone)]
pub struct Service {
    pub auth: AuthService,
}
