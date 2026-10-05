//! Zinc OS kernel entry point.
//!
//! This file contains the main kernel code and starts the Brass shell.

#![no_std]
#![no_main]

// Allows Zinc to use Rust's heap-allocated types such as Vec and String.
extern crate alloc;

// Provides the panic information type required by the panic handler.
use core::panic::PanicInfo;

// Imports the colours used by the kernel and shell.
use zcore_constants::{
    colors::{ CYAN, GREEN, LIGHT_BLUE, LIGHT_GRAY, LIGHT_RED, WHITE },
    keys::ENTER_SCANCODE,
};

// Imports keyboard input, timing, VGA output, and the print macros.
use zcore_drivers::{
    keyboard::{ read_key, scancode_to_ascii },
    time::sleep_millis,
    vga_buffer::*,
    print, println,
};

// Imports Zinc's filesystem functions.
use zcore_fs::ROOT_DIRECTORY_SECTOR;

// Imports Zinc's global memory allocator.
use zcore_memory::ALLOCATOR;

// Imports Zinc's dynamic heap-backed String.
use zcore_types::string::String;

// Imports the commands provided by the Brass shell.
use brass::{
    cd::cd,
    echo::echo,
    help::help,
    list::ls,
    version::version,
};

/// Number of rows needed to display a new shell prompt.
const PROMPT_ROWS: usize = 3;

/// The number of spaces inserted when Tab is pressed.
const TAB_WIDTH: usize = 4;

/// The maximum number of characters allowed in one command.
const MAX_COMMAND_LENGTH: usize = 128;

/// The number of columns used to indent command output.
const COMMAND_OUTPUT_INDENT: usize = 0;

/// The number of sectors available on the test disk.
const TEST_DISK_SECTORS: u32 = 2048;

/// Updates the displayed current path after changing directories.
fn update_path(path: &mut String, target: &str) {
    // Go to the root path.
    if target == "/" {
        path.clear();
        path.push('/');
        return;
    }

    // Move back to the root path from a child directory.
    if target == ".." {
        path.clear();
        path.push('/');
        return;
    }

    // Remove the leading slash from an absolute path.
    let name = target.strip_prefix('/').unwrap_or(target);

    // Update the path.
    path.clear();
    path.push('/');
    path.push_str(name);
}

