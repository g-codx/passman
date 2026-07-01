#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] crate::core::error::Error),
    #[error("Required fields are not filled in")]
    EmptyData,
    #[error("Current master password is incorrect")]
    InvalidMasterPassword,
    #[error("New passwords do not match")]
    PasswordMismatch,
}

pub type Result<T> = std::result::Result<T, Error>;
