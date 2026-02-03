use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = ".gh-flow.json";

#[derive(Debug, Serialize, Deserialize, Default)]
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

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Get config file path
    fn config_path() -> Result<PathBuf> {
        let cwd = std::env::current_dir()?;
        Ok(cwd.join(CONFIG_FILE))
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

    /// Get all branches in dependency order
    #[allow(dead_code)]
    pub fn ordered_branches(&self) -> Vec<&BranchInfo> {
        // TODO: Implement proper topological sort
        // For now, just return in order
        self.branches.iter().collect()
    }
}
