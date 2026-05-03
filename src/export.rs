use std::fs;
use std::path::Path;

use crate::error::DevVaultError;
use crate::vault;

pub fn import_env(file: &Path) -> Result<usize, DevVaultError> {
    let content = fs::read_to_string(file)?;
    let mut count = 0;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = parse_env_line(line) {
            vault::set(&key, &value)?;
            count += 1;
        }
    }
    Ok(count)
}

pub fn export_env(format: &str) -> Result<String, DevVaultError> {
    match format {
        "env" => {
            let vars = vault::list()?;
            let mut output = String::new();
            for (key, entry) in vars {
                output.push_str(&format!("{}={}\n", key, entry.value));
            }
            Ok(output)
        }
        _ => Err(DevVaultError::Other(anyhow::anyhow!(
            "Unsupported format: {}",
            format
        ))),
    }
}

fn parse_env_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let mut parts = line.splitn(2, '=');
    let key = parts.next()?.trim().to_string();
    let value = parts.next()?.trim().to_string();
    // Remove surrounding quotes if present
    let value = if (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        value[1..value.len() - 1].to_string()
    } else {
        value
    };
    Some((key, value))
}