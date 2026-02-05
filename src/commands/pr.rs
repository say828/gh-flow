use crate::{git, github, stack::StackConfig};
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

const PR_TEMPLATE_FILE: &str = "gh-flow-pr-template.md";

fn load_pr_template() -> Option<String> {
    let git_dir = git::run(&["rev-parse", "--git-dir"]).ok()?;
    let template_path = PathBuf::from(git_dir).join(PR_TEMPLATE_FILE);
    fs::read_to_string(template_path).ok()
}

fn generate_stack_visualization(config: &StackConfig, current_branch: &str) -> String {
    let mut stack_viz = String::from("```\n");
    stack_viz.push_str(&format!("{} (base)\n", config.base_branch));

    for (idx, branch) in config.branches.iter().enumerate() {
        let marker = if branch.name == current_branch {
            "▶"
        } else {
            " "
        };
        let pr_info = if let Some(pr_num) = branch.pr_number {
            format!(" PR #{}", pr_num)
        } else {
            String::new()
        };

        stack_viz.push_str(&format!("{}└─ {}{}\n", marker, branch.name, pr_info));

        if idx < config.branches.len() - 1 {
            stack_viz.push_str("  ↓\n");
        }
    }
    stack_viz.push_str("```");

    // Load template from file or use default
    if let Some(tmpl) = load_pr_template() {
        tmpl.replace("{{stack}}", &stack_viz)
            .replace("{{branch}}", current_branch)
    } else {
        // Default template
        format!(
            "## Stack\n\n{}\n\n*This PR is part of a stack. Use [gh-flow](https://github.com/say828/gh-flow) to manage stacked PRs.*\n",
            stack_viz
        )
    }
}

pub fn create(draft: bool) -> Result<()> {
    println!("{}", "Creating PRs for stack...".green().bold());

    if draft {
        println!("{}", "PRs will be created as drafts".dimmed());
    }
    println!();

    // Load configuration
    let mut config = StackConfig::load().context("Failed to load configuration")?;

    if config.branches.is_empty() {
        println!(
            "{}",
            "No branches in stack. Run `gh flow init` first.".yellow()
        );
        return Ok(());
    }

    let mut created_count = 0;
    let mut skipped_count = 0;

    for i in 0..config.branches.len() {
        let branch_name = config.branches[i].name.clone();
        let parent_name = config.branches[i].parent.clone();

        print!("Creating PR for {} ... ", branch_name.cyan());

        // Check if branch exists
        if !git::branch_exists(&branch_name)? {
            println!("{}", "skipped (branch doesn't exist)".yellow());
            skipped_count += 1;
            continue;
        }

        // Check if PR already exists
        if let Ok(Some(existing_pr)) = github::get_pr(&branch_name) {
            println!(
                "{} {}",
                "exists".yellow(),
                format!("(PR #{})", existing_pr.number).dimmed()
            );
            config.branches[i].pr_number = Some(existing_pr.number);
            skipped_count += 1;
            continue;
        }

        // Get commit message for PR title
        let title = git::run(&["log", "-1", "--pretty=%s", &branch_name])
            .unwrap_or_else(|_| format!("Changes in {}", branch_name));

        // Generate PR body with stack visualization
        let body = generate_stack_visualization(&config, &branch_name);

        // Create PR
        match github::create_pr(&branch_name, &parent_name, &title, &body, draft) {
            Ok(pr_number) => {
                println!("{} {}", "✓".green(), format!("PR #{}", pr_number).green());
                config.branches[i].pr_number = Some(pr_number);
                created_count += 1;
            }
            Err(e) => {
                println!("{} {}", "✗".red(), e);
            }
        }
    }

    // Save updated configuration
    config.save().context("Failed to save configuration")?;

    println!();
    println!(
        "{} {} PRs created, {} skipped",
        "✓".green(),
        created_count,
        skipped_count
    );

    Ok(())
}

pub fn update() -> Result<()> {
    println!("{}", "Updating PRs...".green().bold());
    println!();

    // Load configuration
    let config = StackConfig::load().context("Failed to load configuration")?;

    if config.branches.is_empty() {
        println!("{}", "No branches in stack.".yellow());
        return Ok(());
    }

    let mut updated_count = 0;

    for branch_info in &config.branches {
        if let Some(pr_number) = branch_info.pr_number {
            print!(
                "Updating PR #{} ({}) ... ",
                pr_number,
                branch_info.name.cyan()
            );

            // Generate updated stack visualization
            let body = generate_stack_visualization(&config, &branch_info.name);

            match github::update_pr_body(pr_number, &body) {
                Ok(_) => {
                    println!("{}", "✓".green());
                    updated_count += 1;
                }
                Err(e) => {
                    println!("{} {}", "✗".red(), e);
                }
            }
        }
    }

    println!();
    println!("{} {} PRs updated", "✓".green(), updated_count);

    Ok(())
}
