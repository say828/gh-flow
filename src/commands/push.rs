use crate::{git, progress, stack::StackConfig};
use anyhow::{Context, Result};
use colored::Colorize;

pub fn run(force: bool) -> Result<()> {
    println!("{}", "Pushing stack branches...".green().bold());

    if force {
        println!("{} Force push enabled", "⚠".yellow());
    }
    println!();

    // Load configuration
    let config = StackConfig::load().context("Failed to load configuration")?;

    if config.branches.is_empty() {
        println!(
            "{}",
            "No branches in stack. Run `gh flow init` first.".yellow()
        );
        return Ok(());
    }

    // Push each branch
    let mut success_count = 0;
    let mut error_count = 0;

    let total_branches = config.branches.len() as u64;
    let progress_bar = progress::create_progress_bar(total_branches, "Pushing branches");

    for branch_info in &config.branches {
        progress_bar.set_message(format!("Pushing {}", branch_info.name));

        // Check if branch exists
        if !git::branch_exists(&branch_info.name)? {
            progress_bar.println(format!(
                "{} {} - skipped (branch doesn't exist)",
                "⚠".yellow(),
                branch_info.name
            ));
            progress_bar.inc(1);
            continue;
        }

        // Push the branch
        match git::push(&branch_info.name, force) {
            Ok(_) => {
                progress_bar.println(format!(
                    "{} Pushed {}",
                    "✓".green(),
                    branch_info.name.cyan()
                ));
                success_count += 1;
            }
            Err(e) => {
                progress_bar.println(format!(
                    "{} Failed to push {}: {}",
                    "✗".red(),
                    branch_info.name.cyan(),
                    e
                ));
                error_count += 1;
            }
        }
        progress_bar.inc(1);
    }

    progress_bar.finish_and_clear();

    println!();
    if error_count == 0 {
        println!(
            "{} All {} branches pushed successfully",
            "✓".green(),
            success_count
        );
    } else {
        println!(
            "{} {} branches pushed, {} failed",
            if error_count > 0 {
                "⚠".yellow()
            } else {
                "✓".green()
            },
            success_count,
            error_count
        );
    }

    Ok(())
}
