use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::crypto;
use crate::error::DevVaultError;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub version: u32,
    pub variables: BTreeMap<String, VaultEntry>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct VaultEntry {
    pub value: String,
    pub added_at: DateTime<Utc>,
    pub source: Option<String>,
}

impl Vault {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            version: 1,
            variables: BTreeMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

fn get_vault_path() -> PathBuf {
    PathBuf::from(".devvault").join("vault.enc")
}

#[allow(dead_code)]
fn get_config() -> Result<Config, DevVaultError> {
    Config::load()
}

fn load_vault(password: &[u8]) -> Result<Vault, DevVaultError> {
    let vault_path = get_vault_path();
    if !vault_path.exists() {
        return Err(DevVaultError::VaultNotInitialized);
    }
    let encrypted_data = fs::read(&vault_path)?;
    let decrypted_data = crypto::decrypt(&encrypted_data, password)?;
    let vault: Vault = serde_json::from_slice(&decrypted_data)?;
    Ok(vault)
}

fn save_vault(vault: &Vault, password: &[u8]) -> Result<(), DevVaultError> {
    let vault_path = get_vault_path();
    let json_data = serde_json::to_vec(vault)?;
    let encrypted_data = crypto::encrypt(&json_data, password)?;
    fs::write(&vault_path, encrypted_data)?;
    Ok(())
}

fn prompt_password() -> Result<String, DevVaultError> {
    // For now, use a simple environment variable or default password
    // In a real implementation, we'd use rpassword or similar
    let password = std::env::var("DEVVAULT_PASSWORD").unwrap_or_else(|_| "default_password".to_string());
    Ok(password)
}

pub fn init() -> Result<(), DevVaultError> {
    let devvault_dir = PathBuf::from(".devvault");
    if !devvault_dir.exists() {
        fs::create_dir_all(&devvault_dir)?;
    }
    let config = Config::default();
    config.save()?;
    let password = prompt_password()?;
    let vault = Vault::new();
    save_vault(&vault, password.as_bytes())?;
    Ok(())
}

pub fn set(key: &str, value: &str) -> Result<(), DevVaultError> {
    let password = prompt_password()?;
    let mut vault = load_vault(password.as_bytes())?;
    let entry = VaultEntry {
        value: value.to_string(),
        added_at: Utc::now(),
        source: None,
    };
    vault.variables.insert(key.to_string(), entry);
    vault.updated_at = Utc::now();
    save_vault(&vault, password.as_bytes())?;
    Ok(())
}

pub fn get(key: &str) -> Result<String, DevVaultError> {
    let password = prompt_password()?;
    let vault = load_vault(password.as_bytes())?;
    vault
        .variables
        .get(key)
        .map(|entry| entry.value.clone())
        .ok_or_else(|| DevVaultError::VariableNotFound { key: key.to_string() })
}

pub fn list() -> Result<BTreeMap<String, VaultEntry>, DevVaultError> {
    let password = prompt_password()?;
    let vault = load_vault(password.as_bytes())?;
    Ok(vault.variables)
}

pub fn remove(key: &str) -> Result<(), DevVaultError> {
    let password = prompt_password()?;
    let mut vault = load_vault(password.as_bytes())?;
    if vault.variables.remove(key).is_none() {
        return Err(DevVaultError::VariableNotFound { key: key.to_string() });
    }
    vault.updated_at = Utc::now();
    save_vault(&vault, password.as_bytes())?;
    Ok(())
}

