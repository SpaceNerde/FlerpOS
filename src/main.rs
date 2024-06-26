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
use x86_64::VirtAddr;

extern crate alloc;

// entry point for kernel
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use flerp_os::allocator;
    use flerp_os::memory::{self, BootInfoFrameAllocator};

    println!("Hello World{}", "!");
    flerp_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

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
