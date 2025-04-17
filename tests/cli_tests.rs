use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cli_with_target_path() {
    let temp_dir = tempdir().unwrap(); // Create a temporary directory
    let target_path = temp_dir.path().join("test_project"); // Define the target path

    // Run the CLI command with the `-p` flag
    Command::cargo_bin("maker")
        .unwrap()
        .arg("init")
        .arg("go@cli")
        .arg("-p")
        .arg(target_path.to_str().unwrap())
        .assert()
        .success(); // Assert that the command runs successfully

    // Verify that the target path was created
    assert!(target_path.exists());
    assert!(target_path.is_dir());

    // Clean up
    fs::remove_dir_all(target_path).unwrap();
}
