use thiserror::Error;

#[derive(Error, Debug)]
pub enum DevVaultError {
    #[error("Vault not initialized. Run `devvault init` first.")]
    VaultNotInitialized,
    #[error("Variable '{key}' not found.")]
    VariableNotFound { key: String },
    #[error("Invalid key-value format. Expected KEY=VALUE.")]
    InvalidKeyValueFormat,
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    #[error("Password error: {0}")]
    PasswordError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),
    #[error("Base64 error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("Walkdir error: {0}")]
    WalkdirError(#[from] walkdir::Error),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

impl From<argon2::password_hash::Error> for DevVaultError {
    fn from(err: argon2::password_hash::Error) -> Self {
        DevVaultError::PasswordError(err.to_string())
    }
}

impl From<chacha20poly1305::Error> for DevVaultError {
    fn from(err: chacha20poly1305::Error) -> Self {
        DevVaultError::EncryptionError(err.to_string())
    }
}