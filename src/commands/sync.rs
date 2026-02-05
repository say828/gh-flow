use crate::{git, github, stack::StackConfig};
use anyhow::{Context, Result};
use colored::Colorize;

pub fn run(dry_run: bool, _wait_ci: bool) -> Result<()> {
    if dry_run {
        println!("{}", "[DRY RUN] Sync operations:".yellow().bold());
    } else {
        println!("{}", "Synchronizing stack...".green().bold());
    }
    println!();

    let base_branch = "main";

    // Step 1: Fetch and check main branch
    println!("{}", "Fetching remote...".cyan());

    if !dry_run {
        match git::run(&["fetch", "origin", base_branch]) {
            Ok(_) => println!("  {} Fetched origin/{}", "✓".green(), base_branch),
            Err(e) => println!("  {} Failed to fetch: {}", "⚠".yellow(), e),
        }
    }

    // Check if local main is up to date
    let local_main = git::run(&["rev-parse", base_branch]).unwrap_or_default();
    let remote_main = git::run(&["rev-parse", &format!("origin/{}", base_branch)]).unwrap_or_default();

    if local_main != remote_main && !local_main.is_empty() && !remote_main.is_empty() {
        println!("  {} Local {} is behind origin/{}", "⚠".yellow(), base_branch, base_branch);
        if !dry_run {
            let current = git::current_branch()?;
            if current != base_branch {
                let _ = git::run(&["branch", "-f", base_branch, &format!("origin/{}", base_branch)]);
                println!("  {} Updated local {}", "✓".green(), base_branch);
            }
        }
    } else {
        println!("  {} Local {} is up to date", "✓".green(), base_branch);
    }
    println!();

    // Step 2: Auto-discover branch chain
    println!("{}", "Discovering branch chain...".cyan());
    let config = StackConfig::discover(base_branch).context("Failed to discover branches")?;

    if config.branches.is_empty() {
        println!("  {} No branches found", "⚠".yellow());
        return Ok(());
    }

    println!("  {} (base)", base_branch.green());
    for (i, branch) in config.branches.iter().enumerate() {
        let is_last = i == config.branches.len() - 1;
        if is_last {
            println!("    └─ {}  ← current", branch.name.cyan().bold());
        } else {
            println!("    └─ {}", branch.name);
        }
    }
    println!();

    // Step 3: Sync PR bases
    println!("{}", "Syncing PR targets...".cyan());

    for branch_info in &config.branches {
        let pr = github::get_pr(&branch_info.name)?;

        if let Some(pr) = pr {
            let current_base = &pr.base_ref;
            let expected_base = &branch_info.parent;

            if current_base != expected_base {
                if dry_run {
                    println!(
                        "  {} PR #{} ({}) base: {} → {}",
                        "↻".yellow(),
                        pr.number,
                        branch_info.name,
                        current_base.red(),
                        expected_base.green()
                    );
                } else {
                    print!(
                        "  PR #{} ({}) base: {} → {} ... ",
                        pr.number,
                        branch_info.name,
                        current_base.red(),
                        expected_base.green()
                    );
                    match github::update_pr_base(pr.number, expected_base) {
                        Ok(_) => println!("{}", "✓".green()),
                        Err(e) => println!("{} {}", "✗".red(), e),
                    }
                }
            } else {
                println!(
                    "  {} PR #{} ({}) base: {}",
                    "✓".green(),
                    pr.number,
                    branch_info.name,
                    current_base
                );
            }
        } else {
            println!("  {} {} - no PR", "○".dimmed(), branch_info.name);
        }
    }

    // Save config
    if !dry_run {
        config.save().context("Failed to save config")?;
    }

    println!();
    if dry_run {
        println!("{}", "✓ Dry run complete".yellow());
    } else {
        println!("{}", "✓ Stack synchronized".green().bold());
    }

    Ok(())
}
