#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(toyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use toyos::{println, io::vga_buffer::{WRITER, BUFFER_HEIGHT}};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    toyos::test_panic_handler(info)
}

#[test_case]
fn test_print() {
    println!("test_print output");
}

#[test_case]
fn test_print_many() {
    for i in 0..200 {
        println!("test_print output:\t{}", i);
    }
}

#[test_case]
fn test_print_screen_output() {
    let s = "This should appear onto screen.";
    println!("{}",s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
