//! help.rs 
//! Implements the `help` command for the shell.

use zcore_drivers::println;

/// help command
/// 
/// `help` is a simple command that displays available commands and their usage.
/// `help` 
pub fn help(offset: usize) -> usize {
    let mut offset = offset;
    println!(offset, "Available commands:\n");
    println!(offset, "  help - Display this help message");
    println!(offset, "  echo <text> - Print text to the standard output");
    println!(offset, "  clear - Clears the screen, then redraws the prompt");
    println!(offset, "  version - Prints OS Information");
    offset
}