use std::{fs, path::PathBuf};

use anyhow::{Context, anyhow};
use clap::ArgMatches;
use colored::Colorize;
use tempfile::TempDir;
use tera::Tera;
use walkdir::WalkDir;

use crate::{
    cmd::{
        ARGS_FORCE, ARGS_PATH, ARGS_PROJECT_NAME, ARGS_SEED, ARGS_SILO_BRANCH, ARGS_SILO_URL,
        ARGS_VARIABLES,
    },
    constants::TOHUMRC_FILENAME,
    git::git_sparse_clone,
    io::{copy_dir_recursive, is_binary},
    log_err_recursive,
    progress::create_spinner,
    silo::{self, read_silo},
};

pub fn plant_seed(cmd_matches: &ArgMatches) -> anyhow::Result<()> {
    let project_name = cmd_matches.get_one::<String>(ARGS_PROJECT_NAME).unwrap();
    let silo_branch = cmd_matches.get_one::<String>(ARGS_SILO_BRANCH).unwrap();
    let silo_url = cmd_matches.get_one::<String>(ARGS_SILO_URL).unwrap();
    let seed_name = cmd_matches.get_one::<String>(ARGS_SEED).unwrap();
    let path = cmd_matches.get_one::<String>(ARGS_PATH).unwrap();
    let force = cmd_matches.get_one::<bool>(ARGS_FORCE).unwrap();
    let spinner = create_spinner("Fetching silo...");

    let mut project_path = PathBuf::from(path);
    project_path.push(project_name);

    if project_path.exists() {
        if !force {
            return Err(anyhow!(
                "Target dir {} already exists. Either use another path, project name or \"--force\" flag to overwrite",
                project_path.to_string_lossy().cyan()
            ));
        }

        if project_path.is_file() {
            fs::remove_file(&project_path)?;
        } else if project_path.is_dir() {
            fs::remove_dir_all(&project_path)?;
        }
    }

    let silo_path = PathBuf::from(silo_url);
    let seeds = if silo_path.exists() && silo_path.is_dir() {
        read_silo(silo_path)?
    } else {
        silo::fetch_silo_from_git(silo_url, silo_branch)?
    };
    let seed = seeds
        .iter()
        .find(|s| s.name == *seed_name)
        .with_context(|| format!("Seed {} is not found in the silo", seed_name))?;

    let temp_dir = TempDir::new().context("Failed to create temporary directory")?;
    let temp_path = temp_dir.path();

    let glob_pattern = PathBuf::from(&seed.repo_path).join("*");
    let mut seed_repo_path = temp_path.to_path_buf();
    seed_repo_path.push(&seed.repo_path);

    git_sparse_clone(&silo_url, &silo_branch, &glob_pattern, &temp_path)?;

    let mut project_dir = PathBuf::from(&path);
    project_dir.push(project_name);

    spinner.set_message("Setting up the project...");
    copy_dir_recursive(seed_repo_path, &project_dir)?;

    let mut variables = tera::Context::new();
    if let Some(arg_var) = cmd_matches.get_many::<String>(ARGS_VARIABLES) {
        for arg in arg_var {
            let (key, val) = arg
                .split_once('=')
                .with_context(|| format!("Invalid variable format: {}. Use 'key=value'", arg))?;
            variables.insert(key, val);
        }
    }

    // Validate the variables
    if let Some(seed_variables) = &seed.variables {
        for (name, info) in seed_variables {
            if !variables.contains_key(name) {
                if info.required && info.default.is_none() {
                    return Err(anyhow!(format!(
                        "Template variable {} is required but not a value given",
                        name.blue()
                    )));
                }

                // If a default value is defined then use it
                if let Some(default_value) = &info.default {
                    variables.insert(name, default_value);
                }
            }
        }
    }

    // Default variables
    variables.insert("project_name", project_name);
    variables.insert("authors", &seed.authors);

    spinner.set_message("Planting the seed...");

    for entry_result in WalkDir::new(&project_dir) {
        let entry = match entry_result {
            Ok(e) => e,
            Err(err) => {
                eprintln!(
                    "{}: Couldn't read the file/directory: {}",
                    "Warning:".yellow().bold(),
                    err
                );
                continue;
            }
        };

        let path = entry.path();
        if !path.is_file() || is_binary(path) {
            continue;
        }

        // TODO: Read the file chunk by chunk and detect whether it is plaintext at the first chunk.

        // Render the template
        match fs::read_to_string(path) {
            Ok(content) => {
                let mut template = Tera::default();
                template.add_raw_template("template", &content)?;

                match template.render("template", &variables) {
                    Ok(rendered) => {
                        if let Err(e) = fs::write(path, rendered) {
                            eprintln!(
                                "{}: Could not write to {}: {}",
                                "Error".red().bold(),
                                path.to_string_lossy().cyan(),
                                e
                            );
                        }
                    }
                    Err(e) => {
                        log_err_recursive!(e, "Could not render {}", path.to_string_lossy().cyan());
                    }
                }
            }
            Err(e) => eprintln!(
                "{}: Could not read {}: {}",
                "Error".red().bold(),
                path.to_string_lossy().cyan(),
                e
            ),
        }
    }

    // Delete .tohumrc file
    fs::remove_file(project_dir.join(TOHUMRC_FILENAME))?;

    spinner.finish_and_clear();
    println!(
        "Project {} planted at {} from {} seed!",
        project_name.cyan(),
        project_path.to_string_lossy().cyan(),
        seed_name.cyan()
    );

    Ok(())
}
