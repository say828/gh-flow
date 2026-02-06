use crate::git;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = "gh-flow.json";
const PR_TEMPLATE_FILE: &str = "pr-template.md";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StackConfig {
    pub base_branch: String,
    pub branches: Vec<BranchInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub parent: String,
    pub pr_number: Option<u32>,
}

/// Get repository identifier (owner/repo) from git remote
pub fn get_repo_identifier() -> Result<String> {
    let remote_url = git::run(&["remote", "get-url", "origin"])
        .context("Failed to get remote URL. Is this a git repository with an origin remote?")?;

    // Parse owner/repo from various URL formats:
    // https://github.com/owner/repo.git
    // git@github.com:owner/repo.git
    // https://github.com/owner/repo
    let url = remote_url.trim();

    let path = if url.contains("github.com:") {
        // SSH format: git@github.com:owner/repo.git
        url.split("github.com:").last().unwrap_or("")
    } else if url.contains("github.com/") {
        // HTTPS format: https://github.com/owner/repo.git
        url.split("github.com/").last().unwrap_or("")
    } else {
        // Try to extract from any URL
        url.rsplit('/').take(2).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("/").as_str().to_string().leak()
    };

    let repo_id = path.trim_end_matches(".git").to_string();

    if repo_id.is_empty() || !repo_id.contains('/') {
        anyhow::bail!("Could not parse repository identifier from remote URL: {}", url);
    }

    Ok(repo_id)
}

/// Get global config directory (~/.config/gh-flow/)
pub fn get_global_config_dir() -> Result<PathBuf> {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".config")
        });

    Ok(config_home.join("gh-flow"))
}

/// Get repository-specific config directory (~/.config/gh-flow/repos/owner/repo/)
pub fn get_repo_config_dir() -> Result<PathBuf> {
    let global_dir = get_global_config_dir()?;
    let repo_id = get_repo_identifier()?;
    Ok(global_dir.join("repos").join(repo_id))
}

/// Get PR template path (repo-specific or global)
pub fn get_pr_template_path() -> Result<Option<PathBuf>> {
    // Try repo-specific template first
    if let Ok(repo_dir) = get_repo_config_dir() {
        let repo_template = repo_dir.join(PR_TEMPLATE_FILE);
        if repo_template.exists() {
            return Ok(Some(repo_template));
        }
    }

    // Fall back to global template
    if let Ok(global_dir) = get_global_config_dir() {
        let global_template = global_dir.join(PR_TEMPLATE_FILE);
        if global_template.exists() {
            return Ok(Some(global_template));
        }
    }

    Ok(None)
}

/// Load PR template content
pub fn load_pr_template() -> Option<String> {
    get_pr_template_path()
        .ok()
        .flatten()
        .and_then(|path| fs::read_to_string(path).ok())
}

impl StackConfig {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Auto-discover branch chain from git history
    /// This is the primary method - always gets fresh state
    pub fn discover(base_branch: &str) -> Result<Self> {
        let current = git::current_branch()?;

        // Get all commits from base to current branch (full hashes for reliable matching)
        let log_output = git::run(&[
            "log",
            "--format=%H",
            "--first-parent",
            &format!("{}..HEAD", base_branch),
        ])?;

        if log_output.is_empty() {
            let mut config = Self {
                base_branch: base_branch.to_string(),
                branches: Vec::new(),
            };
            if current != base_branch {
                config.add_branch(current, base_branch.to_string());
            }
            return Ok(config);
        }

        // Each line is a full commit hash
        let commit_hashes: Vec<&str> = log_output.lines().collect();

        // Get all local branches and their HEAD commits (full hashes)
        let branches_output = git::run(&[
            "for-each-ref",
            "--format=%(refname:short) %(objectname)",
            "refs/heads/",
        ])?;

        let mut commit_to_branch: HashMap<String, String> = HashMap::new();
        let mut commit_to_branches: HashMap<String, Vec<String>> = HashMap::new();

        for line in branches_output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let branch = parts[0].to_string();
                let commit = parts[1].to_string();
                if branch != base_branch {
                    commit_to_branch.insert(commit.clone(), branch.clone());
                    commit_to_branches.entry(commit).or_default().push(branch);
                }
            }
        }

        // Check for duplicate branches pointing to same commit in our chain
        for commit in &commit_hashes {
            if let Some(branches) = commit_to_branches.get(*commit) {
                if branches.len() > 1 {
                    anyhow::bail!(
                        "Multiple branches point to same commit ({}):\n  {}\n\nPlease delete or rename one of these branches.",
                        &commit[..7.min(commit.len())],
                        branches.join("\n  ")
                    );
                }
            }
        }

        // Find branches in order (from oldest to newest commit)
        let mut chain: Vec<String> = Vec::new();
        for commit in commit_hashes.iter().rev() {
            if let Some(branch) = commit_to_branch.get(*commit) {
                if !chain.contains(branch) {
                    chain.push(branch.clone());
                }
            }
        }

        // Make sure current branch is included
        if !chain.contains(&current) {
            chain.push(current);
        }

        // Build config
        let mut config = Self {
            base_branch: base_branch.to_string(),
            branches: Vec::new(),
        };

        let mut prev = base_branch.to_string();
        for branch in chain {
            config.add_branch(branch.clone(), prev);
            prev = branch;
        }

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Get config file path (~/.config/gh-flow/repos/owner/repo/gh-flow.json)
    fn config_path() -> Result<PathBuf> {
        let repo_dir = get_repo_config_dir()?;
        Ok(repo_dir.join(CONFIG_FILE))
    }

    /// Add a branch to the stack
    pub fn add_branch(&mut self, name: String, parent: String) {
        self.branches.push(BranchInfo {
            name,
            parent,
            pr_number: None,
        });
    }

    /// Get branch info
    #[allow(dead_code)]
    pub fn get_branch(&self, name: &str) -> Option<&BranchInfo> {
        self.branches.iter().find(|b| b.name == name)
    }

    /// Get branch info mutably
    #[allow(dead_code)]
    pub fn get_branch_mut(&mut self, name: &str) -> Option<&mut BranchInfo> {
        self.branches.iter_mut().find(|b| b.name == name)
    }
}
