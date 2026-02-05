use crate::{git, stack::StackConfig};
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

const PR_TEMPLATE_FILE: &str = "gh-flow-pr-template.md";
const DEFAULT_PR_TEMPLATE: &str = r#"## Stack

{{stack}}

---
*This PR is part of a stack. Use [gh-flow](https://github.com/say828/gh-flow) to manage stacked PRs.*
"#;

fn create_pr_template() -> Result<bool> {
    let git_dir = git::run(&["rev-parse", "--git-dir"])?;
    let template_path = PathBuf::from(git_dir).join(PR_TEMPLATE_FILE);

    if template_path.exists() {
        return Ok(false);
    }

    fs::write(template_path, DEFAULT_PR_TEMPLATE)?;
    Ok(true)
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

    // Create PR template if not exists
    match create_pr_template() {
        Ok(true) => println!("{} Created PR template: .git/{}", "✓".green(), PR_TEMPLATE_FILE),
        Ok(false) => println!("{} PR template already exists", "✓".green()),
        Err(_) => println!("{} Failed to create PR template", "⚠".yellow()),
    }

    println!();
    println!("{}", "✓ Stack initialized successfully!".green().bold());
    println!();
    println!("Configuration saved to .git/gh-flow.json");
    println!("PR template: .git/{}", PR_TEMPLATE_FILE);
    println!();
    println!("Next steps:");
    println!("  1. Edit PR template: .git/{}", PR_TEMPLATE_FILE);
    println!("  2. View stack status: gh flow status");
    println!("  3. Create PRs: gh flow pr create");

    Ok(())
}
