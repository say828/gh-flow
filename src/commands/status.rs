use crate::{git, github, stack::StackConfig};
use anyhow::{Context, Result};
use colored::Colorize;

pub fn run() -> Result<()> {
    println!("{}", "Stack Status".green().bold());
    println!();

    // Auto-discover branch chain
    let config = StackConfig::discover("main").context("Failed to discover branches")?;

    if config.branches.is_empty() {
        println!("{}", "No branches found from main to current branch.".yellow());
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

        // Get PR status by querying GitHub directly
        let pr_status = match github::get_pr(&branch_info.name) {
            Ok(Some(pr)) => {
                let status_str = match pr.state.as_str() {
                    "OPEN" => format!("PR #{} (open)", pr.number).green().to_string(),
                    "MERGED" => format!("PR #{} (merged)", pr.number).blue().to_string(),
                    "CLOSED" => format!("PR #{} (closed)", pr.number).red().to_string(),
                    _ => format!("PR #{}", pr.number).white().to_string(),
                };
                status_str
            }
            Ok(None) => "no PR".dimmed().to_string(),
            Err(_) => "error".red().to_string(),
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
