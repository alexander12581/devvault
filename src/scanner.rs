use regex::Regex;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;
use crate::error::DevVaultError;

#[derive(Debug)]
pub struct ScanResult {
    pub file: PathBuf,
    pub line: usize,
    pub matched: String,
    pub pattern_name: String,
}

pub fn scan(path: Option<&Path>) -> Result<Vec<ScanResult>, DevVaultError> {
    let config = Config::load()?;
    let scan_path = path.unwrap_or(Path::new("."));
    let mut results = Vec::new();
    let patterns: Vec<(String, Regex)> = config
        .scan_patterns
        .iter()
        .map(|p| {
            let name = pattern_name(p);
            let regex = Regex::new(p)?;
            Ok((name, regex))
        })
        .collect::<Result<Vec<_>, regex::Error>>()?;
    for entry in WalkDir::new(scan_path)
        .into_iter()
        .filter_entry(|e| !is_excluded(e.path(), &config.exclude_patterns))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            if let Ok(content) = std::fs::read_to_string(file_path) {
                for (line_num, line) in content.lines().enumerate() {
                    for (name, regex) in &patterns {
                        if regex.is_match(line) {
                            results.push(ScanResult {
                                file: file_path.to_path_buf(),
                                line: line_num + 1,
                                matched: line.trim().to_string(),
                                pattern_name: name.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(results)
}

fn pattern_name(pattern: &str) -> String {
    if pattern.contains("AKIA") {
        "AWS Access Key".to_string()
    } else if pattern.contains("gh[pousr]_") {
        "GitHub Token".to_string()
    } else if pattern.contains("PRIVATE KEY") {
        "Private Key".to_string()
    } else if pattern.contains("eyJ") {
        "JWT Token".to_string()
    } else if pattern.contains("mysql") || pattern.contains("postgres") || pattern.contains("mongodb") {
        "Database URL".to_string()
    } else if pattern.contains("[Aa][Pp][Ii]") {
        "Generic API Key".to_string()
    } else if pattern.contains("[Ss]ecret") {
        "Generic Secret".to_string()
    } else {
        "Unknown Pattern".to_string()
    }
}

fn is_excluded(path: &Path, exclude_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    for pattern in exclude_patterns {
        if pattern.contains('*') {
            // Simple glob matching for file extensions
            if pattern.starts_with("*.") {
                let ext = &pattern[2..];
                if path_str.ends_with(ext) {
                    return true;
                }
            }
        } else if path_str.contains(pattern.as_str()) {
            return true;
        }
    }
    false
}