use devvault::scanner;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_scan_aws_key() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Create a file with AWS key
    let test_file = temp_path.join("test.txt");
    fs::write(&test_file, "AKIAIOSFODNN7EXAMPLE").unwrap();
    let results = scanner::scan(Some(temp_path)).unwrap();
    assert!(!results.is_empty());
    assert!(results[0].pattern_name.contains("AWS"));
}

#[test]
fn test_scan_github_token() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Create a file with GitHub token (needs to be at least 36 chars after ghp_)
    let test_file = temp_path.join("test.txt");
    fs::write(&test_file, "ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghij").unwrap();
    let results = scanner::scan(Some(temp_path)).unwrap();
    assert!(!results.is_empty());
    assert!(results[0].pattern_name.contains("GitHub"));
}

#[test]
fn test_scan_private_key() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Create a file with private key
    let test_file = temp_path.join("test.txt");
    fs::write(&test_file, "-----BEGIN RSA PRIVATE KEY-----").unwrap();
    let results = scanner::scan(Some(temp_path)).unwrap();
    assert!(!results.is_empty());
    assert!(results[0].pattern_name.contains("Private Key"));
}

#[test]
fn test_scan_jwt_token() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Create a file with JWT token
    let test_file = temp_path.join("test.txt");
    fs::write(&test_file, "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c").unwrap();
    let results = scanner::scan(Some(temp_path)).unwrap();
    assert!(!results.is_empty());
    assert!(results[0].pattern_name.contains("JWT"));
}

#[test]
fn test_scan_database_url() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Create a file with database URL
    let test_file = temp_path.join("test.txt");
    fs::write(&test_file, "mysql://user:password@localhost/db").unwrap();
    let results = scanner::scan(Some(temp_path)).unwrap();
    assert!(!results.is_empty());
    assert!(results[0].pattern_name.contains("Database"));
}

#[test]
fn test_scan_no_secrets() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Create a file with no secrets
    let test_file = temp_path.join("test.txt");
    fs::write(&test_file, "This is a normal file with no secrets.").unwrap();
    let results = scanner::scan(Some(temp_path)).unwrap();
    assert!(results.is_empty());
}