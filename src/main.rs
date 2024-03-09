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
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;

// main logic
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
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

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

//------------------------------------------------
//
// Test Logic
//
//------------------------------------------------

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// simple test runner for debugging and other stuff
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

// test to test the test runnner? lol
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

//------------------------------------------------
//
// Qemu Logic
//
//------------------------------------------------

// make qemu competible with the test runner so it does not crash or anything like that!
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
