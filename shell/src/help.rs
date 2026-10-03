//! help.rs 
//! Implements the `help` command for the shell.

use zcore_drivers::{
    vga_buffer::WHITE,
    println
};

/// help command
/// 
/// `help` is a simple command that displays available commands and their usage.
/// `help` 
pub fn help(offset: usize) -> usize {
    let mut offset = offset;
    println!(offset, "Available commands:\n", WHITE);
    println!(offset, "  echo <text> - Print text to the standard output\n", WHITE);
    println!(offset, "  help - Display this help message\n", WHITE);
    offset
}