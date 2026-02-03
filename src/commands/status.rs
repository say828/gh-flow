use anyhow::Result;
use colored::Colorize;

pub fn run() -> Result<()> {
    println!("{}", "Stack Status".green().bold());
    println!();

    // TODO: Implement status display
    // - Read stack configuration
    // - Get all branches in stack
    // - Show PR status for each
    // - Indicate which branch is current

    println!("No stack found. Run `gh flow init` first.");
    Ok(())
}
