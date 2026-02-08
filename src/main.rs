mod cmd;
mod commands;
mod config;
mod constants;
mod git;
mod io;
mod macros;
mod process;
mod progress;
mod seed;
mod silo;

use crate::{
    cmd::{CMD_PLANT, CMD_SILO, CMD_SILO_INSPECT, CMD_SILO_LIST},
    commands::{
        plant::plant_seed,
        silo::{silo_inspect, silo_list},
    },
};
use anyhow::{Error, Result};
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    if let Err(err) = run() {
        eprintln!("{}: {}", "Error".red().bold(), err);
        std::process::exit(1);
    }

    Ok(())
}

fn run() -> Result<(), Error> {
    let _config_path = config::config_path()?;
    let command = cmd::build_cmd();
    let cmd_matches = command.get_matches();

    if let Some(silo_matches) = cmd_matches.subcommand_matches(CMD_SILO) {
        if let Some(list_matches) = silo_matches.subcommand_matches(CMD_SILO_LIST) {
            silo_list(list_matches)?
        } else if let Some(inspect_matches) = silo_matches.subcommand_matches(CMD_SILO_INSPECT) {
            silo_inspect(inspect_matches)?
        }
    } else if let Some(plant_matches) = cmd_matches.subcommand_matches(CMD_PLANT) {
        plant_seed(plant_matches)?
    }

    Ok(())
}
