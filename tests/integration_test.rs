use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_init_command() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Vault initialized successfully"));
}

#[test]
fn test_set_and_get_command() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Initialize vault
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .arg("init")
        .assert()
        .success();
    // Set a variable
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["set", "TEST_KEY=test_value"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Set TEST_KEY=test_value"));
    // Get the variable
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["get", "TEST_KEY"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test_value"));
}

#[test]
fn test_list_command() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Initialize vault
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .arg("init")
        .assert()
        .success();
    // Set some variables
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["set", "KEY1=value1"])
        .assert()
        .success();
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["set", "KEY2=value2"])
        .assert()
        .success();
    // List variables
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("KEY1=value1"))
        .stdout(predicate::str::contains("KEY2=value2"));
}

#[test]
fn test_remove_command() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Initialize vault
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .arg("init")
        .assert()
        .success();
    // Set a variable
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["set", "KEY_TO_REMOVE=value"])
        .assert()
        .success();
    // Remove the variable
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["remove", "KEY_TO_REMOVE"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed KEY_TO_REMOVE"));
    // Verify it's gone
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["get", "KEY_TO_REMOVE"])
        .assert()
        .failure();
}

#[test]
fn test_import_command() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Initialize vault
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .arg("init")
        .assert()
        .success();
    // Create a .env file
    let env_file = temp_path.join(".env");
    std::fs::write(&env_file, "IMPORTED_KEY=imported_value\n").unwrap();
    // Import the .env file
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["import", env_file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imported 1 variables"));
    // Verify the imported variable
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["get", "IMPORTED_KEY"])
        .assert()
        .success()
        .stdout(predicate::str::contains("imported_value"));
}

#[test]
fn test_export_command() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Initialize vault
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .arg("init")
        .assert()
        .success();
    // Set a variable
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .args(["set", "EXPORT_KEY=export_value"])
        .assert()
        .success();
    // Export variables
    Command::cargo_bin("devvault")
        .unwrap()
        .current_dir(temp_path)
        .arg("export")
        .assert()
        .success()
        .stdout(predicate::str::contains("EXPORT_KEY=export_value"));
}