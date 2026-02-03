use crate::{git, github, stack::StackConfig};
use anyhow::{Context, Result};
use colored::Colorize;

pub fn run() -> Result<()> {
    println!("{}", "Stack Status".green().bold());
    println!();

    // Load configuration
    let config = StackConfig::load().context("Failed to load configuration")?;

    if config.branches.is_empty() {
        println!("{}", "No stack found. Run `gh flow init` first.".yellow());
        return Ok(());
    }

    // Get current branch
    let current_branch = git::current_branch()?;

    // Display base branch
    println!(
        "{} Base branch: {}",
        "→".cyan(),
        config.base_branch.cyan().bold()
    );
    println!();

    // Display stack
    for branch_info in config.branches.iter() {
        let is_current = branch_info.name == current_branch;
        let prefix = if is_current { "▶" } else { " " };

        // Check if branch exists locally
        let exists = git::branch_exists(&branch_info.name)?;
        if !exists {
            println!(
                "  {} {} {} {}",
                prefix.yellow(),
                "✗".red(),
                branch_info.name.red().strikethrough(),
                "(branch deleted)".red()
            );
            continue;
        }

        // Get PR status if PR exists
        let pr_status = if let Some(pr_number) = branch_info.pr_number {
            match github::get_pr(&branch_info.name) {
                Ok(Some(pr)) => {
                    let status_str = match pr.state.as_str() {
                        "OPEN" => format!("PR #{} (open)", pr_number).green().to_string(),
                        "MERGED" => format!("PR #{} (merged)", pr_number).blue().to_string(),
                        "CLOSED" => format!("PR #{} (closed)", pr_number).red().to_string(),
                        _ => format!("PR #{}", pr_number).white().to_string(),
                    };
                    status_str
                }
                Ok(None) => format!("PR #{} (not found)", pr_number)
                    .yellow()
                    .to_string(),
                Err(_) => format!("PR #{} (error)", pr_number).red().to_string(),
            }
        } else {
            "no PR".dimmed().to_string()
        };

        // Show branch in hierarchy
        let connector = "└─";
        let branch_display = if is_current {
            branch_info.name.yellow().bold()
        } else {
            branch_info.name.white()
        };

        println!(
            "  {} {} {} ← {} [{}]",
            prefix,
            connector.cyan(),
            branch_display,
            branch_info.parent.dimmed(),
            pr_status
        );
    }

    println!();
    println!("{}", "Legend:".dimmed());
    println!("  {} Current branch", "▶".yellow());
    println!("  {} Parent branch", "←".cyan());
    println!();
    println!("Use {} to create PRs", "gh flow pr create".cyan());
    println!("Use {} to synchronize the stack", "gh flow sync".cyan());

    Ok(())
}
