use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

mod commands;
mod git;
mod github;
mod stack;

#[derive(Parser)]
#[command(
    name = "gh-flow",
    about = "GitHub CLI extension for managing stacked PRs",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new PR stack
    Init {
        /// Base branch (default: main)
        #[arg(short, long, default_value = "main")]
        base: String,
    },

    /// Show the status of the current PR stack
    Status,

    /// Synchronize the entire stack (rebase + retarget PRs)
    Sync {
        /// Dry run - show what would be done without doing it
        #[arg(short, long)]
        dry_run: bool,
    },

    /// Push all branches in the stack
    Push {
        /// Force push
        #[arg(short, long)]
        force: bool,
    },

    /// Create PRs for the entire stack
    Pr {
        #[command(subcommand)]
        action: PrAction,
    },
}

#[derive(Subcommand)]
enum PrAction {
    /// Create PRs for all branches in the stack
    Create {
        /// Create as draft PRs
        #[arg(short, long)]
        draft: bool,
    },

    /// Update existing PRs
    Update,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { base } => {
            commands::init::run(&base)?;
        }
        Commands::Status => {
            commands::status::run()?;
        }
        Commands::Sync { dry_run } => {
            commands::sync::run(dry_run)?;
        }
        Commands::Push { force } => {
            commands::push::run(force)?;
        }
        Commands::Pr { action } => {
            match action {
                PrAction::Create { draft } => {
                    commands::pr::create(draft)?;
                }
                PrAction::Update => {
                    commands::pr::update()?;
                }
            }
        }
    }

    Ok(())
}
