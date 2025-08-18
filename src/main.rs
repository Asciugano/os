#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use os::println;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    use os::memory;
    use os::memory::BootInfoFrameAllocator;
    use x86_64::{VirtAddr, structures::paging::Page};

    println!("Hello World{}", "!");
    os::init();

    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&_boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) }

    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     _boot_info.physical_memory_offset,
    // ];
    //
    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    #[cfg(test)]
    test_main();

    println!("It did't crash");

    os::hlt_loop();
}

// #[unsafe(no_mangle)]
// pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
//     println!("Hello World{}", "!");
//
//     os::init();
//
//     use x86_64::registers::control::Cr3;
//
//     let (level_4_page_table, _) = Cr3::read();
//     println!(
//         "Level 4 page table at: {:?}",
//         level_4_page_table.start_address()
//     );
//
//     #[cfg(test)]
//     test_main();
//
//     println!("It did't crash");
//
//     os::hlt_loop();
// }

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    os::test_panic_handler(_info)
}
