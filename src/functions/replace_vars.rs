use anyhow::Result;
use regex::Regex;
use std::{collections::HashMap, fs};
use walkdir::WalkDir;

pub fn replace_placeholders_in_dir(
    target_dir: &str,
    variables: HashMap<&str, String>,
) -> Result<()> {
    for entry in WalkDir::new(target_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            let content = fs::read_to_string(path);

            if content.is_ok() {
                let mut content = content.unwrap();
                for (var_name, value) in &variables {
                    let pattern = format!(r"\{{\{{\s*{}\s*\}}\}}", var_name);
                    let re = Regex::new(&pattern)?;

                    content = re.replace_all(&content, value).to_string();
                }
                fs::write(path, content.as_bytes())?;
            }
        }
    }

    Ok(())
}
