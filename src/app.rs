use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hostctl")]
#[command(about = "CLI tool to manage /etc/hosts using profiles", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new entry (to current profile)
    Add { host: String, address: String },
    /// Remove an entry (from current profile)
    Remove { host: String },
    /// Update an entry (in current profile)
    Update { host: String, address: String },
    /// Apply changes (write to /etc/hosts)
    Apply {
        #[arg(short, long)]
        profile: Option<String>,
    },
    /// Current hosts file
    Current,
    /// Test config generation (dry run)
    Test {
        #[arg(short, long)]
        profile: Option<String>,
    },
    /// Profile management
    Profile {
        #[command(subcommand)]
        subcommand: ProfileCommands,
    },
}

#[derive(Subcommand)]
pub enum ProfileCommands {
    Add { name: String },
    Remove { name: String },
    Use { name: String },
    List,
}
