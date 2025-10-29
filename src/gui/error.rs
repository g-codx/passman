#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] crate::core::error::Error),
    #[error("Required fields are not filled in")]
    EmptyData
}

pub type Result<T> = std::result::Result<T, Error>;
