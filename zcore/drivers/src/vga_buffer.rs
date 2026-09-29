pub const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

pub const BLACK: u16 = 0x0000;
pub const BLUE: u16 = 0x0100;
pub const GREEN: u16 = 0x0200;
pub const CYAN: u16 = 0x0300;
pub const RED: u16 = 0x0400;
pub const MAGENTA: u16 = 0x0500;
pub const BROWN: u16 = 0x0600;
pub const LIGHT_GRAY: u16 = 0x0700;

pub const DARK_GRAY: u16 = 0x0800;
pub const LIGHT_BLUE: u16 = 0x0900;
pub const LIGHT_GREEN: u16 = 0x0A00;
pub const LIGHT_CYAN: u16 = 0x0B00;
pub const LIGHT_RED: u16 = 0x0C00;
pub const LIGHT_MAGENTA: u16 = 0x0D00;
pub const YELLOW: u16 = 0x0E00;
pub const WHITE: u16 = 0x0F00;

pub const BLANK: u16 = 0x0720;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const MAX_CELLS: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

use zcore_types::string::String;

#[inline(always)]
pub fn scroll() {
    unsafe {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let src = VGA_BUFFER.add(row * BUFFER_WIDTH + col);
                let dst = VGA_BUFFER.add((row - 1) * BUFFER_WIDTH + col);

                let character = core::ptr::read_volatile(src);
                core::ptr::write_volatile(dst, character);
            }
        }

        for col in 0..BUFFER_WIDTH {
            let dst = VGA_BUFFER.add((BUFFER_HEIGHT - 1) * BUFFER_WIDTH + col);
            core::ptr::write_volatile(dst, BLANK);
        }
    }
}

#[inline(always)]
pub fn clear_screen(mut i: usize) {
    while i < MAX_CELLS {
        unsafe {
            core::ptr::write_volatile(VGA_BUFFER.add(i), BLANK);
        }
        i += 1;
    }
}

pub fn write_text<const N: usize>(mut offset: usize, text: &String<N>, color: u16) -> usize {
    unsafe {
        for &byte in text.as_bytes() {
            if offset >= MAX_CELLS {
                scroll();
                offset = (BUFFER_HEIGHT - 1) * BUFFER_WIDTH;
            }

            if byte == b'\n' {
                let row = offset / BUFFER_WIDTH;
                offset = (row + 1) * BUFFER_WIDTH;
            } else {
                let cell: u16 = color | (byte as u16);
                core::ptr::write_volatile(VGA_BUFFER.add(offset), cell);
                offset += 1;
            }
        }
    }

    offset
}