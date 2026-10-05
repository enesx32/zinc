//! cd.rs
//! Implements the `cd` command for the shell.

use zcore_fs::change_directory;

/// cd command
///
/// `cd` is a simple command that changes the current directory.
/// This module takes 1 argument:
/// `cd <directory>`
pub fn cd(current_sector: u32, target: &str) -> Option<u32> {
    change_directory(current_sector, target)
}