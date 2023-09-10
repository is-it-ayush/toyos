// this how we tell rust to not use the std lib.
#![no_std]
// this how we tell rust to not use main as the entry point. cause someone needs to
// call main right? & bare metal stuff aint got nobody calling main(). so we ain't
// use that.
#![no_main]

mod io;

/// Also we prolly need to handle panics ourselves. Cause what if kernal go BRRR???
use core::{panic::PanicInfo, fmt::Write};

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
    println!("hello ayush! brrr grrr RAWWWWWWWWWWWWWWWWWWR :3");
    loop {}
}
