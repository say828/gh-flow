use crate::{git, stack::{self, StackConfig}};
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;

const PR_TEMPLATE_FILE: &str = "pr-template.md";
const DEFAULT_PR_TEMPLATE: &str = r#"## Stack

{{stack}}

---
*This PR is part of a stack. Use [gh-flow](https://github.com/say828/gh-flow) to manage stacked PRs.*
"#;

fn create_pr_template() -> Result<(bool, String)> {
    let global_dir = stack::get_global_config_dir()?;
    let template_path = global_dir.join(PR_TEMPLATE_FILE);

    if template_path.exists() {
        return Ok((false, template_path.display().to_string()));
    }

    // Ensure directory exists
    fs::create_dir_all(&global_dir)?;
    fs::write(&template_path, DEFAULT_PR_TEMPLATE)?;
    Ok((true, template_path.display().to_string()))
}

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

    let repo_dir = stack::get_repo_config_dir()?;
    println!("{} Configuration saved to {}", "✓".green(), repo_dir.join("gh-flow.json").display());

    // Create PR template if not exists
    match create_pr_template() {
        Ok((true, path)) => println!("{} Created PR template: {}", "✓".green(), path),
        Ok((false, path)) => println!("{} PR template exists: {}", "✓".green(), path),
        Err(_) => println!("{} Failed to create PR template", "⚠".yellow()),
    }

    println!();
    println!("{}", "✓ Stack initialized successfully!".green().bold());
    println!();
    println!("Next steps:");
    println!("  1. View stack status: gh flow status");
    println!("  2. Create PRs: gh flow pr create");

    Ok(())
}
