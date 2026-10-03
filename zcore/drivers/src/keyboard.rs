use core::arch::asm;

/// Reads a single byte from an x86 I/O port.
///
/// I/O ports are special addresses used by hardware devices to communicate
/// with the CPU.
///
/// `port` tells the CPU which hardware device/port we want to read from.
///
/// The returned `u8` is the single byte of data provided by the hardware.
fn inb(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!("in al, dx",in("dx") port,out("al") value,options(nostack, preserves_flags)); 
    }

    value
}

/// Reads a keyboard scancode from the PS/2 keyboard data port.
///
/// `0x64` is the keyboard controller's status port.
///
/// `0x60` is the keyboard controller's data port.
///
/// Returns `Some(scancode)` when a key event is waiting.
///
/// Returns `None` when there is currently no keyboard data.
pub fn read_key() -> Option<u8> {
    // Read the status byte from the keyboard controller.
    let status = inb(0x64);

    // Bit 0 tells us whether data is waiting in the output buffer.
    //
    // `status & 1` isolates bit 0.
    //
    // If it is zero, there is no keyboard data to read.
    if status & 1 == 0 {
        return None;
    }

    // Data is available, so read the keyboard scancode.
    Some(inb(0x60))
}

/// Converts a keyboard scancode into an ASCII character.
///
/// Keyboard hardware does not directly give us characters such as `a` or `b`.
/// It gives us numeric scancodes instead.
///
/// This function translates the scancode into an ASCII byte.
pub fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    match scancode {
        0x1E => Some(b'a'),
        0x30 => Some(b'b'),
        0x2E => Some(b'c'),
        0x20 => Some(b'd'),
        0x12 => Some(b'e'),
        0x21 => Some(b'f'),
        0x22 => Some(b'g'),
        0x23 => Some(b'h'),
        0x17 => Some(b'i'),
        0x24 => Some(b'j'),
        0x25 => Some(b'k'),
        0x26 => Some(b'l'),
        0x32 => Some(b'm'),
        0x31 => Some(b'n'),
        0x18 => Some(b'o'),
        0x19 => Some(b'p'),
        0x10 => Some(b'q'),
        0x13 => Some(b'r'),
        0x1F => Some(b's'),
        0x14 => Some(b't'),
        0x16 => Some(b'u'),
        0x2F => Some(b'v'),
        0x11 => Some(b'w'),
        0x2D => Some(b'x'),
        0x15 => Some(b'y'),
        0x2C => Some(b'z'),
        0x39 => Some(b' '),
        0x29 => Some(b'`'),
        0x02 => Some(b'1'),
        0x03 => Some(b'2'),
        0x04 => Some(b'3'),
        0x05 => Some(b'4'),
        0x06 => Some(b'5'),
        0x07 => Some(b'6'),
        0x08 => Some(b'7'),
        0x09 => Some(b'8'),
        0x0A => Some(b'9'),
        0x0B => Some(b'0'),
        0x0C => Some(b'-'),
        0x0D => Some(b'='),
        0x1A => Some(b'['),
        0x1B => Some(b']'),
        0x2B => Some(b'\\'),
        0x27 => Some(b';'),
        0x28 => Some(b'\''),
        0x33 => Some(b','),
        0x34 => Some(b'.'),
        0x35 => Some(b'/'),
        0x0E => Some(b'\x08'), // Backspace
        0x0F => Some(b'\t'),   // Tab
        
        

        // Other scancodes are ignored for now.
        _ => None,
    }
}