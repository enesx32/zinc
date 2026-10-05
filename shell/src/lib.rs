#![no_std]

/// echo module
///
/// `echo` is a simple command that prints its arguments to the standard output.
/// This module takes 1 argument:
/// `echo <text>`
pub mod echo;

/// help module
///
/// `help` is a simple command that prints the list of available commands to the standard output.
/// This module takes no arguments:
/// `help`
pub mod help;

/// version module
///
/// `version` is a simple command that prints the version and a few other details to
/// the standard output
/// `version`
pub mod version;

/// list module
///
/// `list` is a simple command that prints the contents of the current directory.
/// This module takes no arguments:
/// `list`
pub mod list;

/// cd module
///
/// `cd` is a simple command that changes the current directory.
/// This module takes 1 argument:
/// `cd <directory>`
pub mod cd;