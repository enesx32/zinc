//! list.rs
//! Implements the `list` command for the shell.

use zcore_constants::colors::{LIGHT_BLUE, LIGHT_GRAY, LIGHT_RED};
use zcore_drivers::{ print, println };
use zcore_fs::read_directory;

/// ls command
///
/// `ls` is a simple command that prints the contents of the current directory.
/// This module takes no arguments:
/// `ls`
pub fn ls(offset: usize, directory_sector: u32) -> usize {
    let mut offset = offset;

    let entries = match read_directory(directory_sector) {
        Some(entries) => entries,
        None => {
            println!(offset, "ls: unable to read directory", LIGHT_RED);
            return offset;
        }
    };

    let mut found = false;

    for entry in entries.iter() {
        if !entry.is_used() {
            continue;
        }

        let name = match entry.name_str() {
            Some(name) => name,
            None => continue,
        };

        if name == "." || name == ".." {
            continue;
        }

        found = true;

        if entry.is_directory() {
            print!(offset, name, LIGHT_BLUE);
            println!(offset, "/", LIGHT_BLUE);
        } else {
            println!(offset, name, LIGHT_GRAY);
        }
    }

    if !found {
        println!(offset, "(empty)", LIGHT_GRAY);
    }

    offset
}