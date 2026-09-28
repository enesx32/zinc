#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;
const VGA_CELLS: usize = 80 * 25;

const BLANK: u16 = 0x0720;
const CYAN: u16 = 0x0b00;

fn clear_screen(mut i:usize) {
    while i < VGA_CELLS {
        unsafe {
            core::ptr::write_volatile(VGA_BUFFER.add(i), BLANK);
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let message: &[u8] = b"                                zinc_os | 1.0";
    let mut i: usize = 0;

    clear_screen(i);
    
    i = 0;
    while i < message.len() {
        let cell: u16 = CYAN | (message[i] as u16);
        unsafe {
            core::ptr::write_volatile(VGA_BUFFER.add(i), cell);
        }
        i += 1;
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}