#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use core::panic::PanicInfo;
use flerp_os::{serial_print, serial_println, exit_qemu, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\n");

    flerp_os::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

//------------------------------------------------
//
// Panic Logic
//
//------------------------------------------------

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    flerp_os::test_panic_handler(info);
}

//------------------------------------------------
//
// Tests
//
//------------------------------------------------

// infinit recursion aka. stack overflow!
#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

// IDT Test
lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(flerp_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);

    loop {}
}

pub fn init_test_idt() {
    TEST_IDT.load();
}
