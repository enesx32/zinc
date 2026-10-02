#![no_std]
#![no_main]

use core::panic::PanicInfo;

use zcore_drivers::vga_buffer::*;
use zcore_drivers::println;
use zcore_util::time::sleep_millis;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    sleep_millis(100);

    clear_screen(0);

    let mut offset = 1;

    println!(offset, "          Zinc OS | v3.1.0", CYAN);
    println!(offset);
    println!(offset);

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