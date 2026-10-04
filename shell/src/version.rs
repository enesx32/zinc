//! version.rs 
//! Implements the `version` command for the shell.

use zcore_drivers::{ println, print };
use zcore_constants::colors::{ BROWN, GREEN, LIGHT_CYAN, LIGHT_GRAY, LIGHT_GREEN, LIGHT_RED };

/// version command
/// 
/// `version` is a simple command that uses no arguments:
/// `echo` - prints the information of the OS to the standard output
pub fn version(offset: usize) -> usize {
    let mut offset = offset;
    print!(offset, "   Operating system  | ");
    print!(offset, "Zinc OS", LIGHT_CYAN); 
    println!(offset);
    print!(offset, "   Kernel            | ");
    print!(offset, "Zink", LIGHT_CYAN);
    println!(offset); 
    print!(offset, "   Shell             | ");
    print!(offset, "Brass", BROWN);
    println!(offset, "\n");
    print!(offset, "   Version           | ");
    print!(offset, "3", LIGHT_GREEN);
    print!(offset, ".");
    print!(offset, "9", GREEN);
    print!(offset, ".");
    print!(offset, "1", LIGHT_RED);
    println!(offset);
    print!(offset, "   Architecture      | ");
    print!(offset, "x86-64", LIGHT_GRAY);
    offset
}