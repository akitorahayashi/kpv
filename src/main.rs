use clap::{Parser, Subcommand};

mod commands;

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
    Save {
        /// The key name to save the .env file under
        key: String,
    },
    /// Link a saved .env file to the current directory
    Link {
        /// The key name to link from
        key: String,
    },
    /// List all saved keys
    List,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Save { key } => commands::save(&key),
        Commands::Link { key } => commands::link(&key),
        Commands::List => commands::list(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
