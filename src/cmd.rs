use clap::{Arg, ArgAction, Command};

use crate::constants::{DEFAULT_SILO, DEFAULT_SILO_BRANCH};

pub const CMD_PLANT: &str = "plant";
pub const CMD_SILO: &str = "silo";
pub const CMD_SILO_LIST: &str = "list";
pub const CMD_SILO_INSPECT: &str = "inspect";

pub const ARGS_SILO_URL: &str = "silo-url";
pub const ARGS_SILO_BRANCH: &str = "silo-branch";
pub const ARGS_SEED: &str = "seed";
pub const ARGS_PROJECT_NAME: &str = "project-name";
pub const ARGS_VARIABLES: &str = "var";
pub const ARGS_PATH: &str = "path";
pub const ARGS_FORCE: &str = "force";

pub fn build_cmd() -> Command {
    let silo_source_args = build_args_silo_source();

    Command::new("tohum")
        .about("Project provisioning tool")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(&silo_source_args[0])
        .arg(&silo_source_args[1])
        .subcommand(build_sub_cmd_plant())
        .subcommand(build_sub_cmd_silo())
}

pub fn build_args_silo_source() -> Vec<Arg> {
    vec![
        Arg::new(ARGS_SILO_URL)
            .short('s')
            .long("silo")
            .action(ArgAction::Set)
            .default_value(DEFAULT_SILO)
            // .hide_default_value(true)
            .global(true)
            .help("Git URL of the silo"),
        Arg::new(ARGS_SILO_BRANCH)
            .short('b')
            .long("branch")
            .action(ArgAction::Set)
            // .hide_default_value(true)
            .global(true)
            .default_value(DEFAULT_SILO_BRANCH)
            .help("Git branch of the silo"),
    ]
}

pub fn build_sub_cmd_plant() -> Command {
    Command::new(CMD_PLANT)
        .about("Makes a new project from the chosen")
        .arg(
            Arg::new(ARGS_SEED)
                .num_args(1)
                .required(true)
                .action(ArgAction::Set)
                .help("Name of the seed in the silo"),
        )
        .arg(
            Arg::new(ARGS_PROJECT_NAME)
                .num_args(1)
                .required(true)
                .action(ArgAction::Set)
                .help("Project name"),
        )
        .arg(
            Arg::new(ARGS_VARIABLES)
                .short('v')
                .long("var")
                .help("Defines a variable for the template rendering.")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new(ARGS_PATH)
                .short('p')
                .long("path")
                .num_args(1)
                .default_value(".")
                .action(ArgAction::Set)
                .help("Path of the project. If not given, uses current directory."),
        )
        .arg(
            Arg::new(ARGS_FORCE)
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Overwrite if the given path already exists."),
        )
}

pub fn build_sub_cmd_silo() -> Command {
    Command::new(CMD_SILO)
        .about("Silo management commands.")
        .subcommand(
            Command::new(CMD_SILO_LIST)
                .alias("ls")
                .about("Lists all the available seeds from the silo."),
        )
        .subcommand(
            Command::new(CMD_SILO_INSPECT)
                .alias("details")
                .about("Shows details of a seed")
                .arg(
                    Arg::new(ARGS_SEED)
                        .num_args(1)
                        .required(true)
                        .action(ArgAction::Set)
                        .help("Name of the seed in the silo"),
                ),
        )
}
