use regex::Regex;
use walkdir::WalkDir;
use std::fs;

pub fn replace_placeholders_in_dir(target_dir: &str, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"\{\{\s*project_name\s*}}")?;

    for entry in WalkDir::new(target_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            let content = fs::read_to_string(path);

            if let Ok(content) = content {
                let replaced = re.replace_all(&content, project_name);
                fs::write(path, replaced.as_bytes())?;
            }
        }
    }

    Ok(())
}
