pub mod auth;
pub mod submission;

use auth::AuthService;
use submission::SubmissionService;

#[derive(Clone)]
pub struct Service {
    pub auth: AuthService,
    pub submission: SubmissionService,
}
