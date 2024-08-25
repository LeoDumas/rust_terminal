use std;
mod utils;
use crate::utils::clear_terminal;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    clear_terminal::clear_terminal();
    loop_user_input()
}

fn loop_user_input() -> io::Result<()> {
    loop {
        let current_dir = get_current_dir()?;
        print!("{}> ", current_dir.display());
        io::stdout().flush()?; // Ensure the prompt is displayed immediately

        let mut user_command = String::new();
        io::stdin().read_line(&mut user_command)?;

        let user_command = user_command.trim(); // Remove whitespace

        if user_command.is_empty() {
            // If the user just pressed Enter, show the current directory
            println!("");
        } else if user_command == "exit" {
            // Allow the user to exit the loop
            println!("Exiting...");
            break;
        } else if user_command == "clear" {
            clear_terminal::clear_terminal();
        } else {
            println!("Unknown command: {}", user_command);
        }
    }
    Ok(())
}

fn get_current_dir() -> io::Result<PathBuf> {
    std::env::current_dir()
}
