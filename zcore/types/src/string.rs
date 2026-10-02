/// Fixed size string with a custom length that must be specified at compile time.<br/>
/// The length is stored in a const generic parameter<br/>
/// 
/// Uses could be if you need to store small strings with a known maximum length<br/>
///
pub struct String<const N: usize> {
    pub data: [u8; N],
    pub len: usize,
}

/// Fixed size string with a 256 Byte length.<br/>
/// 
/// Uses could be if you need near to 200 byte long strings<br/>
/// Or if you dont want to specify a length your self<br/>
///
pub struct BasicString {
    pub data: [u8; 256],
    pub len: usize,
}

/// Implementations for converting from &str to String<N>
/// 
/// `String::<11>::from("Hello World")`
/// will create a String<11> with the content "Hello World"
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

/// Implementations for converting from &str to String<N>
/// 
/// `BasicString::from("Hello World")`
/// will create a String<11> with the content "Hello World"
impl From<&str> for BasicString{
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

/// Implementations for String<N>
/// 
/// e.g
/// 
/// `pub fn len(&self) -> usize` will return the length of the string<br/>
/// `pub fn as_bytes(&self) -> &[u8]` will return the string as a byte slice<br/>
/// `pub fn to_str(&self) -> &str` will return the string as a str slice<br/>
impl<const N: usize> String<N> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }

    pub fn to_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.data[..self.len])
        }
    }
}

/// Implementations for BasicString
/// 
/// e.g
/// 
/// `pub fn len(&self) -> usize` will return the length of the string<br/>
/// `pub fn as_bytes(&self) -> &[u8]` will return the string as a byte slice<br/>
/// `pub fn to_str(&self) -> &str` will return the string as a str slice<br/>
impl BasicString {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }

    pub fn to_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.data[..self.len])
        }
    }
}