use anyhow::{bail, Result};
use colored::*;
use dialoguer::{Confirm, MultiSelect};
use std::process::Command;

use crate::stack::{BranchInfo, StackConfig};

pub fn run(interactive: bool) -> Result<()> {
    let config = StackConfig::load()?;

    if config.branches.len() < 2 {
        bail!("Stack needs at least 2 branches to split");
    }

    println!("{}", "✂️  Split Stack".cyan().bold());
    println!();

    let branch_names: Vec<&str> = config.branches.iter()
        .map(|b| b.name.as_str())
        .collect();

    let selected = if interactive {
        let selection = MultiSelect::new()
            .with_prompt("Select branches for the new stack")
            .items(&branch_names)
            .interact()?;

        if selection.is_empty() {
            bail!("No branches selected");
        }

        selection
    } else {
        // Non-interactive: split at the middle
        let mid = config.branches.len() / 2;
        (mid..config.branches.len()).collect()
    };

    println!();
    println!("Branches to split into new stack:");
    for &idx in &selected {
        println!("  - {}", branch_names[idx].cyan());
    }

    if !Confirm::new()
        .with_prompt("Proceed with split?")
        .default(false)
        .interact()?
    {
        println!("Cancelled.");
        return Ok(());
    }

    // Create new stack with selected branches
    let mut new_branches: Vec<BranchInfo> = Vec::new();
    let mut remaining_branches: Vec<BranchInfo> = Vec::new();

    for (idx, branch) in config.branches.into_iter().enumerate() {
        if selected.contains(&idx) {
            new_branches.push(branch);
        } else {
            remaining_branches.push(branch);
        }
    }

    // Update parent of first branch in new stack to point to base
    if let Some(first) = new_branches.first_mut() {
        first.parent = config.base_branch.clone();
    }

    // Save original stack with remaining branches
    let original_config = StackConfig {
        base_branch: config.base_branch.clone(),
        branches: remaining_branches,
    };
    original_config.save()?;

    // Update the PRs to point to base branch
    for branch in &new_branches {
        if let Some(pr_number) = branch.pr_number {
            println!("  Updating PR #{} base to {}...", pr_number, config.base_branch);
            let _ = Command::new("gh")
                .args(["pr", "edit", &pr_number.to_string(), "--base", &config.base_branch])
                .output();
        }
    }

    println!();
    println!("{} Stack split successfully!", "✓".green());
    println!("  Original stack: {} branches", original_config.branches.len());
    println!("  Split branches retargeted to {}", config.base_branch);

    Ok(())
}
