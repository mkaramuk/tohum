mod cmd;
mod functions;
mod metadata;
use functions::get_template_from_repo::get_template_from_repo;
use functions::replace_vars::replace_placeholders_in_dir;
use std::{collections::HashMap, path::Path};

use anyhow::{Error, Result};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let command = cmd::build_cmd().get_matches();
    match command.subcommand() {
        Some(("init", matches)) => {
            let template_id = matches.get_one::<String>("template-id").unwrap();
            let project_name = match matches.get_one::<String>("project-name") {
                Some(name) => name.to_string(),        // Use the given project name
                None => template_id.replace("@", "-"), // use template identifier as project name
            };

            let target_path = match matches.get_one::<String>("target-path") {
                Some(path) => path.to_string(), // Use the provided target path
                None => project_name.clone(), // Use the project_name which points to the current dir
            };
            let target_path = Path::new(&target_path);

            if target_path.exists() {
                return Err(Error::msg(format!(
                    "target directory ({}) is already exist",
                    target_path.to_str().unwrap()
                )));
            }

            match get_template_from_repo(template_id, Some(target_path.to_str().unwrap())).await {
                Err(err) => {
                    return Err(err);
                }
                Ok(_) => {
                    let mut variables: HashMap<&str, String> = HashMap::new();
                    let data = metadata::parse_metadata_from_file(&format!(
                        "{}/{}",
                        target_path.to_str().unwrap(),
                        "metadata.json"
                    ))?;

                    variables.insert("project_name", project_name.clone());

                    for (var_name, value) in &data.variables {
                        // TODO: Find values of the variables from command line arguments
                        variables.insert(var_name, String::from(value.default.to_string()));
                    }

                    replace_placeholders_in_dir(target_path.to_str().unwrap(), variables)?;
                }
            }

            // TODO: Implement rest of the init command
        }
        _ => unreachable!(),
    }

    Ok(())
}
