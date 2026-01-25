use std::{fs, path::PathBuf};

use anyhow::{Error, Ok};

use crate::constants::CONFIG_DIR_NAME;

pub fn config_path() -> Result<PathBuf, Error> {
    let os_config_dir = dirs::config_local_dir().unwrap();
    let config_dir = os_config_dir.join(CONFIG_DIR_NAME);

    if !config_dir.is_dir() {
        // TODO: Add custom context to mention that the error was coming from config dir make function
        fs::create_dir(&config_dir)?;
    }

    Ok(config_dir)
}
