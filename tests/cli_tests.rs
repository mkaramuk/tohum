use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cli_with_target_path() {
    let temp_dir = tempdir().unwrap(); // Create a temporary directory
    let target_path = temp_dir.path().join("test_project"); // Define the target path

    // Run the CLI command with the `-p` flag
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("init")
        .arg("cli@go")
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

#[test]
fn test_cli_with_project_name_and_target_path() {
    let temp_dir = tempdir().unwrap(); // Create a temporary directory
    let target_path = temp_dir.path().join("custom_project"); // Define the target path
    let project_name = "custom_project";

    // Run the CLI command with both `--project-name` and `-p` flags
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("init")
        .arg("cli@go")
        .arg(project_name)
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

#[test]
fn test_cli_with_missing_template_id() {
    let temp_dir = tempdir().unwrap(); // Create a temporary directory
    let target_path = temp_dir.path().join("test_project"); // Define the target path

    // Run the CLI command without providing the required `--template-id`
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("init")
        .arg("-p")
        .arg(target_path.to_str().unwrap())
        .assert()
        .failure(); // Assert that the command fails
}

#[test]
fn test_cli_with_invalid_command() {
    // Run the CLI command with an invalid subcommand
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("invalid_command")
        .assert()
        .failure(); // Assert that the command fails
}

#[test]
fn test_cli_help_output() {
    // Run the CLI command with the `--help` flag
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("--help")
        .assert()
        .success() // Assert that the command runs successfully
        .stdout(predicates::str::contains("Project provisioning tool")); // Verify help output contains expected text
}

#[test]
fn test_cli_with_invalid_template_id_format() {
    // Run the CLI command with an invalid template ID format
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("init")
        .arg("cli@") // Invalid template ID
        .assert()
        .failure(); // Assert that the command fails
}

#[test]
fn test_cli_with_valid_template_id() {
    let temp_dir = tempdir().unwrap(); // Create a temporary directory
    let target_path = temp_dir.path().join("test_project"); // Define the target path

    // Run the CLI command with a valid template ID
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("init")
        .arg("cli@go") // Valid template ID
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

#[test]
fn test_cli_with_missing_template_id_and_target_path() {
    // Run the CLI command with missing template ID and `-p` flag
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("init")
        .arg("-p") // Missing template ID
        .assert()
        .failure(); // Assert that the command fails
}

#[test]
fn test_cli_with_missing_template_id_and_invalid_target_path() {
    // Run the CLI command with missing template ID and invalid `-p` value
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("init")
        .arg("-p")
        .arg("path") // Invalid target path
        .assert()
        .failure(); // Assert that the command fails
}

#[test]
fn test_cli_with_invalid_template_id_and_target_path() {
    let temp_dir = tempdir().unwrap(); // Create a temporary directory
    let target_path = temp_dir.path().join("test_project"); // Define the target path

    // Run the CLI command with an invalid template ID and valid `-p` flag
    Command::cargo_bin("tohum")
        .unwrap()
        .arg("init")
        .arg("go") // Invalid template ID
        .arg("-p")
        .arg(target_path.to_str().unwrap())
        .assert()
        .failure(); // Assert that the command fails
}
