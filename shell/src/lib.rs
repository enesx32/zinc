#![no_std]

/// echo module
///
/// `echo` is a simple command that prints its arguments to the standard output.
/// This module takes 1 argument:
/// `echo <text>` - prints the text to the standard output
pub mod echo;

/// help module
/// 
/// `help` is a simple command that prints the list of available commands to the standard output.
/// This module takes no arguments:
/// `help` - prints the list of available commands
pub mod help;