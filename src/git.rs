use anyhow::Error;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

use crate::process::check_exit_status;

pub fn git_sparse_clone(
    url: impl AsRef<str>,
    branch: impl AsRef<str>,
    glob_pattern: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<(), Error> {
    let output = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg("--filter=blob:none")
        .arg("--no-checkout")
        .arg("--single-branch")
        .arg(format!("--branch={}", branch.as_ref()))
        .arg(url.as_ref())
        .arg(output_path.as_ref())
        .output()?;
    check_exit_status(output)?;

    let output = Command::new("git")
        .arg("sparse-checkout")
        .arg("init")
        .arg("--no-cone")
        .current_dir(&output_path)
        .output()?;
    check_exit_status(output)?;

    let output = Command::new("git")
        .arg("sparse-checkout")
        .arg("set")
        .arg(glob_pattern.as_ref())
        .current_dir(&output_path)
        .output()?;
    check_exit_status(output)?;

    let output = Command::new("git")
        .arg("checkout")
        .current_dir(&output_path)
        .output()?;
    check_exit_status(output)?;

    Ok(())
}
