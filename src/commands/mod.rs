use anyhow::Result;
use std::path::PathBuf;

mod commands;

pub fn execute_command(command: &str, files: &Vec<PathBuf>) -> Result<()> {
    match command {
        "new" => commands::upload(files)?,
        _ => show_command_not_found(command)
    }

    Ok(())
}

fn show_command_not_found(command: &str) {
    println!("Command {} does not exist", command);
}