//! A custom test runner. We can't use the rust's test runnner
//! since it relies upon std lib.

use crate::{serial_print, serial_println};
use core::{panic::PanicInfo, any};

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::io::{exit, ExitCode};
    serial_println!("[Tests] Executing tests...");
    for test in tests {
        test.run();
    }

    exit(ExitCode::Success);
}

/// This panic handler is only used when running tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::exit;
    serial_println!("[Panic] A panic occured in kernal...\nError: {}", info);
    exit(crate::ExitCode::Failed);
    loop {}
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[test_case]
pub fn test_test_runner() {
    assert_eq!(1, 1);
}
