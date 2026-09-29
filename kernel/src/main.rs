#![no_std]
#![no_main]

use core::panic::PanicInfo;

use zcore_drivers::vga_buffer::{
    clear_screen,
    write_text,
    BLACK,
    BLUE,
    GREEN,
    CYAN,
    RED,
    MAGENTA,
    BROWN,
    LIGHT_GRAY,
    DARK_GRAY,
    LIGHT_BLUE,
    LIGHT_GREEN,
    LIGHT_CYAN,
    LIGHT_RED,
    LIGHT_MAGENTA,
    YELLOW,
    WHITE,
};

use zcore_types::string::String;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    clear_screen(0);

    let zinc_version_display: String<64> =
        String::<64>::from("                                Zinc OS | v2.1.0\n\n\n");

    let black_display: String<64> = String::<64>::from("BLACK  ");
    let blue_display: String<64> = String::<64>::from("BLUE  ");
    let green_display: String<64> = String::<64>::from("GREEN  ");
    let cyan_display: String<64> = String::<64>::from("CYAN  ");
    let red_display: String<64> = String::<64>::from("RED  ");
    let magenta_display: String<64> = String::<64>::from("MAGENTA  ");
    let brown_display: String<64> = String::<64>::from("BROWN  ");
    let light_gray_display: String<64> = String::<64>::from("LIGHT GRAY\n");

    let dark_gray_display: String<64> = String::<64>::from("DARK GRAY  ");
    let light_blue_display: String<64> = String::<64>::from("LIGHT BLUE  ");
    let light_green_display: String<64> = String::<64>::from("LIGHT GREEN  ");
    let light_cyan_display: String<64> = String::<64>::from("LIGHT CYAN  ");
    let light_red_display: String<64> = String::<64>::from("LIGHT RED  ");
    let light_magenta_display: String<64> = String::<64>::from("LIGHT MAGENTA  ");
    let yellow_display: String<64> = String::<64>::from("YELLOW  ");
    let white_display: String<64> = String::<64>::from("WHITE");

    let mut offset = 0;

    offset = write_text(offset, &zinc_version_display, CYAN);

    offset = write_text(offset, &black_display, BLACK);
    offset = write_text(offset, &blue_display, BLUE);
    offset = write_text(offset, &green_display, GREEN);
    offset = write_text(offset, &cyan_display, CYAN);
    offset = write_text(offset, &red_display, RED);
    offset = write_text(offset, &magenta_display, MAGENTA);
    offset = write_text(offset, &brown_display, BROWN);
    offset = write_text(offset, &light_gray_display, LIGHT_GRAY);

    offset = write_text(offset, &dark_gray_display, DARK_GRAY);
    offset = write_text(offset, &light_blue_display, LIGHT_BLUE);
    offset = write_text(offset, &light_green_display, LIGHT_GREEN);
    offset = write_text(offset, &light_cyan_display, LIGHT_CYAN);
    offset = write_text(offset, &light_red_display, LIGHT_RED);
    offset = write_text(offset, &light_magenta_display, LIGHT_MAGENTA);
    offset = write_text(offset, &yellow_display, YELLOW);
    offset = write_text(offset, &white_display, WHITE);

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}