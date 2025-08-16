#![no_std]
#![no_main]

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello, World!";

mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("test");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
