#![no_std]

use zcore_drivers::disk::{read_sector, write_sector};

pub mod directory;

use directory::{DirectoryEntry, DIRECTORY, ENTRY_SIZE, MAX_ENTRIES};

/// The sector containing Zinc's filesystem superblock.
pub const SUPERBLOCK_SECTOR: u32 = 1;

/// The sector containing Zinc's root directory.
pub const ROOT_DIRECTORY_SECTOR: u32 = 2;

/// The size of one disk sector.
pub const SECTOR_SIZE: usize = 512;

/// Zinc filesystem magic bytes.
pub const MAGIC: [u8; 4] = *b"ZINC";

/// Current Zinc filesystem version.
pub const VERSION: u16 = 1;

/// Zinc filesystem superblock.
#[derive(Clone, Copy)]
pub struct Superblock {
    /// Identifies this as a Zinc filesystem.
    pub magic: [u8; 4],

    /// Filesystem format version.
    pub version: u16,

    /// Size of one disk sector.
    pub sector_size: u16,

    /// Total number of sectors on the disk.
    pub total_sectors: u32,

    /// Sector containing the root directory.
    pub root_directory_sector: u32,
}

impl Superblock {
    /// Creates a new Zinc filesystem superblock.
    pub const fn new(total_sectors: u32) -> Self {
        Self {
            magic: MAGIC,
            version: VERSION,
            sector_size: SECTOR_SIZE as u16,
            total_sectors,
            root_directory_sector: ROOT_DIRECTORY_SECTOR,
        }
    }

    /// Writes the superblock into a 512-byte buffer.
    pub fn write_to(&self, buffer: &mut [u8; SECTOR_SIZE]) {
        // Clear the entire sector first.
        buffer.fill(0);

        // Write the filesystem magic.
        buffer[0..4].copy_from_slice(&self.magic);

        // Write the filesystem version.
        buffer[4..6].copy_from_slice(&self.version.to_le_bytes());

        // Write the sector size.
        buffer[6..8].copy_from_slice(&self.sector_size.to_le_bytes());

        // Write the total number of sectors.
        buffer[8..12].copy_from_slice(&self.total_sectors.to_le_bytes());

        // Write the root directory sector.
        buffer[12..16].copy_from_slice(&self.root_directory_sector.to_le_bytes());
    }

    /// Reads a superblock from a 512-byte buffer.
    pub fn from_bytes(buffer: &[u8; SECTOR_SIZE]) -> Option<Self> {
        // Check the filesystem magic.
        if buffer[0..4] != MAGIC {
            return None;
        }

        // Read the filesystem version.
        let version = u16::from_le_bytes([buffer[4], buffer[5]]);

        // Reject unsupported filesystem versions.
        if version != VERSION {
            return None;
        }

        // Read the sector size.
        let sector_size = u16::from_le_bytes([buffer[6], buffer[7]]);

        // Make sure the sector size is the one Zinc expects.
        if sector_size != SECTOR_SIZE as u16 {
            return None;
        }

        // Read the total number of sectors.
        let total_sectors = u32::from_le_bytes([
            buffer[8],
            buffer[9],
            buffer[10],
            buffer[11],
        ]);

        // Read the root directory sector.
        let root_directory_sector = u32::from_le_bytes([
            buffer[12],
            buffer[13],
            buffer[14],
            buffer[15],
        ]);

        // Return the decoded superblock.
        Some(Self {
            magic: MAGIC,
            version,
            sector_size,
            total_sectors,
            root_directory_sector,
        })
    }
}

/// Writes a directory to one sector.
fn write_directory(sector: u32, entries: &[DirectoryEntry; MAX_ENTRIES]) -> bool {
    // Create a buffer for the directory sector.
    let mut buffer = [0u8; SECTOR_SIZE];

    // Create a buffer for one directory entry.
    let mut entry_buffer = [0u8; ENTRY_SIZE];

    // Write every entry into the sector.
    for i in 0..MAX_ENTRIES {
        // Convert the entry into bytes.
        entries[i].write_to(&mut entry_buffer);

        // Find where the entry belongs in the sector.
        let start = i * ENTRY_SIZE;
        let end = start + ENTRY_SIZE;

        // Copy the entry into the sector.
        buffer[start..end].copy_from_slice(&entry_buffer);
    }

    // Write the directory to disk.
    unsafe {
        write_sector(sector, &buffer)
    }
}

/// Reads a directory from one sector.
pub fn read_directory(sector: u32) -> Option<[DirectoryEntry; MAX_ENTRIES]> {
    // Create a buffer for the directory sector.
    let mut buffer = [0u8; SECTOR_SIZE];

    // Read the directory sector.
    let success = unsafe {
        read_sector(sector, &mut buffer)
    };

    // Stop if the sector could not be read.
    if !success {
        return None;
    }

    // Create an empty entry array.
    let mut entries = [DirectoryEntry::empty(); MAX_ENTRIES];

    // Create a buffer for one directory entry.
    let mut entry_buffer = [0u8; ENTRY_SIZE];

    // Read every entry from the sector.
    for i in 0..MAX_ENTRIES {
        // Find where the entry is stored.
        let start = i * ENTRY_SIZE;
        let end = start + ENTRY_SIZE;

        // Copy the entry into its own buffer.
        entry_buffer.copy_from_slice(&buffer[start..end]);

        // Decode the entry.
        entries[i] = DirectoryEntry::from_bytes(&entry_buffer);
    }

    // Return the directory entries.
    Some(entries)
}

