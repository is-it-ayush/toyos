#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(toyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod io;

use core::panic::PanicInfo;

/// On bare metal, you have to handle the panics youself.
/// Imagine if kernel panickned, Who would unwind the stack? There
/// is nobody running the kernel. Kernel is the sole owner of the
/// machine that takes the control from the bootloader.
/// Therefore, it doesn't make sense for kernal to panic &
/// you have to handle the panic yourself for kernal.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    toyos::test_panic_handler(info)
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
