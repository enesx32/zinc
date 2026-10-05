/// Maximum number of directory entries stored in one sector.
pub const MAX_ENTRIES: usize = 8;

/// Size of one directory entry.
pub const ENTRY_SIZE: usize = 64;

/// Maximum length of a directory or file name.
pub const NAME_LENGTH: usize = 48;

/// Directory entry type for a file.
pub const FILE: u8 = 1;

/// Directory entry type for a directory.
pub const DIRECTORY: u8 = 2;

/// A single entry inside a Zinc directory.
#[derive(Clone, Copy)]
pub struct DirectoryEntry {
    /// Whether this entry is being used.
    pub used: u8,

    /// The type of entry.
    pub kind: u8,

    /// The sector containing this entry's data.
    pub sector: u32,

    /// The size of the entry in bytes.
    pub size: u32,

    /// The name of the file or directory.
    pub name: [u8; NAME_LENGTH],

    /// Reserved space for future filesystem features.
    pub reserved: [u8; 4],
}

impl DirectoryEntry {
    /// Creates an empty directory entry.
    pub const fn empty() -> Self {
        Self {
            used: 0,
            kind: 0,
            sector: 0,
            size: 0,
            name: [0; NAME_LENGTH],
            reserved: [0; 4],
        }
    }

    /// Creates a directory entry for a directory.
    pub fn new_directory(name: &str, sector: u32) -> Self {
        let mut entry = Self::empty();

        // Mark the entry as used.
        entry.used = 1;

        // Mark the entry as a directory.
        entry.kind = DIRECTORY;

        // Store the sector containing the directory.
        entry.sector = sector;

        // Copy the directory name.
        let bytes = name.as_bytes();
        let length = if bytes.len() > NAME_LENGTH {
            NAME_LENGTH
        } else {
            bytes.len()
        };

        entry.name[..length].copy_from_slice(&bytes[..length]);

        // Return the new directory entry.
        entry
    }

    /// Checks whether this entry is being used.
    pub fn is_used(&self) -> bool {
        self.used != 0
    }

    /// Checks whether this entry is a directory.
    pub fn is_directory(&self) -> bool {
        self.kind == DIRECTORY
    }

    /// Gets the entry name.
    pub fn name_str(&self) -> Option<&str> {
        // Find the end of the stored name.
        let mut length = 0;

        while length < NAME_LENGTH && self.name[length] != 0 {
            length += 1;
        }

        // Convert the name bytes into a string.
        core::str::from_utf8(&self.name[..length]).ok()
    }

    /// Writes this directory entry into a byte buffer.
    pub fn write_to(&self, buffer: &mut [u8; ENTRY_SIZE]) {
        // Clear the entry buffer.
        buffer.fill(0);

        // Write whether the entry is used.
        buffer[0] = self.used;

        // Write the entry type.
        buffer[1] = self.kind;

        // Write the sector number.
        buffer[2..6].copy_from_slice(&self.sector.to_le_bytes());

        // Write the entry size.
        buffer[6..10].copy_from_slice(&self.size.to_le_bytes());

        // Write the entry name.
        buffer[10..58].copy_from_slice(&self.name);

        // Write the reserved bytes.
        buffer[58..62].copy_from_slice(&self.reserved);
    }

    /// Reads a directory entry from a byte buffer.
    pub fn from_bytes(buffer: &[u8; ENTRY_SIZE]) -> Self {
        // Read the entry name.
        let mut name = [0u8; NAME_LENGTH];
        name.copy_from_slice(&buffer[10..58]);

        // Read the reserved bytes.
        let mut reserved = [0u8; 4];
        reserved.copy_from_slice(&buffer[58..62]);

        // Return the decoded directory entry.
        Self {
            used: buffer[0],
            kind: buffer[1],
            sector: u32::from_le_bytes([
                buffer[2],
                buffer[3],
                buffer[4],
                buffer[5],
            ]),
            size: u32::from_le_bytes([
                buffer[6],
                buffer[7],
                buffer[8],
                buffer[9],
            ]),
            name,
            reserved,
        }
    }
}