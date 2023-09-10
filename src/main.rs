#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod io;
mod tests;

/// On bare metal, you have to handle the panics youself.
/// Imagine if kernel panickned, Who would unwind the stack? There
/// is nobody running the kernel. Kernel is the sole owner of the
/// machine that takes the control from the bootloader.
/// Therefore, it doesn't make sense for kernal to panic &
/// you have to handle the panic yourself for kernal.
use core::panic::PanicInfo;
use crate::io::vga_buffer;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


/// This is where it gets intresting. The _start is the default entry point
/// that the bootloader library calls. We gotta tell the bootloader, "Hey! Our program begins
/// here!". "no mangle" stuff tells the rust compiler to not do generate a cryptic name
/// for our function. Wihtout no mangle, it would generate something like
/// fjspa349029aas_start_asjodbob & system won't be able to find our _start. geddit?
///
/// If you were implementing your own bootloader, you could change this behaviour & call
/// your starting function whatever you like. Just know, that bootloader doesn't really know
/// that the function exists in your kernal. It just calls it cause you said so.
#[no_mangle]
pub extern "C" fn _start() -> ! {

    #[cfg(test)]
    test_main();

    print!("hello ayush! brrr grrr RAWWWWWWWWWWWWWWWWWWR :3");
    loop {}
}
