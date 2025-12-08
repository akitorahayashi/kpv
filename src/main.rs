use clap::{Parser, Subcommand};
use kpv::commands;
use kpv::error::KpvError;

#[derive(Parser)]
#[command(name = "kpv")]
#[command(version)]
#[command(about = "Key-Pair Vault: Manage .env files across projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save the current .env file. Uses current directory name if <KEY> is omitted.
    #[clap(visible_alias = "sv")]
    Save {
        /// The key name to save under. If omitted, uses the current directory's name.
        key: Option<String>,
    },
    /// Link a saved .env file to the current directory
    #[clap(visible_alias = "ln")]
    Link {
        /// The key name to link from. If omitted, uses the current directory's name.
        key: Option<String>,
    },
    /// List all saved keys
    #[clap(visible_alias = "ls")]
    List,
    /// Delete a saved key
    #[clap(visible_alias = "d")]
    Delete {
        /// The key name to delete
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result: Result<(), KpvError> = match cli.command {
        Commands::Save { key } => commands::save(key.as_deref()),
        Commands::Link { key } => commands::link(key.as_deref()),
        Commands::List => commands::list(),
        Commands::Delete { key } => commands::delete(&key),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
