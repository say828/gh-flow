use crate::git;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = "gh-flow.json";

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

impl StackConfig {
    /// Load configuration from file (legacy)
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

        // Get all commits from base to current branch
        let log_output = git::run(&[
            "log",
            "--oneline",
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

        // Get commit hashes
        let commits: Vec<&str> = log_output.lines().collect();

        // Get all local branches and their HEAD commits
        let branches_output = git::run(&[
            "for-each-ref",
            "--format=%(refname:short) %(objectname:short)",
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
        let commit_hashes: Vec<&str> = commits.iter()
            .filter_map(|line| line.split_whitespace().next())
            .collect();

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
        for commit_line in commits.iter().rev() {
            let commit_hash = commit_line.split_whitespace().next().unwrap_or("");
            if let Some(branch) = commit_to_branch.get(commit_hash) {
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
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Get config file path (stored in .git directory)
    fn config_path() -> Result<PathBuf> {
        let git_dir = git::run(&["rev-parse", "--git-dir"])
            .context("Not in a git repository")?;
        Ok(PathBuf::from(git_dir).join(CONFIG_FILE))
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
