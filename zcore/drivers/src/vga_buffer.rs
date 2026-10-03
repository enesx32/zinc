pub const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

pub const WHITE: u16 = 0x0F00;
pub const BLANK: u16 = 0x0720;

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;
pub const MAX_CELLS: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

pub use zcore_types::string::String;

/// Moves every row of the screen up by one and blanks the last row.
///
/// Called automatically by `write_text` when text runs past the bottom
/// of the screen. It is deliberately not `#[inline(always)]`: the macros
/// create a differently sized `String` at almost every call, so inlining
/// would paste a full copy of these loops into each one and bloat the kernel.
pub fn scroll() {
    unsafe {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let src = VGA_BUFFER.add(row * BUFFER_WIDTH + col);
                let dst = VGA_BUFFER.add((row - 1) * BUFFER_WIDTH + col);

                let cell = core::ptr::read_volatile(src);

                core::ptr::write_volatile(dst, cell);
            }
        }

        for col in 0..BUFFER_WIDTH {
            core::ptr::write_volatile(
                VGA_BUFFER.add((BUFFER_HEIGHT - 1) * BUFFER_WIDTH + col),
                BLANK,
            );
        }
    }
}

/// Writes one character with the given color into the VGA cell at `offset`.
///
/// `offset` is a cell index (`row * 80 + column`), not a byte offset.
/// Offsets past the end of the screen are ignored.
pub fn write_cell(offset: usize, character: u8, color: u16) {
    if offset >= MAX_CELLS {
        return;
    }

    unsafe {
        core::ptr::write_volatile(VGA_BUFFER.add(offset), color | character as u16);
    }
}

/// Clears the VGA cell at `offset` back to a blank space.
///
/// Offsets past the end of the screen are ignored.
pub fn blank_cell(offset: usize) {
    if offset >= MAX_CELLS {
        return;
    }

    unsafe {
        core::ptr::write_volatile(VGA_BUFFER.add(offset), BLANK);
    }
}

/// Scrolls the screen up until `needed` cells fit from `offset` onwards.
///
/// Every scroll moves all rows up by one and blanks the last row, so
/// everything on screen, including the cursor position, shifts up by
/// one row (`BUFFER_WIDTH` cells).
///
/// Returns how many rows were scrolled. The caller must subtract
/// `rows * BUFFER_WIDTH` from its own `offset` (and from any other saved
/// positions) so they keep pointing at the same text.
pub fn scroll_to_fit(offset: usize, needed: usize) -> usize {
    let mut rows = 0;
    let mut position = offset;

    while position + needed > MAX_CELLS {
        scroll();
        position -= BUFFER_WIDTH;
        rows += 1;
    }

    rows
}

/// Fills the screen with blank cells, starting at cell `start`.
///
/// `clear_screen(0)` clears the whole screen. Also moves the print cursor
/// to `start`, so the next `print!` / `println!` begins there.
pub fn clear_screen(start: usize) {
    let mut i = start;

    while i < MAX_CELLS {
        unsafe {
            core::ptr::write_volatile(
                VGA_BUFFER.add(i),
                BLANK,
            );
        }

        i += 1;
    }
}

/// draws text starting at cell `offset` and returns the
/// cell where the next character would go.
///
/// - `offset`: cell index, `row * 80 + column`.
/// - `text`: the string to draw. A `'\n'` jumps to the start of the next row.
/// - `color`: one of the color constants, for example `CYAN`.
///
/// If the text reaches the bottom of the screen, the screen scrolls up one
/// row and writing continues on the last row.
///
/// This does not touch the global cursor. Normally you want `print!` or
/// `println!` instead, which track the position for you.
pub fn write_text<const N: usize>(mut offset: usize, text: &String<N>, color: u16,) -> usize {
    for &byte in text.as_bytes() {
        if offset >= MAX_CELLS {
            scroll();

            offset = (BUFFER_HEIGHT - 1) * BUFFER_WIDTH;
        }

        if byte == b'\n' {
            let row = offset / BUFFER_WIDTH;

            offset = (row + 1) * BUFFER_WIDTH;
        } else {
            let cell = color | byte as u16;

            unsafe {
                core::ptr::write_volatile(
                    VGA_BUFFER.add(offset),
                    cell,
                );
            }

            offset += 1;
        }
    }

    offset
}

/// Prints text at the current cursor position without a trailing newline.
///
/// - `print!("text")` prints in white.
/// - `print!("text", RED)` prints in the given color.
///
/// The text must be a string literal or a `const` string, because its
/// length is needed at compile time to size the `String`.
#[macro_export]
macro_rules! print {
    ($offset:ident, $text:expr) => {{
        const LEN: usize = $text.len();

        let string = $crate::vga_buffer::String::<LEN>::from($text);

        $offset = $crate::vga_buffer::write_text($offset, &string, $crate::vga_buffer::WHITE,);
    }};

    ($offset:ident, $text:expr, $color:expr) => {{
        const LEN: usize = $text.len();

        let string = $crate::vga_buffer::String::<LEN>::from($text);

        $offset = $crate::vga_buffer::write_text($offset, &string, $color,);
    }};
}

/// Like `print!`, but moves to the next row afterwards.
///
/// - `println!()` prints an empty line.
/// - `println!("text")` prints white text, then a newline.
/// - `println!("text", RED)` prints colored text, then a newline.
#[macro_export]
macro_rules! println {
    ($offset:ident) => {{
        const LEN: usize = 1;

        let string = $crate::vga_buffer::String::<LEN>::from("\n");

        $offset = $crate::vga_buffer::write_text($offset, &string, $crate::vga_buffer::WHITE,);
    }};

    ($offset:ident, $text:expr) => {{
        const LEN: usize = $text.len();

        let string = $crate::vga_buffer::String::<LEN>::from($text);

        $offset = $crate::vga_buffer::write_text($offset, &string, $crate::vga_buffer::WHITE,);

        const NEWLINE_LEN: usize = 1;

        let newline = $crate::vga_buffer::String::<NEWLINE_LEN>::from("\n");

        $offset = $crate::vga_buffer::write_text($offset, &newline, $crate::vga_buffer::WHITE,);
    }};

    ($offset:ident, $text:expr, $color:expr) => {{
        const LEN: usize = $text.len();

        let string = $crate::vga_buffer::String::<LEN>::from($text);

        $offset = $crate::vga_buffer::write_text($offset, &string, $color,);

        const NEWLINE_LEN: usize = 1;

        let newline = $crate::vga_buffer::String::<NEWLINE_LEN>::from("\n");

        $offset = $crate::vga_buffer::write_text($offset, &newline, $crate::vga_buffer::WHITE,);
    }};
}