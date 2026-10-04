extern crate alloc;
use alloc::vec::Vec;

/// A heap-allocated UTF-8 string.
///
/// `String` stores its contents in Zinc's global heap allocator.
///
/// # Example
///
/// ```text
/// let my_string = String::from("Hello World");
/// ```
pub struct String {
    data: Vec<u8>,
}

/// `BasicString` is kept as an alias for compatibility.
///
/// It uses the same heap-allocated implementation as `String`.
pub type BasicString = String;

/// Converts a string slice into a heap-allocated `String`.
///
/// # Example
///
/// ```text
/// let my_string = String::from("Hello World");
/// ```
impl From<&str> for String {
    fn from(string: &str) -> Self {
        Self {
            data: Vec::from(string.as_bytes()),
        }
    }
}

/// Methods for working with a heap-allocated `String`.
impl String {
    /// Creates an empty `String`.
    ///
    /// # Example
    ///
    /// ```text
    /// let my_string = String::new();
    /// ```
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Creates an empty `String` with space reserved for `capacity` bytes.
    ///
    /// # Example
    ///
    /// ```text
    /// let my_string = String::with_capacity(64);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of bytes currently stored in the string.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of bytes the string can store before
    /// another allocation is required.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns `true` if the string contains no characters.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the contents of the string as a byte slice.
    ///
    /// The bytes contain the UTF-8 representation of the string.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Returns the contents of the string as a string slice.
    ///
    /// # Safety
    ///
    /// `String` only stores valid UTF-8 through its safe methods,
    /// so the stored bytes are expected to always contain valid UTF-8.
    pub fn to_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.data)
        }
    }

    /// Adds a character to the end of the string.
    ///
    /// # Example
    ///
    /// ```text
    /// let mut my_string = String::from("Hello");
    /// my_string.push('!');
    /// ```
    pub fn push(&mut self, character: char) {
        let mut buffer = [0u8; 4];
        let encoded = character.encode_utf8(&mut buffer);

        self.data.extend_from_slice(encoded.as_bytes());
    }

    /// Adds a string slice to the end of the string.
    ///
    /// # Example
    ///
    /// ```text
    /// let mut my_string = String::from("Hello");
    /// my_string.push_str(" World");
    /// ```
    pub fn push_str(&mut self, string: &str) {
        self.data.extend_from_slice(string.as_bytes());
    }

    /// Removes all characters from the string.
    ///
    /// The allocated memory is kept for reuse.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Removes the last character from the string.
    ///
    /// Returns the removed character, or `None` if the string is empty.
    pub fn pop(&mut self) -> Option<char> {
        if self.data.is_empty() {
            return None;
        }

        let mut index = self.data.len() - 1;

        while index > 0 && (self.data[index] & 0b1100_0000) == 0b1000_0000 {
            index -= 1;
        }

        let character = unsafe {
            core::str::from_utf8_unchecked(&self.data[index..])
        }
        .chars()
        .next();

        self.data.truncate(index);

        character
    }

    /// Splits the string using a byte as the separator.
    ///
    /// The results are stored in a fixed-size array of `String`s.
    ///
    /// `N` specifies the maximum number of parts that can be returned.
    ///
    /// # Example
    ///
    /// ```text
    /// let command = String::from("echo hello world");
    /// let parts = command.split::<6>(b' ');
    ///
    /// parts[0].to_str(); // "echo"
    /// parts[1].to_str(); // "hello"
    /// parts[2].to_str(); // "world"
    /// ```
    pub fn split<const N: usize>(&self, separator: u8) -> [String; N] {
        let mut parts = core::array::from_fn(|_| String::new());

        let mut part = 0;
        let mut start = 0;

        for i in 0..self.data.len() {
            if self.data[i] == separator {
                if part < N {
                    parts[part] = String::from(
                        unsafe {
                            core::str::from_utf8_unchecked(&self.data[start..i])
                        }
                    );

                    part += 1;
                }

                start = i + 1;
            }
        }

        if part < N && start <= self.data.len() {
            parts[part] = String::from(
                unsafe {
                    core::str::from_utf8_unchecked(&self.data[start..])
                }
            );
        }

        parts
    }

    /// Converts the string into a `u8`.
    ///
    /// Returns `None` if the string does not contain a valid `u8`.
    pub fn to_u8(&self) -> Option<u8> {
        self.to_str().parse().ok()
    }

    /// Converts the string into a `u16`.
    ///
    /// Returns `None` if the string does not contain a valid `u16`.
    pub fn to_u16(&self) -> Option<u16> {
        self.to_str().parse().ok()
    }

    /// Converts the string into a `u32`.
    ///
    /// Returns `None` if the string does not contain a valid `u32`.
    pub fn to_u32(&self) -> Option<u32> {
        self.to_str().parse().ok()
    }

    /// Converts the string into a `u64`.
    ///
    /// Returns `None` if the string does not contain a valid `u64`.
    pub fn to_u64(&self) -> Option<u64> {
        self.to_str().parse().ok()
    }

    /// Converts the string into a `u128`.
    ///
    /// Returns `None` if the string does not contain a valid `u128`.
    pub fn to_u128(&self) -> Option<u128> {
        self.to_str().parse().ok()
    }

    /// Converts the string into a `usize`.
    ///
    /// Returns `None` if the string does not contain a valid `usize`.
    pub fn to_usize(&self) -> Option<usize> {
        self.to_str().parse().ok()
    }

    /// Converts the string into an `i8`.
    ///
    /// Returns `None` if the string does not contain a valid `i8`.
    pub fn to_i8(&self) -> Option<i8> {
        self.to_str().parse().ok()
    }

    /// Converts the string into an `i16`.
    ///
    /// Returns `None` if the string does not contain a valid `i16`.
    pub fn to_i16(&self) -> Option<i16> {
        self.to_str().parse().ok()
    }

    /// Converts the string into an `i32`.
    ///
    /// Returns `None` if the string does not contain a valid `i32`.
    pub fn to_i32(&self) -> Option<i32> {
        self.to_str().parse().ok()
    }

    /// Converts the string into an `i64`.
    ///
    /// Returns `None` if the string does not contain a valid `i64`.
    pub fn to_i64(&self) -> Option<i64> {
        self.to_str().parse().ok()
    }

    /// Converts the string into an `i128`.
    ///
    /// Returns `None` if the string does not contain a valid `i128`.
    pub fn to_i128(&self) -> Option<i128> {
        self.to_str().parse().ok()
    }

    /// Converts the string into an `isize`.
    ///
    /// Returns `None` if the string does not contain a valid `isize`.
    pub fn to_isize(&self) -> Option<isize> {
        self.to_str().parse().ok()
    }
}