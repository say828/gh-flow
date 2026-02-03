use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: u32,
    pub title: String,
    #[serde(rename = "baseRefName")]
    pub base_ref: String,
    #[serde(rename = "headRefName")]
    pub head_ref: String,
    pub state: String,
}

/// Run gh CLI command and return output
fn run_gh(args: &[&str]) -> Result<String> {
    let output = Command::new("gh")
        .args(args)
        .output()
        .context("Failed to execute gh command. Is gh CLI installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("gh command failed: {}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Get PR for a branch
pub fn get_pr(branch: &str) -> Result<Option<PullRequest>> {
    let output = run_gh(&[
        "pr",
        "list",
        "--head",
        branch,
        "--json",
        "number,title,baseRefName,headRefName,state",
        "--limit",
        "1",
    ])?;

    let prs: Vec<PullRequest> = serde_json::from_str(&output).context("Failed to parse PR JSON")?;

    Ok(prs.into_iter().next())
}

/// Create a new PR
pub fn create_pr(head: &str, base: &str, title: &str, body: &str, draft: bool) -> Result<u32> {
    let mut args = vec![
        "pr", "create", "--head", head, "--base", base, "--title", title, "--body", body,
    ];

    if draft {
        args.push("--draft");
    }

    let output = run_gh(&args)?;

    // Parse PR URL to get number
    // gh CLI outputs: https://github.com/owner/repo/pull/123
    if let Some(pr_number_str) = output.split('/').next_back() {
        if let Ok(pr_number) = pr_number_str.trim().parse::<u32>() {
            return Ok(pr_number);
        }
    }

    // If parsing fails, try to get it from gh api
    // Query the PR that was just created
    if let Ok(Some(pr)) = get_pr(head) {
        return Ok(pr.number);
    }

    anyhow::bail!("Failed to parse PR number from output: {}", output)
}

/// Update PR base branch
pub fn update_pr_base(pr_number: u32, new_base: &str) -> Result<()> {
    run_gh(&["pr", "edit", &pr_number.to_string(), "--base", new_base])?;
    Ok(())
}

/// Update PR body
pub fn update_pr_body(pr_number: u32, body: &str) -> Result<()> {
    run_gh(&["pr", "edit", &pr_number.to_string(), "--body", body])?;
    Ok(())
}

/// Get PR status (OPEN, MERGED, CLOSED)
pub fn get_pr_status(pr_number: u32) -> Result<String> {
    let output = Command::new("gh")
        .args([
            "pr", "view", &pr_number.to_string(),
            "--json", "state",
            "--jq", ".state"
        ])
        .output()
        .context("Failed to get PR status")?;

    if output.status.success() {
        let status = String::from_utf8_lossy(&output.stdout).trim().to_uppercase();
        Ok(if status.is_empty() { "UNKNOWN".to_string() } else { status })
    } else {
        Ok("UNKNOWN".to_string())
    }
}

/// Get CI status (SUCCESS, PENDING, FAILURE)
pub fn get_ci_status(pr_number: u32) -> Result<String> {
    let output = Command::new("gh")
        .args([
            "pr", "view", &pr_number.to_string(),
            "--json", "statusCheckRollup",
            "--jq", ".statusCheckRollup[0].conclusion // \"PENDING\""
        ])
        .output()
        .context("Failed to get CI status")?;

    if output.status.success() {
        let status = String::from_utf8_lossy(&output.stdout).trim().to_uppercase();
        Ok(if status.is_empty() { "PENDING".to_string() } else { status })
    } else {
        Ok("UNKNOWN".to_string())
    }
}

/// Get review status (APPROVED, PENDING, CHANGES_REQUESTED)
pub fn get_review_status(pr_number: u32) -> Result<String> {
    let output = Command::new("gh")
        .args([
            "pr", "view", &pr_number.to_string(),
            "--json", "reviewDecision",
            "--jq", ".reviewDecision // \"PENDING\""
        ])
        .output()
        .context("Failed to get review status")?;

    if output.status.success() {
        let status = String::from_utf8_lossy(&output.stdout).trim().to_uppercase();
        Ok(if status.is_empty() { "PENDING".to_string() } else { status })
    } else {
        Ok("UNKNOWN".to_string())
    }
}
