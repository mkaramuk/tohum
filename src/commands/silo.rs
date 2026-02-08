use std::path::PathBuf;

use anyhow::Context;
use clap::ArgMatches;
use colored::Colorize;

use crate::{
    cmd::{ARGS_SEED, ARGS_SILO_BRANCH, ARGS_SILO_URL},
    progress::create_spinner,
    silo::{self, read_silo},
};

pub fn silo_list(cmd_matches: &ArgMatches) -> anyhow::Result<()> {
    let silo_branch = cmd_matches.get_one::<String>(ARGS_SILO_BRANCH).unwrap();
    let silo_url = cmd_matches.get_one::<String>(ARGS_SILO_URL).unwrap();
    let spinner = create_spinner("Fetching silo...");

    let silo_path = PathBuf::from(silo_url);
    let seeds = if silo_path.exists() && silo_path.is_dir() {
        read_silo(silo_path)?
    } else {
        silo::fetch_silo_from_git(silo_url, silo_branch)?
    };

    spinner.finish_and_clear();
    println!(
        "🌱 {} {} {}",
        "Found".white(),
        seeds.len().to_string().green().bold(),
        "seeds in the silo:".white()
    );

    println!("{}", "─".repeat(40).bright_black());
    for seed in &seeds {
        println!("  {} {}", "•".cyan().bold(), seed.name.cyan().bold());

        if let Some(desc) = &seed.description {
            let dots = if desc.chars().count() > 100 {
                "..."
            } else {
                ""
            };
            println!(
                "    {}{}",
                desc.chars().take(100).collect::<String>().bright_black(),
                dots
            );
        }

        let primary_author = &seed.authors[0];
        println!("    {} {}", "by".bright_black(), primary_author.name);

        println!();
    }

    Ok(())
}

pub fn silo_inspect(cmd_matches: &ArgMatches) -> anyhow::Result<()> {
    let silo_branch = cmd_matches.get_one::<String>(ARGS_SILO_BRANCH).unwrap();
    let silo_url = cmd_matches.get_one::<String>(ARGS_SILO_URL).unwrap();
    let seed_name = cmd_matches.get_one::<String>(ARGS_SEED).unwrap();
    let spinner = create_spinner("Fetching silo...");

    let silo_path = PathBuf::from(silo_url);
    let seeds = if silo_path.exists() && silo_path.is_dir() {
        read_silo(silo_path)?
    } else {
        silo::fetch_silo_from_git(silo_url, silo_branch)?
    };

    spinner.finish_and_clear();

    let seed = seeds
        .iter()
        .find(|s| s.name == seed_name.as_str())
        .with_context(|| format!("Seed {} is not found in the silo", seed_name))?;

    println!("\n{}", "─".repeat(50).bright_black());
    println!("🌱 {}", seed.name.bold().underline());

    if let Some(description) = &seed.description {
        println!("   {}", description.italic().bright_black());
    }
    println!("{}", "─".repeat(50).bright_black());
    println!("👥 Authors");
    for author in &seed.authors {
        let email = if let Some(e) = &author.email {
            format!(" <{}>", e)
        } else {
            String::new()
        };
        let website = if let Some(w) = &author.website {
            format!(" - {}", w.blue().underline())
        } else {
            String::new()
        };

        println!("  • {} {} {}", author.name.white(), email.dimmed(), website);
    }

    if let Some(tags) = &seed.tags {
        let formatted_tags = tags
            .iter()
            .map(|t| format!("[{}]", t.yellow()))
            .collect::<Vec<_>>()
            .join(" ");
        println!("\n{} {}", "🏷️ Tags:".bold(), formatted_tags);
    }

    if let Some(variables) = &seed.variables {
        println!("\n{}", "⚙️ Variables".white().bold());
        println!("  {}", "─".repeat(30).bright_black());

        for (name, info) in variables {
            let default_val = info
                .default
                .as_ref()
                .map(|v| format!(" = {}", v).bright_black())
                .unwrap_or_default();

            println!(
                "  {} {} {} {}",
                name.yellow().bold(),
                "→".bright_black(),
                info.var_type.cyan().italic(),
                default_val
            );
        }
    }

    Ok(())
}
