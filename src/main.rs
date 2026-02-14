mod app;
mod config;
mod db;
mod file_ops;
mod template;

use askama::Template;
use clap::Parser;
use db::Database;
use std::error::Error;
use template::{HostEntry, HostsTemplate};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = app::Cli::parse();

    sudo::escalate_if_needed()?;

    let path = config::db_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let db = Database::open(&path)?;

    match cli.command {
        app::Commands::Add { host, address } => {
            let profile = db
                .get_active_profile()?
                .unwrap_or_else(|| "default".to_string());
            db.add_entry(&profile, &host, &address)?;
            log::info!("Added {} → {}", host, address);
        }
        app::Commands::Remove { host } => {
            let profile = db
                .get_active_profile()?
                .unwrap_or_else(|| "default".to_string());
            if !db.remove_entry(&profile, &host)? {
                log::error!("Entry for host '{}' not found", host);
                std::process::exit(1);
            }
        }
        app::Commands::Update { host, address } => {
            let profile = db
                .get_active_profile()?
                .unwrap_or_else(|| "default".to_string());
            if !db.update_entry(&profile, &host, &address)? {
                log::error!("Entry for host '{}' not found", host);
                std::process::exit(1);
            }
        }
        app::Commands::Apply { profile } => {
            let profile_name = profile
                .or_else(|| db.get_active_profile().ok().flatten())
                .unwrap_or_else(|| "default".to_string());
            if !db.list_profiles()?.contains(&profile_name.clone()) {
                log::error!("Profile '{}' doesn't exist", profile_name);
                std::process::exit(1);
            }

            let entries = db.get_entries(&profile_name)?;
            let template = HostsTemplate {
                profile: profile_name.clone(),
                entries: &entries
                    .iter()
                    .map(|(h, a)| HostEntry {
                        host: h.clone(),
                        address: a.clone(),
                    })
                    .collect::<Vec<_>>(),
            };
            let content = template.render()?;
            file_ops::write_hosts(&content)?;
        }
        app::Commands::Current => {
            let hosts = file_ops::read_hosts().unwrap();

            eprintln!("=== CURRENT HOSTS FILE OUTPUT ===");
            println!("{hosts}");
        }
        app::Commands::Test { profile } => {
            let profile_name = profile
                .or_else(|| db.get_active_profile().ok().flatten())
                .unwrap_or_else(|| "default".to_string());
            if !db.list_profiles()?.contains(&profile_name.clone()) {
                log::error!("Profile '{}' doesn't exist", profile_name);
                std::process::exit(1);
            }

            let entries = db.get_entries(&profile_name)?;
            let template = HostsTemplate {
                profile: profile_name.clone(),
                entries: &entries
                    .iter()
                    .map(|(h, a)| HostEntry {
                        host: h.clone(),
                        address: a.clone(),
                    })
                    .collect::<Vec<_>>(),
            };
            let content = template.render()?;
            file_ops::dry_run(&content);
        }
        app::Commands::Profile { subcommand } => match subcommand {
            app::ProfileCommands::Add { name } => {
                db.add_profile(&name)?;
                log::info!("Profile '{}' created", name);
            }
            app::ProfileCommands::Remove { name } => {
                if !db.remove_profile(&name)? {
                    log::error!("Profile '{}' not found", name);
                    std::process::exit(1);
                }
                log::info!("Profile '{}' removed", name);
            }
            app::ProfileCommands::Use { name } => {
                db.use_profile(&name)?;
                log::info!("Switched to profile '{}'", name);
            }
            app::ProfileCommands::List => {
                let profiles = db.list_profiles()?;
                println!("Available profiles:");
                for p in profiles {
                    if p == db.get_active_profile()?.as_deref().unwrap_or_default() {
                        println!("• {} (active)", p);
                    } else {
                        println!("• {}", p);
                    }
                }
            }
        },
    }

    Ok(())
}
