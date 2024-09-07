#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum color_code {
    BLACK,
    BLUE,
    GREEN,
    CYAN,
    RED,
    MAGENTA,
    BROWN,
    LIGHTGRAY,
    DARKGRAY,
    LIGHTBLUE,
    LIGHTGREEN,
    LIGHTCYAN,
    LIGHTRED,
    LIGHTMAGENTA,
    YELLOW,
    WHITE
}


struct Color {
    foreground: color_code,
    background: color_code,
    actual_code: u8,
}

impl Color {
    pub fn new(foreground: color_code, background: color_code) -> Self {
        let mut actual_code = (background.clone() as u8) << 4;
        actual_code = actual_code | (foreground.clone() as u8);


        Self { foreground, background, actual_code }

    }   
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            let black_on_white = Color::new(color_code::BLACK, color_code::WHITE);
            *vga_buffer.offset(i as isize * 2 + 1) = black_on_white.actual_code;
        }
    }

    loop {}
}


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}