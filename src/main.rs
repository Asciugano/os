#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    os::init();

    #[cfg(test)]
    test_main();

    println!("It did't crash");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    os::test_panic_handler(_info)
}
