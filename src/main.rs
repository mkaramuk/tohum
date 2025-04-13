mod functions;
use crate::functions::get_template_from_repo::get_template_from_repo;
use crate::functions::replace_vars::replace_placeholders_in_dir;
use std::path::Path;
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
                )),
        )
        .get_matches();

    match command.subcommand() {
        Some(("init", matches)) => {
            let template_id = matches.get_one::<String>("template-id").unwrap();
            let project_name = match matches.get_one::<String>("project-name") {
                Some(name) => name.to_string(),
                None => template_id.replace("@", "-"),
            };

            match get_template_from_repo(template_id, Some(project_name.as_str())).await {
                Err(err) => {
                    eprintln!("Error: {}", err)
                }
                Ok(_) => {
                    let target_dir = Path::new(project_name.as_str());
                    replace_placeholders_in_dir(target_dir.to_str().unwrap(), &project_name)
                        .expect("TODO: panic message");
                }
            }

            // TODO: Implement rest of the init command
        }
        _ => unreachable!(),
    }

    Ok(())
}
