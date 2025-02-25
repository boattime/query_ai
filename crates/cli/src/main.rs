use clap::Parser;
use tracing::info;

mod commands;
use commands::{Cli, Command};

use query_ai_core::{config::Config, logging, Result};

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Initialize logging
    let log_level = cli.get_log_level()?;
    logging::init(log_level)?;

    // Load configuration
    let config = match &cli.config {
        Some(path) => {
            info!("Loading configuration from {}", path);
            Config::load(path)?
        }
        None => {
            info!("Loading default configuration");
            Config::load_default()?
        }
    };

    // Validate configuration
    config.validate()?;

    // Execute subcommand
    match cli.command {
        Command::Index(cmd) => {
            info!("Indexing repository: {}", cmd.repo_url);
            // TODO: Implement indexing
            println!("Repository indexing not yet implemented");
        }
        Command::Update(cmd) => {
            info!("Updating index from webhook payload: {}", cmd.payload_file);
            // TODO: Implement updating
            println!("Index updating not yet implemented");
        }
        Command::Query(cmd) => {
            info!("Executing query: {}", cmd.query);
            // TODO: Implement querying
            println!("Query execution not yet implemented");
        }
    }

    Ok(())
}
