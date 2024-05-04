// build
// cargo build
// run
// cargo run --bin qemu-uefi 
// bios not supported anymore return to 0.1.0 for bios support

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flerp_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader_api::{entry_point, BootInfo};
use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::info::FrameBuffer;
use core::panic::PanicInfo;
use kernel::println;
use x86_64::VirtAddr;

extern crate alloc;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

// entry point for kernel
entry_point!(kernel_main, config=&BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    use kernel::allocator;
    use kernel::memory::{self, BootInfoFrameAllocator};
    use kernel::frame_buffer;

    //println!("Hello World{}", "!");
    kernel::init();
    
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let mut buffer_writer = frame_buffer::Writer {
            framebuffer: framebuffer.buffer_mut(),
            info:  
            pos_x: 10,
            pos_y: 100,
        };
        buffer_writer.clear();
    }

    
    // why the frick did they turn physical... into a fucking Optional!!! not Option no OPTIONAL!!!!
    // AHHHHHHHHHHHHHHHHH
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    
    #[cfg(test)]
    test_main();

    //println!("It did not crash!");
    kernel::hlt_loop();
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
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
