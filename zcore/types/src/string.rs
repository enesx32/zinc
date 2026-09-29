pub struct String<const N: usize> {
    pub data: [u8; N],
    pub len: usize,
}

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

impl<const N: usize> String<N> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }
}