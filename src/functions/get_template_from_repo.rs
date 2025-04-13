use flate2::read::GzDecoder;
use reqwest::Client;
use std::fs;
use std::path::{Path, PathBuf};
use tar::Archive;

pub async fn get_template_from_repo(
    template_id: &str,
    folder_name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let template_name: Vec<&str> = template_id.split('@').collect();

    if template_name.len() != 2 {
        return Err("❌ Invalid template identifier format. Use something like 'react@ts'".into());
    }

    let filename = format!("{}-{}.tar.gz", template_name[0], template_name[1]);
    let request_string = format!(
        "https://raw.githubusercontent.com/mkaramuk/maker/main/templates/{}",
        filename
    );

    let client = Client::new();
    let response = client.get(&request_string).send().await?;

    if !response.status().is_success() {
        return Err(format!("❌ Failed to download file: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;
    let tar_gz = GzDecoder::new(&bytes[..]);
    let mut archive = Archive::new(tar_gz);

    let target_dir = Path::new(folder_name.unwrap_or("template"));

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let stripped_path = path
            .iter()
            .skip(1) // İlk klasörü atla (orijinal arşivdeki klasör ismini)
            .collect::<PathBuf>();

        let dest_path = target_dir.join(stripped_path);

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        entry.unpack(&dest_path)?;
    }

    println!("✅ Template unpacked into: {:?}", target_dir);
    Ok(())
}