/// Finds a directory inside another directory.
pub fn find_directory(parent_sector: u32, name: &str) -> Option<u32> {
    // Read the parent directory.
    let entries = read_directory(parent_sector)?;

    // Search every entry.
    for entry in entries.iter() {
        // Ignore unused entries.
        if !entry.is_used() {
            continue;
        }

        // Ignore files.
        if entry.kind != DIRECTORY {
            continue;
        }

        // Get the entry name.
        let entry_name = entry.name_str()?;

        // Check whether the names match.
        if entry_name == name {
            return Some(entry.sector);
        }
    }

    // The directory could not be found.
    None
}

/// Gets the parent directory of a directory.
pub fn parent_directory(sector: u32) -> Option<u32> {
    // Read the current directory.
    let entries = read_directory(sector)?;

    // Search for the ".." entry.
    for entry in entries.iter() {
        // Ignore unused entries.
        if !entry.is_used() {
            continue;
        }

        // Get the entry name.
        let name = entry.name_str()?;

        // Check for the parent entry.
        if name == ".." {
            return Some(entry.sector);
        }
    }

    // Fall back to the root directory.
    Some(ROOT_DIRECTORY_SECTOR)
}

/// Changes the current directory.
pub fn change_directory(current_sector: u32, target: &str) -> Option<u32> {
    // Go directly to the root directory.
    if target == "/" {
        return Some(ROOT_DIRECTORY_SECTOR);
    }

    // Stay in the current directory.
    if target == "." {
        return Some(current_sector);
    }

    // Move to the parent directory.
    if target == ".." {
        return parent_directory(current_sector);
    }

    // Remove a starting slash from an absolute path.
    let name = target.strip_prefix('/').unwrap_or(target);

    // Find the requested directory.
    find_directory(
        if target.starts_with('/') {
            ROOT_DIRECTORY_SECTOR
        } else {
            current_sector
        },
        name,
    )
}

/// Formats the disk with a Zinc filesystem.
///
/// `total_sectors` is the total number of sectors available on the disk.
///
/// Returns `true` when the filesystem was written successfully.
pub fn format(total_sectors: u32) -> bool {
    // Create the Zinc filesystem superblock.
    let superblock = Superblock::new(total_sectors);

    // Create a buffer for the superblock sector.
    let mut buffer = [0u8; SECTOR_SIZE];

    // Convert the superblock into bytes.
    superblock.write_to(&mut buffer);

    // Write the superblock to sector 1.
    let superblock_written = unsafe {
        write_sector(SUPERBLOCK_SECTOR, &buffer)
    };

    // Stop if writing the superblock failed.
    if !superblock_written {
        return false;
    }

    // Create the root directory entries.
    let mut root_entries = [DirectoryEntry::empty(); MAX_ENTRIES];

    // Add the current directory entry.
    root_entries[0] = DirectoryEntry::new_directory(".", ROOT_DIRECTORY_SECTOR);

    // Add the parent directory entry.
    root_entries[1] = DirectoryEntry::new_directory("..", ROOT_DIRECTORY_SECTOR);

    // Add the /bin directory.
    root_entries[2] = DirectoryEntry::new_directory("bin", 3);

    // Add the /etc directory.
    root_entries[3] = DirectoryEntry::new_directory("etc", 4);

    // Add the /home directory.
    root_entries[4] = DirectoryEntry::new_directory("home", 5);

    // Add the /lib directory.
    root_entries[5] = DirectoryEntry::new_directory("lib", 6);

    // Add the /tmp directory.
    root_entries[6] = DirectoryEntry::new_directory("tmp", 7);

    // Write the root directory to sector 2.
    if !write_directory(ROOT_DIRECTORY_SECTOR, &root_entries) {
        return false;
    }

    // Create the child directory sectors.
    let directories = [
        (3, "bin"),
        (4, "etc"),
        (5, "home"),
        (6, "lib"),
        (7, "tmp"),
    ];

    // Create every child directory.
    for (sector, _name) in directories {
        // Create an empty directory.
        let mut entries = [DirectoryEntry::empty(); MAX_ENTRIES];

        // Add the current directory entry.
        entries[0] = DirectoryEntry::new_directory(".", sector);

        // Add the parent directory entry.
        entries[1] = DirectoryEntry::new_directory("..", ROOT_DIRECTORY_SECTOR);

        // Write the directory to disk.
        if !write_directory(sector, &entries) {
            return false;
        }
    }

    // The filesystem was created successfully.
    true
}

/// Reads the Zinc filesystem superblock from the disk.
pub fn read_superblock() -> Option<Superblock> {
    // Create a buffer for the superblock sector.
    let mut buffer = [0u8; SECTOR_SIZE];

    // Read sector 1.
    let success = unsafe {
        read_sector(SUPERBLOCK_SECTOR, &mut buffer)
    };

    // Stop if the sector could not be read.
    if !success {
        return None;
    }

    // Decode the superblock.
    Superblock::from_bytes(&buffer)
}