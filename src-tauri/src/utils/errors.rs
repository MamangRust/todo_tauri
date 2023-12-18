use sqlx::{Error as SqlxError, sqlite::SqliteError};
use bcrypt::BcryptError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Hashing error: {0}")]
    HashingError(#[from] BcryptError),

    #[error("SQLx error: {0}")]
    SqlxError(#[from] SqlxError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] SqliteError), 

    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("User not found")]
    UserNotFound,
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

