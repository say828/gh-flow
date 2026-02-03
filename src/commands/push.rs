use anyhow::{Context, Result};
use colored::Colorize;
use crate::{git, stack::StackConfig};

pub fn run(force: bool) -> Result<()> {
    println!("{}", "Pushing stack branches...".green().bold());

    if force {
        println!("{} Force push enabled", "⚠".yellow());
    }
    println!();

    // Load configuration
    let config = StackConfig::load()
        .context("Failed to load configuration")?;

    if config.branches.is_empty() {
        println!("{}", "No branches in stack. Run `gh flow init` first.".yellow());
        return Ok(());
    }

    // Push each branch
    let mut success_count = 0;
    let mut error_count = 0;

    for branch_info in &config.branches {
        print!("Pushing {} ... ", branch_info.name.cyan());

        // Check if branch exists
        if !git::branch_exists(&branch_info.name)? {
            println!("{}", "skipped (branch doesn't exist)".yellow());
            continue;
        }

        // Push the branch
        match git::push(&branch_info.name, force) {
            Ok(_) => {
                println!("{}", "✓".green());
                success_count += 1;
            }
            Err(e) => {
                println!("{} {}", "✗".red(), e);
                error_count += 1;
            }
        }
    }

    println!();
    if error_count == 0 {
        println!("{} All {} branches pushed successfully", "✓".green(), success_count);
    } else {
        println!(
            "{} {} branches pushed, {} failed",
            if error_count > 0 { "⚠".yellow() } else { "✓".green() },
            success_count,
            error_count
        );
    }

    Ok(())
}
