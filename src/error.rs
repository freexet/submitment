use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Internal server error")]
    Internal,
}
