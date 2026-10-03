//! Zinc OS kernel entry point.
//!

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use zcore_drivers::{
    keyboard::{read_key, scancode_to_ascii},
    vga_buffer::*,
    time::sleep_millis,
    println, print,
};

use zcore_constants::{
    colors::{CYAN, GREEN, LIGHT_BLUE, RED, WHITE, LIGHT_GRAY}, keys::ENTER_SCANCODE,
};

use zcore_types::string::BasicString;
use brass::{ echo::echo, help::help };

/// Number of rows the prompt takes up.
const PROMPT_ROWS: usize = 3;

/// How many spaces a tab key press inserts.
const TAB_WIDTH: usize = 4;

/// Maximum number of characters that can be stored in a command.
const MAX_COMMAND_LENGTH: usize = 128;

/// Kernel entry point.
///
/// The bootloader jumps here after entering 64-bit mode.
/// This function never returns.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Give the hardware a short moment before starting the kernel.
    sleep_millis(800);

    // Clear the screen so the bootloader's messages are removed.
    clear_screen(0);

    // Current position of the cursor in the VGA buffer.
    let mut offset = 0;

    // Store the characters typed by the user.
    let mut command = [0u8; MAX_COMMAND_LENGTH];

    // Number of characters currently in the command.
    let mut command_length = 0;

    // Print the Zinc OS banner.
    println!(offset, "                 Zinc OS | v3.3.0", CYAN);
    println!(offset);
    println!(offset);

    // Print the first shell prompt.
    println!(offset, "+- enesx32[/]", LIGHT_BLUE);
    println!(offset, "|", LIGHT_BLUE);
    print!(offset, "+---", LIGHT_BLUE);
    print!(offset, " $ ", GREEN);

    // Remember where the user's input starts.
    // Backspace cannot move before this position.
    let mut input_start = offset;

    loop {
        // Check if a key has been pressed.
        if let Some(scancode) = read_key() {
            // Enter executes the current command.
            if scancode == ENTER_SCANCODE {
                // Move to the next row.
                offset = (offset / BUFFER_WIDTH + 2) * BUFFER_WIDTH;

                // Parse the command into a BasicString and split it into parts.
                let command_str: &str = unsafe { core::str::from_utf8_unchecked(&command[..command_length]) };
                let command = BasicString::from(command_str);
                let split_command = command.split::<6>(b' ');

                if split_command[0].to_str() == "echo" {
                    echo(offset, split_command[1].to_str());
                } else if split_command[0].to_str() == "clear" {
                    clear_screen(0);
                } else if split_command[0].to_str() == "help" {
                    help(offset);
                } else {
                    print!(offset, "Unknown command: {}\n", RED);
                    print!(offset, command, WHITE);
                    println!(offset, "\n", WHITE);
                    print!(offset, "Run ", WHITE);
                    print!(offset, "help", LIGHT_GRAY);
                    print!(offset, "to display commands and how to use them", WHITE);
                }

                // Move to the next row.
                offset = (offset / BUFFER_WIDTH + 3) * BUFFER_WIDTH;

                // Make sure there is room for the next prompt.
                let rows = scroll_to_fit(offset, PROMPT_ROWS * BUFFER_WIDTH);
                offset -= rows * BUFFER_WIDTH;

                // Print a new prompt.
                println!(offset, "+- enesx32[/]", LIGHT_BLUE);
                println!(offset, "|", LIGHT_BLUE);
                print!(offset, "+---", LIGHT_BLUE);
                print!(offset, " $ ", GREEN);

                // Reset the command buffer for the next command.
                command_length = 0;

                // Remember where the new input starts.
                input_start = offset;
            } else if let Some(character) = scancode_to_ascii(scancode) {
                // Backspace removes the previous character.
                if character == 8 {
                    if offset > input_start && command_length > 0 {
                        offset -= 1;
                        blank_cell(offset);
                        command_length -= 1;
                    }
                } else if character == 9 {
                    // Tab inserts four spaces.
                    if command_length + TAB_WIDTH <= MAX_COMMAND_LENGTH {
                        let mut count = 0;

                        while count < TAB_WIDTH {
                            write_cell(offset, b' ', WHITE);
                            offset += 1;
                            count += 1;
                        }

                        // Store the spaces in the command.
                        let mut count = 0;

                        while count < TAB_WIDTH {
                            command[command_length] = b' ';
                            command_length += 1;
                            count += 1;
                        }
                    }
                } else if command_length < MAX_COMMAND_LENGTH {
                    // Store the character.
                    command[command_length] = character;
                    command_length += 1;

                    // Print the character immediately.
                    write_cell(offset, character, WHITE);
                    offset += 1;
                }
            }
        }

        // Tell the CPU that we are waiting in a busy loop.
        core::hint::spin_loop();
    }
}

/// Called when the kernel panics.
///
/// Zinc does not have panic output yet,
/// so the CPU simply stays here.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}