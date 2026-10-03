/// Fixed-size string with a custom length specified at compile time.
///
/// The length is stored in the const generic parameter `N`.
///
/// This is useful for storing small strings where the maximum size
/// is known at compile time.
///
/// # Example
///
/// ```text
/// let my_string = String::<11>::from("Hello World");
/// ```
pub struct String<const N: usize> {
    pub data: [u8; N],
    pub len: usize,
}

/// Fixed-size string with a maximum length of 256 bytes.
///
/// `BasicString` is useful when you want a string with a reasonably
/// large fixed capacity without having to specify the size yourself.
///
/// # Example
///
/// ```text
/// let my_string = BasicString::from("Hello World");
/// ```
pub struct BasicString {
    pub data: [u8; 256],
    pub len: usize,
}

/// Converts a string slice into a fixed-size `String<N>`.
///
/// If the input string is longer than `N` bytes, it is truncated.
///
/// # Example
///
/// ```text
/// let my_string = String::<11>::from("Hello World");
/// ```
impl<const N: usize> From<&str> for String<N> {
    fn from(string: &str) -> Self {
        let bytes = string.as_bytes();
        let mut data = [0u8; N];

        let len = if bytes.len() < N {
            bytes.len()
        } else {
            N
        };

        data[..len].copy_from_slice(&bytes[..len]);

        Self {
            data,
            len,
        }
    }
}

/// Converts a string slice into a `BasicString`.
///
/// If the input string is longer than 256 bytes, it is truncated.
///
/// # Example
///
/// ```text
/// let my_string = BasicString::from("Hello World");
/// ```
impl From<&str> for BasicString {
    fn from(string: &str) -> Self {
        let bytes = string.as_bytes();
        let mut data = [0u8; 256];

        let len = if bytes.len() < 256 {
            bytes.len()
        } else {
            256
        };

        data[..len].copy_from_slice(&bytes[..len]);

        Self {
            data,
            len,
        }
    }
}

/// Methods for working with a fixed-size `String<N>`.
impl<const N: usize> String<N> {
    /// Returns the number of bytes currently stored in the string.
    ///
    /// # Example
    ///
    /// ```text
    /// let my_string = String::<13>::from("Hello, World!");
    /// my_string.len();
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the contents of the string as a byte slice.
    ///
    /// The bytes contain the UTF-8 representation of the string.
    ///
    /// # Example
    ///
    /// ```text
    /// let my_string = String::<11>::from("Hello World");
    /// my_string.as_bytes();
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }

    /// Returns the contents of the string as a string slice.
    ///
    /// # Safety
    ///
    /// This function assumes that the stored bytes are valid UTF-8.
    /// Strings created through `From<&str>` satisfy this requirement.
    ///
    /// # Example
    ///
    /// ```text
    /// let my_string = String::<11>::from("Hello World");
    /// my_string.to_str();
    /// ```
    pub fn to_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.data[..self.len])
        }
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

/// Methods for working with a `BasicString`.
impl BasicString {
    /// Returns the number of bytes currently stored in the string.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the contents of the string as a byte slice.
    ///
    /// The bytes contain the UTF-8 representation of the string.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }

    /// Splits the string using a byte as the separator.
    ///
    /// The results are stored in a fixed-size array of `BasicString`s.
    ///
    /// `N` specifies the maximum number of parts that can be returned.
    /// Any additional parts beyond `N` are ignored.
    ///
    /// # Example
    ///
    /// ```text
    /// let command = BasicString::from("echo hello world");
    /// let parts = command.split::<6>(b' ');
    ///
    /// parts[0].to_str(); // "echo"
    /// parts[1].to_str(); // "hello"
    /// parts[2].to_str(); // "world"
    /// ```
    pub fn split<const N: usize>(&self, separator: u8) -> [BasicString; N] {
        let mut parts = core::array::from_fn(|_| BasicString {
            data: [0u8; 256],
            len: 0,
        });

        let mut part = 0;
        let mut start = 0;

        for i in 0..self.len {
            if self.data[i] == separator {
                if part < N {
                    parts[part] = BasicString::from(
                        unsafe {
                            core::str::from_utf8_unchecked(&self.data[start..i])
                        }
                    );

                    part += 1;
                }

                start = i + 1;
            }
        }

        if part < N && start <= self.len {
            parts[part] = BasicString::from(
                unsafe {
                    core::str::from_utf8_unchecked(&self.data[start..self.len])
                }
            );
        }

        parts
    }

    /// Returns the contents of the string as a string slice.
    ///
    /// # Safety
    ///
    /// This function assumes that the stored bytes are valid UTF-8.
    /// Strings created through `From<&str>` satisfy this requirement.
    pub fn to_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.data[..self.len])
        }
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