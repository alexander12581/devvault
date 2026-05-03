use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::error::DevVaultError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub vault_path: PathBuf,
    pub scan_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            vault_path: PathBuf::from("vault.enc"),
            scan_patterns: vec![
                r"AKIA[0-9A-Z]{16}".to_string(),
                r"gh[pousr]_[A-Za-z0-9_]{36,}".to_string(),
                r#"[Aa][Pp][Ii][-_]?[Kk][Ee][Yy].*['"][0-9a-zA-Z]{32,}['"]"#.to_string(),
                r"-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----".to_string(),
                r"eyJ[A-Za-z0-9-_]+\.eyJ[A-Za-z0-9-_]+\.[A-Za-z0-9-_.+/=]+".to_string(),
                r#"[Ss]ecret[-_]?[Kk]ey.*['"][0-9a-zA-Z]{16,}['"]"#.to_string(),
                r"(mysql|postgres|mongodb)://[^\s]+".to_string(),
            ],
            exclude_patterns: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "target".to_string(),
                "*.lock".to_string(),
            ],
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, DevVaultError> {
        let config_path = PathBuf::from(".devvault").join("config.toml");
        if !config_path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), DevVaultError> {
        let config_path = PathBuf::from(".devvault").join("config.toml");
        let content = toml::to_string_pretty(self)
            .map_err(|e| DevVaultError::Other(anyhow::anyhow!("TOML serialization error: {}", e)))?;
        fs::write(&config_path, content)?;
        Ok(())
    }
}