// use this for testing purpose
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

mod vga_buffer;

use core::panic::PanicInfo;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");
    
    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}