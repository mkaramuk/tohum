mod cmd;
mod functions;
mod git;
mod metadata;
mod progress;
mod store;
mod template;

use anyhow::{Error, Result};
use functions::replace_vars::replace_placeholders_in_dir;
use names::Generator;
use std::{collections::HashMap, path::Path, process::Command};

use crate::{
    store::{
        DEFAULT_STORE_JSON_URL, fetch_default_store_template_list, get_latest_version_of_template,
    },
    template::{fetch_template_from_default_store, read_metadata_from_template},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let command = cmd::build_cmd();
    let cmd_matches = command.clone().get_matches();

    match cmd_matches.subcommand() {
        Some(("store", store_cmd_matches)) => match store_cmd_matches.subcommand() {
            Some(("list", _)) => {
                let templates = store::fetch_default_store_template_list().await?;
                let mut paths: Vec<String> = templates.keys().cloned().collect();

                // Sort the templates in alphabetical order
                paths.sort_by(|a, b| a.cmp(b));

                println!("\n📦 Available templates from Store {DEFAULT_STORE_JSON_URL}:\n");
                for path in paths {
                    let template = templates.get(&path).unwrap();

                    if template.description.is_some() {
                        println!(
                            "  📝 {}\n     {}\n",
                            path,
                            template
                                .description
                                .clone()
                                .unwrap_or(String::from("<Description not available>"))
                        );
                    }
                }
            }
            _ => {
                let mut store_cmd = command.find_subcommand("store").unwrap().clone();
                store_cmd.print_help().unwrap();
            }
        },

        Some(("init", matches)) => {
            let mut variables: HashMap<String, String> = HashMap::new();

            // Parse and add the variables that passed from the args
            if let Some(defined_vars) = matches.get_many::<String>("var") {
                for var in defined_vars {
                    if let Some((key, value)) = var.split_once('=') {
                        variables.insert(key.to_string(), value.to_string());
                    } else {
                        return Err(Error::msg(format!(
                            "Invalid variable definition: {} (expected key=value)",
                            var
                        )));
                    }
                }
            }

            let template_source = matches.get_one::<String>("template-source").unwrap();
            let project_name = match matches.get_one::<String>("project-name") {
                // Use the given project name if it is presented
                Some(name) => name.to_string(),

                // Otherwise check if it is given as a variable
                None => match variables.get("project_name") {
                    Some(project_name_var) => project_name_var.clone(),

                    // Fallback to random name
                    None => {
                        let mut generator = Generator::default();
                        generator.next().unwrap()
                    }
                },
            };

            // If user didn't pass a `project_name` variable (to customize it)
            // we need to pass the project name that we have defined by default
            // so in the next step, this variable can be replaced by the template engine.
            if !variables.contains_key("project_name") {
                variables.insert(String::from("project_name"), project_name.clone());
            }

            let output_path = match matches.get_one::<String>("output") {
                // Use the provided target path
                Some(path) => Path::new(path),

                // Use the project_name which points to the current dir
                None => Path::new(&project_name),
            };

            // Check if output directory exists and handle overwrite
            if output_path.exists() {
                let should_overwrite = if matches.get_flag("overwrite") {
                    true
                } else {
                    print!(
                        "Output directory ({}) already exists. Do you want to overwrite it? [y/N]: ",
                        output_path.display()
                    );
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().to_lowercase();

                    input == "y" || input == "yes"
                };

                if !should_overwrite {
                    return Err(Error::msg("Operation cancelled by user"));
                }

                std::fs::remove_dir_all(&output_path).map_err(|e| {
                    Error::msg(format!("Failed to remove existing directory: {}", e))
                })?;
            }

            // TODO: Add functionality to fetch templates from different sources (e.g github, local file system, another stores)

            let spinner = progress::create_spinner("Fetching the store template list");
            let templates = fetch_default_store_template_list().await?;
            let template = templates.get(template_source).expect(
                format!(
                    "Template path \"{}\" not found in the default Store",
                    template_source
                )
                .as_str(),
            );

            spinner.set_message("Fetching the template");

            // TODO: parse version from `template_source`
            let latest_version = get_latest_version_of_template(template)?;
            let template_output = fetch_template_from_default_store(
                &template_source,
                latest_version.to_string().as_str(),
                &output_path,
            )?;

            let metadata = read_metadata_from_template(&template_output)?;

            // TODO: Place some metadata into the variables so they can be used in the template engine

            // Check if all the necessary variables by template
            // are presented in the CLI
            if metadata.variables.is_some() {
                let metadata_variables = metadata.variables.unwrap();
                for (var_name, info) in metadata_variables {
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
            }

            spinner.set_message("Preparing project");

            // Apply template engine (aka replace variables from the template files)
            replace_placeholders_in_dir(output_path.to_str().unwrap(), variables)?;

            // Init the git repository in the output
            Command::new("git")
                .args(["init"])
                .current_dir(output_path.to_str().unwrap())
                .output()?;

            spinner.finish_and_clear();

            println!("🎉 Project '{project_name}' successfully initialized!");
            println!("📝 Template: {template_source}");
            println!("📁 Location: {}", output_path.display());
        }
        _ => unreachable!(),
    }

    Ok(())
}
