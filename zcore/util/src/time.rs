use core::arch::asm;

/// Writes a byte to the specified port
/// 
fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nostack, preserves_flags));
    }
}

/// Reads a byte from the specified port
/// 
fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") value, options(nostack, preserves_flags));
    }
    value
}

/// Reads the current value of the PIT counter
/// 
fn read_pit() -> u16 {
    outb(0x43, 0x00);
    let low = inb(0x40) as u16;
    let high = inb(0x40) as u16;
    (high << 8) | low
}

/// Sleeps for the specified number of milliseconds
/// 
/// By reading the PIT counter and calculating the elapsed time, 
/// this function provides a simple way to pause execution for a given duration. 
/// The PIT is configured to run at a frequency of 1193 Hz, so the number of 
/// milliseconds is converted to PIT ticks accordingly.
pub fn sleep_millis(milliseconds: usize) {
    outb(0x43, 0x34);
    outb(0x40, 0x00);
    outb(0x40, 0x00);

    let mut remaining: usize = milliseconds * 1193;
    let mut last = read_pit();

    while remaining > 0 {
        let now = read_pit();
        let elapsed = last.wrapping_sub(now) as usize;
        last = now;

        if elapsed >= remaining {
            remaining = 0;
        } else {
            remaining -= elapsed;
        }
    }
}