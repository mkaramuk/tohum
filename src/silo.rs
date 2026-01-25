use anyhow::{Context, Error, Result};
use colored::Colorize;
use std::{ffi::OsStr, fs::File, io::BufReader, path::Path};
use tempfile::TempDir;
use walkdir::WalkDir;

use crate::{
    constants::{TOHUMRC_FILENAME, TOHUMRC_GLOB_PATTERN},
    git::git_sparse_clone,
    seed::Seed,
};

pub type Silo = Vec<Seed>;

pub fn fetch_silo_from_git(
    git_url: impl AsRef<str>,
    branch: impl AsRef<str>,
) -> Result<Silo, Error> {
    let temp_dir = TempDir::new().context("Failed to create temporary directory")?;
    let temp_path = temp_dir.path().join("repo");

    git_sparse_clone(git_url, branch, TOHUMRC_GLOB_PATTERN, &temp_path)?;

    let silo = read_silo(&temp_path)?;
    Ok(silo)
}

pub fn read_silo(path: impl AsRef<Path>) -> Result<Vec<Seed>, Error> {
    let mut silo: Silo = vec![];

    let walker = WalkDir::new(&path)
        .into_iter()
        .filter_entry(|e| e.file_name().to_str().map(|s| s != ".git").unwrap_or(true))
        .filter_map(Result::ok);

    for entry in walker {
        let entry_path = entry.path();

        if !entry_path.is_file()
            || entry_path.file_name().unwrap_or(OsStr::new("")) != TOHUMRC_FILENAME
        {
            continue;
        }

        let file = File::open(entry_path)?;
        let reader = BufReader::new(file);
        let mut seed: Seed = match serde_json::from_reader(reader) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "{}: Invalid seed {}: {}",
                    "Warning".yellow(),
                    entry_path.display(),
                    e
                );
                continue;
            }
        };

        // `.repo_path` points to the seed path inside the repo.
        // e.g `/tmp/temp-dir/repo-root/silo/seed` -> `silo/seed`
        seed.repo_path = entry_path
            .strip_prefix(&path)?
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        silo.push(seed);
    }

    Ok(silo)
}
