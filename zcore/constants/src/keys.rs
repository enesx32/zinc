/// Keyboard scancode for the Enter key.
///
/// Enter is handled by scancode instead of ASCII because
/// `scancode_to_ascii` does not map it.
pub const ENTER_SCANCODE: u8 = 0x1C;