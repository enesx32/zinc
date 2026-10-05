use core::arch::asm;

/// Primary ATA data port.
const DATA_PORT: u16 = 0x1F0;

/// Primary ATA sector count port.
const SECTOR_COUNT_PORT: u16 = 0x1F2;

/// Primary ATA LBA low port.
const LBA_LOW_PORT: u16 = 0x1F3;

/// Primary ATA LBA middle port.
const LBA_MID_PORT: u16 = 0x1F4;

/// Primary ATA LBA high port.
const LBA_HIGH_PORT: u16 = 0x1F5;

/// Primary ATA drive and LBA port.
const DRIVE_PORT: u16 = 0x1F6;

/// Primary ATA status and command port.
const STATUS_PORT: u16 = 0x1F7;

/// Read sector command.
const READ_SECTOR: u8 = 0x20;

/// Write sector command.
const WRITE_SECTOR: u8 = 0x30;

/// Flush cache command.
const FLUSH_CACHE: u8 = 0xE7;

/// ATA status bit for the drive being busy.
const STATUS_BSY: u8 = 0x80;

/// ATA status bit for an error.
const STATUS_ERR: u8 = 0x01;

/// ATA status bit for data being ready.
const STATUS_DRQ: u8 = 0x08;

/// Reads one 512-byte sector from the primary ATA drive.
///
/// `lba` is the sector number to read.
///
/// `buffer` must be exactly 512 bytes long.
///
/// Returns `true` when the sector was read successfully.
pub unsafe fn read_sector(lba: u32, buffer: &mut [u8; 512]) -> bool {
    // Select the primary master drive.
    unsafe {
        outb(DRIVE_PORT, 0xE0 | ((lba >> 24) as u8 & 0x0F));
    }

    // Tell the drive that one sector should be read.
    unsafe {
        outb(SECTOR_COUNT_PORT, 1);
    }

    // Send the lower eight bits of the LBA.
    unsafe {
        outb(LBA_LOW_PORT, lba as u8);
    }

    // Send the middle eight bits of the LBA.
    unsafe {
        outb(LBA_MID_PORT, (lba >> 8) as u8);
    }

    // Send the upper eight bits of the LBA.
    unsafe {
        outb(LBA_HIGH_PORT, (lba >> 16) as u8);
    }

    // Tell the drive to read the requested sector.
    unsafe {
        outb(STATUS_PORT, READ_SECTOR);
    }

    // Wait until the drive has finished processing the command.
    if !wait_for_data() {
        return false;
    }

    // Read all 512 bytes as 256 words.
    for i in 0..256 {
        // Read one 16-bit word from the drive.
        let value = unsafe {
            inw(DATA_PORT)
        };

        // Store the lower byte in the buffer.
        buffer[i * 2] = value as u8;

        // Store the upper byte in the buffer.
        buffer[i * 2 + 1] = (value >> 8) as u8;
    }

    // The sector was read successfully.
    true
}

/// Writes one 512-byte sector to the primary ATA drive.
///
/// `lba` is the sector number to write.
///
/// `buffer` must be exactly 512 bytes long.
///
/// Returns `true` when the sector was written successfully.
pub unsafe fn write_sector(lba: u32, buffer: &[u8; 512]) -> bool {
    // Select the primary master drive.
    unsafe {
        outb(DRIVE_PORT, 0xE0 | ((lba >> 24) as u8 & 0x0F));
    }

    // Tell the drive that one sector should be written.
    unsafe {
        outb(SECTOR_COUNT_PORT, 1);
    }

    // Send the lower eight bits of the LBA.
    unsafe {
        outb(LBA_LOW_PORT, lba as u8);
    }

    // Send the middle eight bits of the LBA.
    unsafe {
        outb(LBA_MID_PORT, (lba >> 8) as u8);
    }

    // Send the upper eight bits of the LBA.
    unsafe {
        outb(LBA_HIGH_PORT, (lba >> 16) as u8);
    }

    // Tell the drive to write the requested sector.
    unsafe {
        outb(STATUS_PORT, WRITE_SECTOR);
    }

    // Wait until the drive is ready to receive data.
    if !wait_for_data() {
        return false;
    }

    // Write all 512 bytes as 256 words.
    for i in 0..256 {
        // Combine two bytes into one 16-bit word.
        let value = buffer[i * 2] as u16 | ((buffer[i * 2 + 1] as u16) << 8);

        // Write the word to the drive.
        unsafe {
            outw(DATA_PORT, value);
        }
    }

    // Tell the drive to flush its cache.
    unsafe {
        outb(STATUS_PORT, FLUSH_CACHE);
    }

    // Wait until the write has finished.
    wait_for_ready()
}

/// Waits for the ATA drive to become ready for data.
fn wait_for_data() -> bool {
    // Keep checking the drive status.
    loop {
        // Read the current ATA status.
        let status = unsafe {
            inb(STATUS_PORT)
        };

        // Stop if the drive reports an error.
        if status & STATUS_ERR != 0 {
            return false;
        }

        // Stop waiting once the drive is no longer busy and data is ready.
        if status & STATUS_BSY == 0 && status & STATUS_DRQ != 0 {
            return true;
        }
    }
}

/// Waits for the ATA drive to finish its current operation.
fn wait_for_ready() -> bool {
    // Keep checking the drive status.
    loop {
        // Read the current ATA status.
        let status = unsafe {
            inb(STATUS_PORT)
        };

        // Stop if the drive reports an error.
        if status & STATUS_ERR != 0 {
            return false;
        }

        // Stop once the drive is no longer busy.
        if status & STATUS_BSY == 0 {
            return true;
        }
    }
}

/// Writes one byte to an I/O port.
unsafe fn outb(port: u16, value: u8) {
    // Send the byte to the requested hardware port.
    asm!("out dx, al", in("dx") port, in("al") value, options(nostack, preserves_flags));
}

/// Reads one byte from an I/O port.
unsafe fn inb(port: u16) -> u8 {
    // Stores the byte read from the port.
    let value: u8;

    // Read one byte from the requested hardware port.
    asm!("in al, dx", in("dx") port, out("al") value, options(nostack, preserves_flags));

    // Return the byte that was read.
    value
}

/// Reads one 16-bit word from an I/O port.
unsafe fn inw(port: u16) -> u16 {
    // Stores the word read from the port.
    let value: u16;

    // Read one 16-bit word from the requested hardware port.
    asm!("in ax, dx", in("dx") port, out("ax") value, options(nostack, preserves_flags));

    // Return the word that was read.
    value
}

/// Writes one 16-bit word to an I/O port.
unsafe fn outw(port: u16, value: u16) {
    // Send the word to the requested hardware port.
    asm!("out dx, ax", in("dx") port, in("ax") value, options(nostack, preserves_flags));
}