use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Error, Result};
use flate2::read::GzDecoder;
use reqwest::Client;
use tar::Archive;
use tempfile::env::temp_dir;

use crate::metadata::{Metadata, parse_metadata_from_file};

/// Fetches the given `.tar.gz` template file to the system's
/// temp directory. Returns the absolute path of the downloaded file.
pub async fn fetch_template(template_id: &str) -> Result<PathBuf, Error> {
    // Parse template name and group
    let [template_name, template_group]: [&str; 2] = template_id
        .split('@')
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| Error::msg("Invalid template identifier. Expected format: 'name@group'"))?;

    let file_name = format!("{}-{}.tar.gz", template_name, template_group);
    let absolute_path = temp_dir().join(&file_name);

    let client = Client::new();
    let request_string = format!(
        "https://raw.githubusercontent.com/mkaramuk/maker/main/templates/{}",
        file_name
    );
    let response = client
        .get(&request_string)
        .send()
        .await
        .map_err(|e| Error::msg(format!("Failed to download {e}")))?;

    if !response.status().is_success() {
        return Err(Error::msg(format!(
            "Failed to download file: {}",
            response.status(),
        )));
    }

    let bytes = response.bytes().await?;
    let mut file = File::create(&absolute_path)
        .map_err(|e| Error::msg(format!("Failed to create file at {:?}: {e}", absolute_path)))?;

    file.write_all(&bytes).map_err(|e| {
        Error::msg(format!(
            "Failed to write to file at {:?}: {e}",
            absolute_path
        ))
    })?;

    Ok(absolute_path)
}

/// Extracts the given `.tar.gz` template file into the `extract_path`
/// and returns a Metadata object that includes information about the
/// template. Returns error if the metadata.json is not available.
pub async fn extract_template(
    template_file_path: &Path,
    extract_path: &Path,
) -> Result<Metadata, Error> {
    let file = File::open(template_file_path)
        .map_err(|e| Error::msg(format!("Error while opening the template file: {e}")))?;

    let tar_gz = GzDecoder::new(file);
    let mut archive = Archive::new(tar_gz);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.clone();
        let extracted_entry_path = extract_path.join(&path);

        // Create the parent directory of the entry
        if let Some(parent) = extracted_entry_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Unpack the entry into the target path
        entry.unpack(&extracted_entry_path)?;
    }

    match read_metadata_from_template(&extract_path).await {
        Err(e) => {
            fs::remove_dir_all(&extract_path)?;
            return Err(e);
        }
        Ok(metadata) => return Ok(metadata),
    }
}

/// Reads the metadata file from the given template path.
/// Returns error if metadata.json is not found.
pub async fn read_metadata_from_template(template_path: &Path) -> Result<Metadata, Error> {
    let metadata_path = template_path.join("metadata.json");

    if !metadata_path.exists() {
        return Err(Error::msg("\"metadata.json\" not found in the template"));
    }

    Ok(parse_metadata_from_file(&metadata_path)?)
}
