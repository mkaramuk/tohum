use crate::{
    git::fetch_github_directory,
    metadata::{TemplateMetadata, parse_metadata_from_file},
};
use anyhow::{Error, Result};
use std::path::{Path, PathBuf};

// Default Store information
const DEFAULT_STORE_REPO_URL: &str = "https://github.com/mkaramuk/tohum";
const DEFAULT_STORE_REPO_BRANCH: &str = "main";
const DEFAULT_TEMPLATES_DIR_PATH: &str = "templates";

/// Downloads the given template from the given store and saves it
/// to the given output path.
pub fn fetch_template_from_store(
    store_repo_url: &str,
    store_repo_branch: &str,
    store_templates_dir_path: &str,
    template_path: &str,
    template_version: &str,
    output_path: &Path,
) -> Result<PathBuf, Error> {
    let folder_name = format!("{}_{}", template_path, template_version);
    let full_path = format!("{}/{}", store_templates_dir_path, folder_name);
    let output_dir = fetch_github_directory(
        store_repo_url,
        store_repo_branch,
        full_path.as_str(),
        output_path,
    )?;

    Ok(output_dir)
}

/// Same as `fetch_from_store` but uses the official (default) Store as the source.
pub fn fetch_template_from_default_store(
    path: &str,
    version: &str,
    output_path: &Path,
) -> Result<PathBuf, Error> {
    fetch_template_from_store(
        DEFAULT_STORE_REPO_URL,
        DEFAULT_STORE_REPO_BRANCH,
        DEFAULT_TEMPLATES_DIR_PATH,
        path,
        version,
        output_path,
    )
}

/// Reads the metadata file from the given template path.
/// Returns error if metadata.json is not found. It must
/// be placed under `<template path>/.tohum/metadata.json`.
pub fn read_metadata_from_template(template_path: &Path) -> Result<TemplateMetadata, Error> {
    let metadata_path = template_path.join(".tohum").join("metadata.json");

    if !metadata_path.exists() {
        return Err(Error::msg("\"metadata.json\" not found in the template"));
    }

    Ok(parse_metadata_from_file(&metadata_path)?)
}
