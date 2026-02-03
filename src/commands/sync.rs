use crate::{git, github, progress, stack::StackConfig};
use anyhow::{Context, Result};
use colored::Colorize;

pub fn run(dry_run: bool, wait_ci: bool) -> Result<()> {
    if dry_run {
        println!("{}", "[DRY RUN] Sync operations:".yellow().bold());
    } else {
        println!("{}", "Synchronizing stack...".green().bold());
    }
    println!();

    // Load configuration
    let mut config = StackConfig::load().context("Failed to load configuration")?;

    if config.branches.is_empty() {
        println!("{}", "No branches in stack.".yellow());
        return Ok(());
    }

    // Store current branch to restore later
    let original_branch = git::current_branch()?;
    let mut changes_made = false;

    // Process each branch in order
    for i in 0..config.branches.len() {
        let branch_name = config.branches[i].name.clone();
        let current_parent = config.branches[i].parent.clone();
        let pr_number = config.branches[i].pr_number;

        println!(
            "{}",
            format!("Processing {} ...", branch_name).cyan().bold()
        );

        // Check if branch exists
        if !git::branch_exists(&branch_name)? {
            println!("  {} Branch doesn't exist locally, skipping", "⚠".yellow());
            continue;
        }

        // Check CI status if requested
        if wait_ci {
            if let Some(pr_num) = pr_number {
                let spinner = progress::create_spinner(&format!("  Checking CI for PR #{}", pr_num));
                match github::get_ci_status(pr_num) {
                    Ok(ci_status) => {
                        spinner.finish_and_clear();
                        match ci_status.as_str() {
                            "SUCCESS" => {
                                println!("  {} CI passed for PR #{}", "✓".green(), pr_num);
                            }
                            "PENDING" => {
                                println!("  {} CI pending for PR #{}, skipping sync", "⏳".yellow(), pr_num);
                                continue;
                            }
                            "FAILURE" => {
                                println!("  {} CI failed for PR #{}, skipping sync", "✗".red(), pr_num);
                                continue;
                            }
                            _ => {
                                println!("  {} CI status unknown for PR #{}, proceeding", "⚠".yellow(), pr_num);
                            }
                        }
                    }
                    Err(e) => {
                        spinner.finish_and_clear();
                        println!("  {} Failed to check CI status: {}, proceeding", "⚠".yellow(), e);
                    }
                }
            }
        }

        // Determine actual parent (check if current parent's PR is merged)
        let mut new_parent = current_parent.clone();
        let mut parent_was_merged = false;

        // If parent is not the base branch, check if its PR is merged
        if current_parent != config.base_branch {
            // Find parent branch info
            if let Some(parent_info) = config.branches.iter().find(|b| b.name == current_parent) {
                if let Some(parent_pr) = parent_info.pr_number {
                    // Check if parent PR is merged
                    if let Ok(Some(pr)) = github::get_pr(&current_parent) {
                        if pr.state == "MERGED" {
                            println!(
                                "  {} Parent PR #{} is merged, retargeting to {}",
                                "→".green(),
                                parent_pr,
                                config.base_branch
                            );
                            new_parent = config.base_branch.clone();
                            parent_was_merged = true;
                            changes_made = true;
                        }
                    }
                }
            }
        }

        // Update parent if it changed
        if new_parent != current_parent {
            config.branches[i].parent = new_parent.clone();
        }

        if dry_run {
            if parent_was_merged {
                println!("  {} Would rebase onto {}", "↻".yellow(), new_parent);
                if let Some(pr_num) = pr_number {
                    println!(
                        "  {} Would update PR #{} base to {}",
                        "↻".yellow(),
                        pr_num,
                        new_parent
                    );
                }
            } else {
                println!(
                    "  {} Would rebase onto {} (no changes needed)",
                    "↻".dimmed(),
                    new_parent
                );
            }
            continue;
        }

        // Checkout the branch
        let spinner = progress::create_spinner(&format!("  Checking out branch {}", branch_name));
        git::run(&["checkout", &branch_name])?;
        spinner.finish_with_message(format!(
            "  {} Checked out branch {}",
            "✓".green(),
            branch_name
        ));

        // Rebase onto the parent
        let spinner = progress::create_spinner(&format!("  Rebasing onto {}", new_parent));
        match git::rebase(&new_parent) {
            Ok(_) => {
                spinner.finish_with_message(format!(
                    "  {} Rebased onto {}",
                    "✓".green(),
                    new_parent
                ));
            }
            Err(e) => {
                spinner.finish_with_message(format!("  {} Rebase failed", "✗".red()));
                println!();
                println!(
                    "{}",
                    "Rebase failed. Please resolve conflicts manually:".red()
                );
                println!("  1. Resolve conflicts in your editor");
                println!("  2. Run: git add <resolved-files>");
                println!("  3. Run: git rebase --continue");
                println!("  4. Run: gh flow sync again");

                // Try to return to original branch
                let _ = git::run(&["rebase", "--abort"]);
                let _ = git::run(&["checkout", &original_branch]);

                return Err(e);
            }
        }

        // Push changes
        let spinner = progress::create_spinner(&format!("  Pushing changes to {}", branch_name));
        git::push(&branch_name, true)?;
        spinner.finish_with_message(format!("  {} Pushed changes", "✓".green()));

        // Update PR base if needed and PR exists
        if let Some(pr_num) = pr_number {
            if parent_was_merged {
                let spinner = progress::create_spinner(&format!(
                    "  Updating PR #{} base to {}",
                    pr_num, new_parent
                ));
                match github::update_pr_base(pr_num, &new_parent) {
                    Ok(_) => {
                        spinner.finish_with_message(format!(
                            "  {} Updated PR #{} base",
                            "✓".green(),
                            pr_num
                        ));
                    }
                    Err(e) => {
                        spinner.finish_with_message(format!(
                            "  {} Failed to update PR base: {}",
                            "⚠".yellow(),
                            e
                        ));
                    }
                }
            }
        }

        println!();
    }

    // Restore original branch
    if git::branch_exists(&original_branch)? && git::current_branch()? != original_branch {
        git::run(&["checkout", &original_branch])?;
    }

    // Save updated configuration
    if changes_made && !dry_run {
        config.save().context("Failed to save configuration")?;
    }

    if dry_run {
        println!("{}", "✓ Dry run complete (no changes made)".yellow());
    } else {
        println!("{}", "✓ Stack synchronized successfully".green().bold());
        println!();
        println!(
            "Run {} to update PR descriptions with new stack info",
            "gh flow pr update".cyan()
        );
    }

    Ok(())
}
