//! echo.rs 
//! Implements the `echo` command for the shell.

use zcore_drivers::{
    vga_buffer::WHITE,
    println
};

/// echo command
/// 
/// `echo` is a simple command that uses 1 or 2 arguments:
/// `echo <text>` - prints the text to the standard output
pub fn echo(offset: usize, text: &str) -> usize {
    let mut offset = offset;
    println!(offset, text, WHITE);
    offset
}