use anyhow::{Context, Error};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

pub fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), Error> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !dst.exists() {
        fs::create_dir_all(dst)
            .with_context(|| format!("Failed to create directory: {}", dst.display()))?;
    }

    for entry in
        fs::read_dir(src).with_context(|| format!("Failed to read directory: {}", src.display()))?
    {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).with_context(|| {
                format!("Failed to copy file from {:?} to {:?}", src_path, dst_path)
            })?;
        }
    }

    Ok(())
}

/// Reads first 1 KB of the given file to detect
/// whether or not it is a binary or plain text file
pub fn is_binary(path: impl AsRef<Path>) -> bool {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return true,
    };

    let mut buffer = [0u8; 1024]; // Sadece ilk 1 KB
    let n = file.read(&mut buffer).unwrap_or(0);

    // İlk 1024 byte içinde Null Byte varsa büyük ihtimalle binary'dir
    buffer[..n].contains(&0)
}
