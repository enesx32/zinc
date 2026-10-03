//! PIT drivers.
//!
//! There are no interrupts or timer handlers in Zinc yet, so delays
//! are done by polling the PIT (Programmable Interval Timer), a chip
//! that counts down at about 1.193 MHz.

use core::arch::asm;

/// Writes one byte to an x86 I/O port.
///
/// I/O ports are a separate address space the CPU uses to talk to
/// hardware chips. Rust has no way to use the `out` instruction
/// directly, so it is done with inline assembly.
fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nostack, preserves_flags));
    }
}

/// Reads one byte from an x86 I/O port.
fn inb(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!("in al, dx", in("dx") port, out("al") value, options(nostack, preserves_flags));
    }

    value
}

/// Reads the PIT's current countdown value.
///
/// Port `0x43` is the PIT command port and `0x40` is the data port of
/// its channel 0. Writing `0x00` to the command port freezes the
/// current count so it can be read in two steps: low byte first, then
/// high byte.
fn read_pit() -> u16 {
    outb(0x43, 0x00);
    let low = inb(0x40) as u16;
    let high = inb(0x40) as u16;
    (high << 8) | low
}

/// Waits for roughly `milliseconds` milliseconds by busy-waiting.
///
/// First the PIT is put into a mode where it counts down by one per
/// tick and wraps around after 65,536 ticks (command `0x34`, reload
/// value 0). The BIOS default counts in steps of two, which would
/// make every delay half as long.
///
/// Then the loop measures how many ticks have passed since the last
/// read, using `wrapping_sub` so a wrap-around of the counter is
/// handled correctly. About 1193 ticks make one millisecond.
///
/// The CPU spins for the whole delay. That is fine for now; once
/// interrupts exist, a sleep can halt the CPU instead.
pub fn sleep_millis(milliseconds: usize) {
    // Set channel 0 to count down by one per tick, reload value 0
    // (which means 65,536).
    outb(0x43, 0x34);
    outb(0x40, 0x00);
    outb(0x40, 0x00);

    // Ticks still to wait.
    let mut remaining: usize = milliseconds * 1193;

    // Counter value at the previous check.
    let mut last = read_pit();

    while remaining > 0 {
        let now = read_pit();

        // The counter counts down, so elapsed ticks = last - now.
        let elapsed = last.wrapping_sub(now) as usize;
        last = now;

        if elapsed >= remaining {
            remaining = 0;
        } else {
            remaining -= elapsed;
        }
    }
}