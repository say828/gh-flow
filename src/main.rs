use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_complete::Shell;

mod commands;
mod git;
mod github;
mod progress;
mod stack;
mod validation;

#[derive(Parser)]
#[command(
    name = "gh-flow",
    about = "GitHub CLI extension for managing stacked PRs",
    version
)]
pub struct Cli {
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

    /// Adopt an existing PR into the stack
    Adopt,

    /// Show the status of the current PR stack
    Status,

    /// Synchronize the entire stack (rebase + retarget PRs)
    Sync {
        /// Dry run - show what would be done without doing it
        #[arg(short, long)]
        dry_run: bool,

        /// Wait for CI to pass before syncing
        #[arg(long)]
        wait_ci: bool,
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

    /// Merge PRs in the stack (with auto-merge support)
    Merge {
        /// Enable auto-merge (merge when CI passes and approved)
        #[arg(long)]
        auto: bool,

        /// Wait for CI to pass before merging
        #[arg(long)]
        wait_ci: bool,
    },

    /// Split the stack into separate stacks
    Split {
        /// Interactive mode to select branches
        #[arg(short, long)]
        interactive: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
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
        Commands::Adopt => {
            commands::adopt::run()?;
        }
        Commands::Status => {
            commands::status::run()?;
        }
        Commands::Sync { dry_run, wait_ci } => {
            commands::sync::run(dry_run, wait_ci)?;
        }
        Commands::Push { force } => {
            commands::push::run(force)?;
        }
        Commands::Pr { action } => match action {
            PrAction::Create { draft } => {
                commands::pr::create(draft)?;
            }
            PrAction::Update => {
                commands::pr::update()?;
            }
        },
        Commands::Merge { auto, wait_ci } => {
            commands::merge::run(auto, wait_ci)?;
        }
        Commands::Split { interactive } => {
            commands::split::run(interactive)?;
        }
        Commands::Completions { shell } => {
            commands::completions::run(shell)?;
        }
    }

    Ok(())
}
