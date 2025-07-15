use clap::{Arg, ArgAction, Command};

pub fn build_cmd() -> Command {
    Command::new("tohum")
        .about("Project provisioning tool")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .alias("new")
                .about("Provisions a new project from a template source.")
                .arg(
                    Arg::new("template-source")
                        .num_args(1)
                        .required(true)
                        .action(ArgAction::Set)
                        .help("Template source identifier. Currently only supports default Store"),
                )
                .arg(
                    Arg::new("project-name")
                        .num_args(1)
                        .action(ArgAction::Set)
                        .help("Name of the project. Uses a random name if not given."),
                )
                .arg(
                    Arg::new("var")
                        .short('v')
                        .long("var")
                        .help(
                            "Defines a variable that will be used while configuring the template.",
                        )
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .num_args(1)
                        .action(ArgAction::Set)
                        .help(
                            "Output path of the new project. If not given, uses current directory.",
                        ),
                )
                .arg(
                    Arg::new("overwrite")
                        .long("overwrite")
                        .action(ArgAction::SetTrue)
                        .help("Automatically overwrite existing directory without prompting"),
                ),
        )
        .subcommand(Command::new("store").about("Store commands.").subcommand(
            Command::new("list").about("Lists all the available templates from the default Store."),
        ))
}
