mod cmd;
mod functions;
mod metadata;
mod template;

use functions::replace_vars::replace_placeholders_in_dir;
use std::{collections::HashMap, fs, path::Path};
use template::{extract_template, fetch_template};

use anyhow::{Error, Result};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let command = cmd::build_cmd().get_matches();
    match command.subcommand() {
        Some(("init", matches)) => {
            let mut variables: HashMap<&str, String> = HashMap::new();

            // Parse and add the variables that passed by the user
            if let Some(defined_vars) = matches.get_many::<String>("var") {
                for var in defined_vars {
                    if let Some((key, value)) = var.split_once('=') {
                        variables.insert(key, value.to_string());
                    } else {
                        return Err(Error::msg(format!(
                            "Invalid variable definition: {} (expected key=value)",
                            var
                        )));
                    }
                }
            }

            let template_id = matches.get_one::<String>("template-id").unwrap();
            let project_name = match matches.get_one::<String>("project-name") {
                // Use the given project name if it is given
                Some(name) => name.to_string(),

                // Otherwise check if it is given as a variable
                None => match variables.get("project_name") {
                    Some(project_name_var) => project_name_var.clone(),

                    // Project name is not given, use the template identifier as it
                    None => template_id.replace("@", "-"),
                },
            };

            if !variables.contains_key("project_name") {
                variables.insert("project_name", project_name.clone());
            }

            let target_path = match matches.get_one::<String>("target-path") {
                // Use the provided target path
                Some(path) => Path::new(path),

                // Use the project_name which points to the current dir
                None => Path::new(&project_name),
            };

            // NOTE: Maybe we can implement a workflow where we ask user to whether delete that existing directory.
            if target_path.exists() {
                return Err(Error::msg(format!(
                    "target directory ({}) is already exist",
                    target_path.display()
                )));
            }

            let template_file_path = fetch_template(template_id).await?;
            let metadata = extract_template(&template_file_path, &target_path).await?;

            variables.insert("author", metadata.author.name);

            // Check if all the necessary variables are presented
            for (var_name, info) in &metadata.variables {
                // If the variable value was given from the CLI flags
                // it is already presented in the variables list.
                if variables.contains_key(var_name.as_str()) {
                    continue;
                }

                // Find the default value of the variable
                let value = match &info.default {
                    Some(default_value) => default_value.as_str().unwrap().to_string(),
                    None => {
                        return Err(Error::msg(format!("Missing variable \"{}\"", var_name)));
                    }
                };

                variables.insert(var_name, value);
            }

            // Apply template engine (aka replace variables from the template files)
            replace_placeholders_in_dir(target_path.to_str().unwrap(), variables)?;

            fs::remove_file(target_path.join("metadata.json"))?;

            println!(
                "Project {} successfully initialized at {}",
                project_name,
                target_path.display()
            );
        }
        _ => unreachable!(),
    }

    Ok(())
}
