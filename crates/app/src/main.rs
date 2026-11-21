//! The application crate: parses the command line, loads configuration, wires features.
//!
//! It is the only crate that knows about every feature. Features never depend on each other.

use anyhow::Result;
use clap::{Parser, Subcommand};
use essential_core::Config;
use essential_feature_greeting::greet;
use essential_feature_todo::TodoList;
use tracing_subscriber::EnvFilter;

/// Rust Essential command line.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print a greeting.
    Greet {
        /// Who to greet.
        name: String,
    },
    /// Run a small in-memory todo demo.
    Todo,
}

fn main() -> Result<()> {
    let config = Config::from_env()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(&config.log_level))
        .init();

    match Cli::parse().command {
        Command::Greet { name } => println!("{}", greet(&name, &config.app_name)?),
        Command::Todo => {
            let mut list = TodoList::new();
            let id = list.add("ship the template")?;
            list.complete(id)?;
            for todo in list.items() {
                println!(
                    "[{}] {} {}",
                    if todo.done { "x" } else { " " },
                    todo.id,
                    todo.title
                );
            }
        }
    }
    Ok(())
}
