//! Zinc OS kernel entry point.
//!

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use zcore_drivers::{
    keyboard::{ read_key, scancode_to_ascii },
    vga_buffer::*,
    time::sleep_millis,
    println, print,
};

use zcore_constants::{
    keys::ENTER_SCANCODE,
    colors::{ WHITE, GREEN, LIGHT_BLUE, CYAN },
};

/// Number of rows the prompt takes up (name line, "|" line, input line).
/// Used to check there is room before printing a new prompt.
const PROMPT_ROWS: usize = 3;

/// How many spaces a tab key press inserts.
const TAB_WIDTH: usize = 4;

/// Kernel entry point. The bootloader jumps here after entering 64-bit mode.
///
/// Never returns: after drawing the first prompt it runs the keyboard
/// loop forever.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Wait 800 milliseconds
    sleep_millis(800);

    // Blank the whole screen, removing the bootloader's messages.
    clear_screen(0);

    // The cell where the next character will be written.
    // `println!` and `print!` advance it as they write.
    let mut offset = 0;

    // Boot banner followed by two blank lines.
    println!(offset, "                 Zinc OS | v3.3.0", CYAN);
    println!(offset);
    println!(offset);

    // First prompt: username and directory, a "|" connector line,
    // then the line where typing happens.
    println!(offset, "+- enesx32[/]", LIGHT_BLUE);
    println!(offset, "|", LIGHT_BLUE);
    print!(offset, "+---", LIGHT_BLUE);
    print!(offset, " $ ", GREEN);

    // The first cell of the current input line, right after the "$".
    // Backspace is not allowed to go further back than this, so the
    // prompt itself cannot be erased.
    let mut input_start = offset;

    loop {
        // Check the keyboard. `None` means no key is waiting.
        if let Some(scancode) = read_key() {
            if scancode == ENTER_SCANCODE {
                // Move to the start of the next row.
                offset = (offset / BUFFER_WIDTH + 2) * BUFFER_WIDTH;

                // Make sure the three prompt rows fit, scrolling if not,
                // and shift the cursor up to match.
                let rows = scroll_to_fit(offset, PROMPT_ROWS * BUFFER_WIDTH);
                offset -= rows * BUFFER_WIDTH;

                // Draw a fresh prompt.
                println!(offset, "+- enesx32[/]", LIGHT_BLUE);
                println!(offset, "|", LIGHT_BLUE);
                print!(offset, "+---", LIGHT_BLUE);
                print!(offset, " $ ", GREEN);

                // Typing for this new line starts here.
                input_start = offset;
            } else if let Some(character) = scancode_to_ascii(scancode) {
                // How many cells this key needs. Backspace needs none
                // because it only removes a cell.
                let needed = match character {
                    8 => 0,
                    9 => TAB_WIDTH,
                    _ => 1,
                };

                // Scroll if the key would go past the bottom of the
                // screen, and shift the cursor and the input start up
                // so both still point at the same text.
                let rows = scroll_to_fit(offset, needed);
                offset -= rows * BUFFER_WIDTH;
                input_start = input_start.saturating_sub(rows * BUFFER_WIDTH);

                match character {
                    // Backspace: erase the previous cell, but never
                    // go back past the start of the input.
                    8 => {
                        if offset > input_start {
                            offset -= 1;
                            blank_cell(offset);
                        }
                    }

                    // Tab: insert TAB_WIDTH spaces.
                    9 => {
                        let mut count = 0;
                        while count < TAB_WIDTH {
                            write_cell(offset, b' ', WHITE);
                            offset += 1;
                            count += 1;
                        }
                    }

                    // Any other character: draw it and move forward.
                    _ => {
                        write_cell(offset, character, WHITE);
                        offset += 1;
                    }
                }
            }
        }

        // Hint to the CPU that this is a busy-wait loop.
        core::hint::spin_loop();
    }
}

/// Called if the kernel panics.
///
/// Zinc has nowhere to report panic information yet, so it just halts
/// in a loop.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}