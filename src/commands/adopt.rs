use crate::{git, github, stack::StackConfig};
use anyhow::{Context, Result};
use colored::Colorize;

pub fn run() -> Result<()> {
    println!("{}", "Adopting existing PR into stack...".green().bold());
    println!();

    // Load configuration
    let mut config = StackConfig::load().context("Failed to load configuration")?;

    if config.base_branch.is_empty() {
        anyhow::bail!("No stack found. Run `gh flow init` first.");
    }

    // Get current branch
    let current_branch = git::current_branch()?;
    println!("Current branch: {}", current_branch.cyan());

    // Check if branch already in stack
    if config.branches.iter().any(|b| b.name == current_branch) {
        anyhow::bail!("Branch '{}' is already in the stack", current_branch);
    }

    // Check if PR exists for this branch
    let pr = github::get_pr(&current_branch)?
        .context(format!("No PR found for branch '{}'. Create a PR first.", current_branch))?;

    println!("Found PR #{}: {}", pr.number, pr.title);
    println!("Current base: {}", pr.base_ref.yellow());

    // Determine parent branch (last branch in stack, or base_branch if empty)
    let parent_branch = if let Some(last) = config.branches.last() {
        last.name.clone()
    } else {
        config.base_branch.clone()
    };

    println!("New base: {}", parent_branch.green());
    println!();

    // Update PR base if different
    if pr.base_ref != parent_branch {
        print!("Updating PR base... ");
        github::update_pr_base(pr.number, &parent_branch)?;
        println!("{}", "✓".green());
    } else {
        println!("PR base already correct.");
    }

    // Add branch to stack
    config.add_branch(current_branch.clone(), parent_branch.clone());
    if let Some(branch_info) = config.branches.iter_mut().find(|b| b.name == current_branch) {
        branch_info.pr_number = Some(pr.number);
    }

    // Save configuration
    config.save().context("Failed to save configuration")?;

    println!();
    println!(
        "{} Branch '{}' adopted into stack (PR #{})",
        "✓".green().bold(),
        current_branch.cyan(),
        pr.number
    );
    println!("  Parent: {}", parent_branch);

    Ok(())
}
