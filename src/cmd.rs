use clap::{Arg, ArgAction, Command};

pub fn build_cmd() -> Command {
    Command::new("tohum")
        .about("Project provisioning tool")
        .version(env!("CARGO_PKG_VERSION"))
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
                .arg(
                    Arg::new("project-name")
                        .num_args(1)
                        .action(ArgAction::Set)
                        .help("Name of the project, uses template identifier by default"),
                )
                .arg(
                    Arg::new("var")
                        .short('v')
                        .long("var")
                        .help("Defines a variable that will be used by the template")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("target-path")
                        .short('p') // Short flag for the argument
                        .long("target-path") // Long flag for the argument
                        .num_args(1) // Specifies that the argument takes one value
                        .action(ArgAction::Set) // Sets the value of the argument
                        .help("Target path of the project. If not given, uses current directory."), // Help message for the argument
                ),
        )
}
