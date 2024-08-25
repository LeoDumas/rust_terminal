mod file_system;

use crate::utils::clear_terminal;
use std::io::{self, Write};

pub use file_system::{change_directory, get_current_dir, list_directory};

pub fn loop_user_input() -> io::Result<()> {
    loop {
        let current_dir = get_current_dir()?;
        print!("{}> ", current_dir.display());
        io::stdout().flush()?;

        let mut user_command = String::new();
        io::stdin().read_line(&mut user_command)?;

        let user_command = user_command.trim();

        if let Err(e) = execute_command(user_command) {
            eprintln!("Error: {}", e);
        }
    }
}

pub fn execute_command(command: &str) -> io::Result<()> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    match parts.get(0) {
        Some(&"cd") => {
            if let Some(path) = parts.get(1) {
                change_directory(path)
            } else {
                println!("Usage: cd <directory>");
                Ok(())
            }
        }
        Some(&"ls") => list_directory(),
        Some(&"clear") => {
            clear_terminal::clear_terminal();
            Ok(())
        }
        Some(&"exit") => {
            println!("Exiting...");
            std::process::exit(0);
        }
        Some(cmd) => {
            println!("Unknown command: {}", cmd);
            Ok(())
        }
        None => Ok(()),
    }
}
