#![test_runner(os::test_runner)]

use core::panic::PanicInfo;
use os::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    os::test_panic_handler(_info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
