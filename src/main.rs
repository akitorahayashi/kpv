use clap::{Parser, Subcommand};
use kpv::commands;
use kpv::error::KpvError;

#[derive(Parser)]
#[command(name = "kpv")]
#[command(about = "Key-Pair Vault: Manage .env files across projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save the current .env file under a key
    #[clap(alias = "sv")]
    Save {
        /// The key name to save the .env file under
        key: String,
    },
    /// Link a saved .env file to the current directory
    #[clap(alias = "ln")]
    Link {
        /// The key name to link from
        key: String,
    },
    /// List all saved keys
    #[clap(alias = "ls")]
    List,
    /// Delete a saved key
    #[clap(alias = "rm")]
    Delete {
        /// The key name to delete
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result: Result<(), KpvError> = match cli.command {
        Commands::Save { key } => commands::save(&key),
        Commands::Link { key } => commands::link(&key),
        Commands::List => commands::list(),
        Commands::Delete { key } => commands::delete(&key),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
