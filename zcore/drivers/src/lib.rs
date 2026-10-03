#![no_std]

/// VGA text-mode display driver.
///
/// Provides the VGA buffer, colours, screen clearing,
/// text writing, and printing macros.
pub mod vga_buffer;

/// PS/2 keyboard driver.
///
/// Provides functions for reading keyboard scancodes
/// and converting them into ASCII characters.
pub mod keyboard;

/// PIT Chip Drivers.
/// 
/// Provides functions for reading and writing to the PIT chip.
/// and functions for sleeping the CPU for a given amount of time.
pub mod time;