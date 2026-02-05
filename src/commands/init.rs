use crate::{git, stack::StackConfig};
use anyhow::{Context, Result};
use colored::Colorize;

pub fn run(base: &str) -> Result<()> {
    println!("{}", "Initializing gh-flow stack...".green().bold());
    println!("Base branch: {}", base.cyan());
    println!();

    // Verify we're in a git repository
    git::current_branch()
        .context("Not in a git repository. Please run this command from within a git repo.")?;

    println!("{} Verified git repository", "✓".green());

    // Check if base branch exists
    if !git::branch_exists(base)? {
        anyhow::bail!(
            "Base branch '{}' does not exist. Please create it or specify a different base branch.",
            base
        );
    }

    println!("{} Base branch '{}' exists", "✓".green(), base);

    // Auto-discover branch chain from git history
    let config = StackConfig::discover(base)?;

    if config.branches.is_empty() {
        println!("{} No branches found in stack", "✓".green());
    } else {
        println!(
            "{} Discovered {} branches in stack:",
            "✓".green(),
            config.branches.len()
        );
        for branch in &config.branches {
            println!("    {} ← {}", branch.name.cyan(), branch.parent.dimmed());
        }
    }

    // Save configuration
    config.save().context("Failed to save configuration")?;

    println!();
    println!("{}", "✓ Stack initialized successfully!".green().bold());
    println!();
    println!("Configuration saved to .git/gh-flow.json");
    println!();
    println!("Next steps:");
    println!("  1. View stack status: gh flow status");
    println!("  2. Create PRs: gh flow pr create");

    Ok(())
}
