use anyhow::{bail, Result};
use colored::*;
use std::process::Command;

use crate::github;
use crate::progress::create_spinner;
use crate::stack::StackConfig;

pub fn run(auto: bool, wait_ci: bool) -> Result<()> {
    let config = StackConfig::load()?;

    if config.branches.is_empty() {
        println!("{}", "No branches in stack".yellow());
        return Ok(());
    }

    println!("{}", "🔀 Merge Stack".cyan().bold());
    println!();

    // Get all PRs in order
    let branches_with_prs: Vec<_> = config.branches.iter()
        .filter(|b| b.pr_number.is_some())
        .collect();

    if branches_with_prs.is_empty() {
        bail!("No PRs found in stack. Run 'gh flow pr create' first.");
    }

    for branch in &branches_with_prs {
        let pr_number = branch.pr_number.unwrap();

        // Check PR status
        let spinner = create_spinner(&format!("Checking PR #{}", pr_number));
        let pr_status = github::get_pr_status(pr_number)?;
        spinner.finish_and_clear();

        match pr_status.as_str() {
            "MERGED" => {
                println!("  {} PR #{} already merged", "✓".green(), pr_number);
                continue;
            }
            "CLOSED" => {
                println!("  {} PR #{} is closed", "✗".red(), pr_number);
                continue;
            }
            _ => {}
        }

        // Check CI status if requested
        if wait_ci {
            let spinner = create_spinner(&format!("Checking CI for PR #{}", pr_number));
            let ci_status = github::get_ci_status(pr_number)?;
            spinner.finish_and_clear();

            match ci_status.as_str() {
                "SUCCESS" => println!("  {} CI passed for PR #{}", "✓".green(), pr_number),
                "PENDING" => {
                    println!("  {} CI pending for PR #{}, skipping", "⏳".yellow(), pr_number);
                    continue;
                }
                "FAILURE" => {
                    println!("  {} CI failed for PR #{}, skipping", "✗".red(), pr_number);
                    continue;
                }
                _ => {}
            }
        }

        // Check if PR is approved
        let spinner = create_spinner(&format!("Checking reviews for PR #{}", pr_number));
        let review_status = github::get_review_status(pr_number)?;
        spinner.finish_and_clear();

        if review_status != "APPROVED" && !auto {
            println!("  {} PR #{} not approved, skipping", "⚠".yellow(), pr_number);
            continue;
        }

        if auto {
            // Enable auto-merge
            let spinner = create_spinner(&format!("Enabling auto-merge for PR #{}", pr_number));
            let output = Command::new("gh")
                .args(["pr", "merge", &pr_number.to_string(), "--auto", "--squash"])
                .output()?;
            spinner.finish_and_clear();

            if output.status.success() {
                println!("  {} Auto-merge enabled for PR #{}", "✓".green(), pr_number);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("  {} Failed to enable auto-merge for PR #{}: {}", "✗".red(), pr_number, stderr.trim());
            }
        } else {
            // Merge immediately
            let spinner = create_spinner(&format!("Merging PR #{}", pr_number));
            let output = Command::new("gh")
                .args(["pr", "merge", &pr_number.to_string(), "--squash"])
                .output()?;
            spinner.finish_and_clear();

            if output.status.success() {
                println!("  {} Merged PR #{}", "✓".green(), pr_number);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("  {} Failed to merge PR #{}: {}", "✗".red(), pr_number, stderr.trim());
                // Stop on first failure to maintain order
                if !auto {
                    break;
                }
            }
        }
    }

    println!();
    println!("{}", "Done!".green().bold());
    Ok(())
}
