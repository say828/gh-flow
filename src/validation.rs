use anyhow::{bail, Result};
use colored::*;
use std::process::Command;

#[allow(dead_code)]
pub fn check_git_repo() -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()?;

    if !output.status.success() {
        bail!(
            "{} Not a git repository\n\n{}\n  Run this command inside a git repository",
            "Error:".red().bold(),
            "Suggestion:".yellow()
        );
    }
    Ok(())
}

#[allow(dead_code)]
pub fn check_gh_auth() -> Result<()> {
    let output = Command::new("gh").args(["auth", "status"]).output()?;

    if !output.status.success() {
        bail!(
            "{} GitHub CLI not authenticated\n\n{}\n  Run: gh auth login",
            "Error:".red().bold(),
            "Suggestion:".yellow()
        );
    }
    Ok(())
}

#[allow(dead_code)]
pub fn check_uncommitted_changes() -> Result<bool> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;

    Ok(!output.stdout.is_empty())
}

#[allow(dead_code)]
pub fn warn_uncommitted_changes() -> Result<()> {
    if check_uncommitted_changes()? {
        println!(
            "{} You have uncommitted changes\n",
            "Warning:".yellow().bold()
        );
    }
    Ok(())
}

#[allow(dead_code)]
pub fn check_branch_exists(branch: &str) -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", branch])
        .output()?;

    Ok(output.status.success())
}
