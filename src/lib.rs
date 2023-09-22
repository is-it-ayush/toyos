#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod io;

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) -> () {
        serial_print!("[Tests] {}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("[Tests] Executing {} tests...", tests.len());
    for test in tests {
        test.run();
    }
    exit(ExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[Tests] A panic occured while testing...\nError: {}", info);
    exit(crate::ExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    Failed = 0x11
}

/// An exit fucntion to exit with a status code.
pub fn exit(exit_code: ExitCode) {
    unsafe {
        // This is only for qemu. Might implemenent a proper exit system later.
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
