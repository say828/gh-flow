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

    // Get current branch
    let current = git::current_branch()?;

    // Create configuration
    let mut config = StackConfig {
        base_branch: base.to_string(),
        ..Default::default()
    };

    // If we're not on the base branch, add current branch to stack
    if current != base {
        println!(
            "{} Adding current branch '{}' to stack",
            "✓".green(),
            current
        );
        config.add_branch(current.clone(), base.to_string());
    }

    // Save configuration
    config.save().context("Failed to save configuration")?;

    println!();
    println!("{}", "✓ Stack initialized successfully!".green().bold());
    println!();
    println!("Configuration saved to .git/gh-flow.json");
    println!();
    println!("Next steps:");
    println!("  1. Create your stacked branches: git checkout -b feature/branch-name");
    println!("  2. View stack status: gh flow status");
    println!("  3. Create PRs: gh flow pr create");

    Ok(())
}
