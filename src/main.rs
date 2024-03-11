// use this for special purpose?
// cargo build --target x86_64-flerp_os.json
// if it not works use this command
// cargo build --target thumbv7em-none-eabihf

// build
// 1: cargo install bootimage
// 2: cargo bootimage
// 3: cargo build (start from here if u did steps above)
// 4: cargo run

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flerp_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use flerp_os::println;

// entry point for kernel
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use flerp_os::memory;
    use flerp_os::memory::BootInfoFrameAllocator;
    use x86_64::{
        structures::paging::{Page, Translate},
        VirtAddr,
    };

    println!("Hello World{}", "!");
    flerp_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    flerp_os::hlt_loop();
}

//------------------------------------------------
//
// Panic Logic
//
//------------------------------------------------

// This function is called on panic when not testing
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    flerp_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    flerp_os::test_panic_handler(info)
}
