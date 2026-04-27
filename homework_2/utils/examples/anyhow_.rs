use anyhow::{Context, Result};
use std::fs::read_to_string;

fn main() -> Result<()> {
    let content = read_config("config.toml").context("Could not read a config file")?; // adding a context

    println!("Content: {}", content);
    Ok(())
}

fn read_config(path: &str) -> Result<String> {
    if path.is_empty() {
        anyhow::bail!("File path cannot be empty"); // creating an error
    }
    let data = read_to_string(path)?;
    Ok(data)
}
