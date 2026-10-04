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
    colors::{CYAN, GREEN, LIGHT_BLUE, LIGHT_RED, WHITE, LIGHT_GRAY}, keys::ENTER_SCANCODE,
};

use zcore_types::string::BasicString;
use brass::{echo::echo, help::help, version::version};

/// Number of rows the prompt takes up.
const PROMPT_ROWS: usize = 3;

/// How many spaces a tab key press inserts.
const TAB_WIDTH: usize = 4;

/// Maximum number of characters that can be stored in a command.
const MAX_COMMAND_LENGTH: usize = 128;

/// Number of columns used to indent command output.
const COMMAND_OUTPUT_INDENT: usize = 0;

/// Kernel entry point.
///
/// The bootloader jumps here after entering 64-bit mode.
/// This function never returns.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    sleep_millis(800);

    clear_screen(0);

    let mut offset = 0;

    let mut command = [0u8; MAX_COMMAND_LENGTH];

    let mut command_length = 0;

    println!(offset, "                                Zinc OS | v3.9.0", CYAN);
    println!(offset);
    println!(offset);

    println!(offset, "+- enesx32[/]", LIGHT_BLUE);
    println!(offset, "|", LIGHT_BLUE);
    print!(offset, "+---", LIGHT_BLUE);
    print!(offset, " $ ", GREEN);

    let mut input_start = offset;

    loop {
        if let Some(scancode) = read_key() {
            if scancode == ENTER_SCANCODE {
                offset = (offset / BUFFER_WIDTH + 1) * BUFFER_WIDTH + COMMAND_OUTPUT_INDENT;

                let command_str: &str = unsafe {
                    core::str::from_utf8_unchecked(&command[..command_length])
                };

                let command = BasicString::from(command_str);
                let split_command = command.split::<6>(b' ');

                if split_command[0].to_str() == "echo" {
                    for i in 1..split_command.len() {    
                        offset = echo(offset, split_command[i].to_str());
                    }
                } else if split_command[0].to_str() == "clear" {
                    clear_screen(0);
                    offset = 0;
                } else if split_command[0].to_str() == "help" {
                    offset = help(offset);
                } else if split_command[0].to_str() == "version" {
                    offset = version(offset)
                } else {
                    print!(offset, "Unknown command: ", LIGHT_RED);
                    print!(offset, command.to_str(), WHITE);
                    println!(offset);
                    print!(offset, "Run ", WHITE);
                    print!(offset, "help", LIGHT_GRAY);
                    print!(offset, " to display commands and how to use them", WHITE);
                    println!(offset);
                }

                offset = (offset / BUFFER_WIDTH + 2) * BUFFER_WIDTH;

                let rows = scroll_to_fit(offset, PROMPT_ROWS * BUFFER_WIDTH);
                offset -= rows * BUFFER_WIDTH;

                println!(offset, "+- enesx32[/]", LIGHT_BLUE);
                println!(offset, "|", LIGHT_BLUE);
                print!(offset, "+---", LIGHT_BLUE);
                print!(offset, " $ ", GREEN);

                command_length = 0;

                input_start = offset;
            } else if let Some(character) = scancode_to_ascii(scancode) {
                if character == 8 {
                    if offset > input_start && command_length > 0 {
                        offset -= 1;
                        blank_cell(offset);
                        command_length -= 1;
                    }
                } else if character == 9 {
                    if command_length + TAB_WIDTH <= MAX_COMMAND_LENGTH {
                        let mut count = 0;

                        while count < TAB_WIDTH {
                            write_cell(offset, b' ', WHITE);
                            offset += 1;
                            count += 1;
                        }

                        let mut count = 0;

                        while count < TAB_WIDTH {
                            command[command_length] = b' ';
                            command_length += 1;
                            count += 1;
                        }
                    }
                } else if command_length < MAX_COMMAND_LENGTH {
                    command[command_length] = character;
                    command_length += 1;

                    write_cell(offset, character, WHITE);
                    offset += 1;
                }
            }
        }

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