/// Kernel entry point.
///
/// The bootloader jumps here after entering 64-bit mode.
/// This function never returns.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Initialise Zinc's global memory allocator.
    unsafe {
        // Give the allocator the starting address and size of Zinc's heap.
        //
        // This allows heap-backed types such as String and Vec to allocate memory.
        ALLOCATOR.init(zcore_memory::HEAP_START, zcore_memory::HEAP_SIZE);
    }

    // Stores the current position of the cursor on the VGA screen.
    let mut offset = 0;

    // Clear the screen before starting the kernel.
    clear_screen(0);

    // Test the new dynamic String.
    let banner = String::from("Zinc OS | v4.0.2");

    // Show that the kernel has started.
    println!(offset, "Kernel started", GREEN);

    // Print the dynamic String to test that it works with the VGA driver.
    println!(offset, banner.to_str(), CYAN);

    // Wait briefly so the startup results can be seen.
    sleep_millis(1500);

    // Clear the startup messages from the screen.
    clear_screen(0);

    // Start writing from the top-left corner again.
    offset = 0;

    // Store the current directory sector.
    let mut current_directory = ROOT_DIRECTORY_SECTOR;

    // Store the current directory path.
    let mut current_path = String::from("/");

    // Display the Zinc OS banner.
    println!(offset, banner.to_str(), CYAN);

    // Add an empty line below the banner.
    println!(offset);

    // Add another empty line before the shell prompt.
    println!(offset);

    // Print the shell prompt header.
    print!(offset, "+- enesx32[", LIGHT_BLUE);

    // Print the current directory path.
    print!(offset, current_path.to_str(), LIGHT_BLUE);

    // Finish the shell prompt header.
    println!(offset, "]", LIGHT_BLUE);

    // Print the vertical part of the shell prompt.
    println!(offset, "|", LIGHT_BLUE);

    // Print the command prompt decoration.
    print!(offset, "+---", LIGHT_BLUE);

    // Print the shell input symbol.
    print!(offset, " $ ", GREEN);

    // Stores the command currently being typed.
    //
    // This is now a dynamic String instead of a fixed-size byte array.
    let mut command = String::with_capacity(MAX_COMMAND_LENGTH);

    // Stores where the current command starts on the VGA screen.
    let mut input_start = offset;

    // Keep the kernel running forever.
    loop {
        // Check whether a keyboard scancode is available.
        if let Some(scancode) = read_key() {
            // Check whether the user pressed Enter.
            if scancode == ENTER_SCANCODE {
                // Move the cursor to the beginning of the next line.
                offset = (offset / BUFFER_WIDTH + 1) * BUFFER_WIDTH + COMMAND_OUTPUT_INDENT;

                // Only process the command if something was actually typed.
                if !command.is_empty() {
                    // Split the command into up to six parts separated by spaces.
                    let split_command = command.split::<6>(b' ');

                    // Check whether the command is "echo".
                    if split_command[0].to_str() == "echo" {
                        // Loop through everything after "echo".
                        for i in 1..split_command.len() {
                            // Print each argument supplied to echo.
                            offset = echo(offset, split_command[i].to_str());
                        }

                    // Check whether the command is "clear".
                    } else if split_command[0].to_str() == "clear" {
                        // Clear the entire VGA screen.
                        clear_screen(0);

                        // Reset the cursor to the beginning of the screen.
                        offset = 0;

                    // Check whether the command is "help".
                    } else if split_command[0].to_str() == "help" {
                        // Run the help command.
                        offset = help(offset);

                    // Check whether the command is "version".
                    } else if split_command[0].to_str() == "version" {
                        // Run the version command.
                        offset = version(offset);

                    // Check whether the command is "ls".
                    } else if split_command[0].to_str() == "ls" {
                        // List the current directory.
                        offset = ls(offset, current_directory);

                    // Check whether the command is "cd".
                    } else if split_command[0].to_str() == "cd" {
                        // Make sure a directory was supplied.
                        if split_command.len() < 2 {
                            // Tell the user how to use cd.
                            println!(offset, "Usage: cd <directory>", LIGHT_RED);
                        } else {
                            // Get the requested directory.
                            let target = split_command[1].to_str();

                            // Try to change directory.
                            if let Some(directory) = cd(current_directory, target) {
                                // Store the new directory sector.
                                current_directory = directory;

                                // Update the displayed path.
                                update_path(&mut current_path, target);
                            } else {
                                // Tell the user that the directory was not found.
                                print!(offset, "cd: ", LIGHT_RED);

                                // Print the requested directory.
                                print!(offset, target, WHITE);

                                // Explain the error.
                                println!(offset, ": directory not found", LIGHT_RED);
                            }
                        }

                    // The command does not match any known command.
                    } else {
                        // Tell the user that the command was not found.
                        print!(offset, "Unknown command: ", LIGHT_RED);

                        // Print the command that the user entered.
                        print!(offset, command.to_str(), WHITE);

                        // Move to the next line.
                        println!(offset);

                        // Tell the user how to find the available commands.
                        print!(offset, "Run ", WHITE);

                        // Highlight the help command.
                        print!(offset, "help", LIGHT_GRAY);

                        // Explain what the help command does.
                        print!(offset, " to display commands and how to use them", WHITE);

                        // Move to the next line.
                        println!(offset);
                    }
                }

                // Remove the old command so the String can be reused.
                command.clear();

                // Move down enough space for the next shell prompt.
                offset = (offset / BUFFER_WIDTH + 2) * BUFFER_WIDTH;

                // Scroll the screen if there is not enough room for the prompt.
                let rows = scroll_to_fit(offset, PROMPT_ROWS * BUFFER_WIDTH);

                // Move the cursor back by the number of rows that were scrolled.
                offset -= rows * BUFFER_WIDTH;

                // Print the new shell prompt header.
                print!(offset, "+- enesx32[", LIGHT_BLUE);

                // Print the current directory path.
                print!(offset, current_path.to_str(), LIGHT_BLUE);

                // Finish the shell prompt header.
                println!(offset, "]", LIGHT_BLUE);

                // Print the vertical part of the new prompt.
                println!(offset, "|", LIGHT_BLUE);

                // Print the command prompt decoration.
                print!(offset, "+---", LIGHT_BLUE);

                // Print the shell input symbol.
                print!(offset, " $ ", GREEN);

                // Remember where the new command starts.
                input_start = offset;

            // The key was not Enter, so try converting it to a character.
            } else if let Some(character) = scancode_to_ascii(scancode) {

                // Check whether the character is Backspace.
                if character == 8 {
                    // Only delete something if the command is not empty.
                    if offset > input_start && !command.is_empty() {
                        // Move the cursor one cell backwards.
                        offset -= 1;

                        // Clear the character from the VGA screen.
                        blank_cell(offset);

                        // Remove the last character from the dynamic String.
                        command.pop();
                    }

                // Check whether the character is Tab.
                } else if character == 9 {
                    // Make sure the tab will not exceed the command limit.
                    if command.len() + TAB_WIDTH <= MAX_COMMAND_LENGTH {
                        // Start counting the spaces displayed on screen.
                        let mut count = 0;

                        // Insert four spaces into the VGA buffer.
                        while count < TAB_WIDTH {
                            // Write one blank space to the current cell.
                            write_cell(offset, b' ', WHITE);

                            // Move the cursor forward.
                            offset += 1;

                            // Increase the space counter.
                            count += 1;
                        }

                        // Reset the counter for the String.
                        let mut count = 0;

                        // Add the same four spaces to the command String.
                        while count < TAB_WIDTH {
                            // Store one space in the dynamic String.
                            command.push(' ');

                            // Increase the space counter.
                            count += 1;
                        }
                    }

                // Handle a normal character.
                } else if command.len() < MAX_COMMAND_LENGTH {
                    // Add the character to Zinc's dynamic String.
                    command.push(character as char);

                    // Display the character on the VGA screen.
                    write_cell(offset, character, WHITE);

                    // Move the cursor forward.
                    offset += 1;
                }
            }
        }

        // Tell the CPU that the loop is intentionally waiting.
        core::hint::spin_loop();
    }
}

/// Called when the kernel panics.
///
/// Zinc does not have panic output yet,
/// so the CPU simply stays here.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Keep the CPU here instead of returning from the panic handler.
    loop {
        // Wait without doing unnecessary work.
        core::hint::spin_loop();
    }
}