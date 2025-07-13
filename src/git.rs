use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use tempfile::TempDir;

/// Fetches a directory from a GitHub repository using git CLI with sparse checkout
pub fn fetch_github_directory(
    repo_url: &str,
    branch: &str,
    target_path: &str,
    output_path: &Path,
) -> Result<PathBuf, Error> {
    let temp_dir = TempDir::new().context("Failed to create temporary directory")?;
    let temp_path = temp_dir.path();

    // Initialize an empty repository
    let output = Command::new("git")
        .args(["init"])
        .current_dir(temp_path)
        .output()?;

    check_exit_status("git init", output)?;

    // Add remote origin
    let output = Command::new("git")
        .args(["remote", "add", "origin", repo_url])
        .current_dir(temp_path)
        .output()?;

    check_exit_status("git remote add origin ...", output)?;

    // Enable sparse checkout
    let output = Command::new("git")
        .args(["config", "core.sparseCheckout", "true"])
        .current_dir(temp_path)
        .output()?;

    check_exit_status("git config core.sparseCheckout true", output)?;

    // Init sparse checkout
    let output = Command::new("git")
        .args(["sparse-checkout", "init"])
        .current_dir(temp_path)
        .output()?;

    check_exit_status("git sparse-checkout init", output)?;

    // Set sparse checkout directory
    let output = Command::new("git")
        .args(["sparse-checkout", "set", target_path])
        .current_dir(temp_path)
        .output()?;

    check_exit_status("git sparse-checkout set ...", output)?;

    // Pull the directory
    let output = Command::new("git")
        .args(["pull", "origin", branch])
        .current_dir(temp_path)
        .output()?;

    check_exit_status(format!("git pull origin {branch}").as_str(), output)?;

    // Copy to the target
    fs::rename(temp_path.join(target_path), output_path)?;

    Ok(output_path.to_path_buf())
}

fn check_exit_status(cmd: &str, output: Output) -> Result<(), Error> {
    if !output.status.success() {
        return Err(Error::msg(format!(
            "\"{cmd}\" failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(())
}
