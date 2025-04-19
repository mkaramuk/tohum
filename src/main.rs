mod functions;
use crate::functions::get_template_from_repo::get_template_from_repo;
use crate::functions::replace_vars::replace_placeholders_in_dir;
use std::{collections::HashMap, path::Path};
mod metadata;

use anyhow::Result;
use clap::{Arg, ArgAction, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let command = Command::new("maker")
        .about("project provisioning tool")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .alias("create")
                .about("Provisions a new project from a template definition")
                .arg(
                    Arg::new("template-id")
                        .num_args(1)
                        .required(true)
                        .action(ArgAction::Set)
                        .help("Template identifier. Should be in 'name@group' format"),
                )
                .arg(Arg::new("project-name").action(ArgAction::Set).help(
                    "Name of the project. Uses name of the template by default if it is not given",
                ))
                .arg(
                    Arg::new("target-path")
                        .short('p') // Short flag for the argument
                        .long("target-path") // Long flag for the argument
                        .num_args(1) // Specifies that the argument takes one value
                        .action(ArgAction::Set) // Sets the value of the argument
                        .help("Target path of the project. If not given, uses current directory.") // Help message for the argument
                        .visible_alias("target-path"), // Alias to make the argument more discoverable
                ),
        )
        .get_matches();

    match command.subcommand() {
        Some(("init", matches)) => {
            let template_id = matches.get_one::<String>("template-id").unwrap();
            let project_name = match matches.get_one::<String>("project-name") {
                Some(name) => name.to_string(),
                None => template_id.replace("@", "-"),
            };

            let target_path = match matches.get_one::<String>("target-path") {
                Some(path) => path.to_string(), // Use the provided target path
                None => project_name.clone(), // Use the project_name which points to the current dir
            };

            match get_template_from_repo(template_id, Some(target_path.as_str())).await {
                Err(err) => {
                    eprintln!("Error: {err}")
                }
                Ok(_) => {
                    let target_dir = Path::new(&target_path);

                    let data = metadata::parse_metadata_from_file(&format!(
                        "{}/{}",
                        target_dir.to_str().unwrap(),
                        "metadata.json"
                    ))?;
                    let mut variables: HashMap<&str, String> = HashMap::new();

                    variables.insert("project_name", project_name.clone());

                    for (var_name, value) in &data.variables {
                        // TODO: Find values of the variables from command line arguments
                        variables.insert(var_name, String::from(value.default.to_string()));
                    }

                    replace_placeholders_in_dir(target_dir.to_str().unwrap(), variables)?;
                }
            }

            // TODO: Implement rest of the init command
        }
        _ => unreachable!(),
    }

    Ok(())
}
