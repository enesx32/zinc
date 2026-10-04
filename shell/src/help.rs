//! help.rs 
//! Implements the `help` command for the shell.

use zcore_constants::colors::LIGHT_GRAY;
use zcore_drivers::{ println, print };

/// help command
/// 
/// `help` is a simple command that displays available commands and their usage.
/// `help`
pub fn help(offset: usize) -> usize {
    let mut offset = offset;

    println!(offset, "Available commands:\n");

    print!(offset, "  help", LIGHT_GRAY);
    println!(offset, " - Display this help message");

    print!(offset, "  echo <text>", LIGHT_GRAY);
    println!(offset, " - Print text to the standard output");

    print!(offset, "  clear", LIGHT_GRAY);
    println!(offset, " - Clears the screen, then redraws the prompt");

    print!(offset, "  version", LIGHT_GRAY);
    println!(offset, " - Prints OS Information");

    offset
}