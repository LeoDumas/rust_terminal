mod commands;
mod utils;

use crate::commands::loop_user_input;
use crate::utils::clear_terminal;
use std::io;

fn main() -> io::Result<()> {
    clear_terminal::clear_terminal();
    loop_user_input()
}
