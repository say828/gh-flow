use anyhow::{Context, Result};
use std::process::Command;

/// Run a git command and return output
pub fn run(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git command failed: {}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Get current branch name
pub fn current_branch() -> Result<String> {
    run(&["branch", "--show-current"])
}

/// Get list of all local branches
pub fn list_branches() -> Result<Vec<String>> {
    let output = run(&["branch", "--format=%(refname:short)"])?;
    Ok(output.lines().map(String::from).collect())
}

/// Check if branch exists
pub fn branch_exists(branch: &str) -> Result<bool> {
    let branches = list_branches()?;
    Ok(branches.contains(&branch.to_string()))
}

/// Rebase current branch onto target
pub fn rebase(target: &str) -> Result<()> {
    run(&["rebase", target])?;
    Ok(())
}

/// Push branch to remote
pub fn push(branch: &str, force: bool) -> Result<()> {
    let mut args = vec!["push", "origin", branch];
    if force {
        args.push("--force-with-lease");
    }
    run(&args)?;
    Ok(())
}
