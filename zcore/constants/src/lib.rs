#![no_std]

/// Keyboard scancode constants. 
/// These are the raw values sent by the keyboard hardware when a key is pressed or released. 
/// They are used to identify which key was pressed, and can be converted to ASCII characters using the 
/// `scancode_to_ascii` function in the `zcore_drivers` crate.
pub mod keys;

/// Color constants for VGA text mode.
/// These are the raw values used by the VGA hardware to set the foreground and
/// background colors of text on the screen.
pub mod colors;
