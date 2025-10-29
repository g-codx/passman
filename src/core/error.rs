#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    AesGcm(#[from] aes_gcm::Error),
    #[error(transparent)]
    InvalidKeyLength(#[from] aes_gcm::aes::cipher::InvalidLength),
    #[error("Ciphertext too short")]
    ShortCiphertext,
    #[error("{0}")]
    Argon2(argon2::Error),
    #[error("{0}")]
    Storage(String),
}

pub type Result<T> = std::result::Result<T, Error>;
