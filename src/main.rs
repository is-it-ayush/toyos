// this how we tell rust to not use the std lib.
#![no_std]

// this how we tell rust to not use main as the entry point. cause someone needs to
// call main right? & bare metal stuff aint got nobody calling main(). so we ain't
// use that.
#![no_main]

/// Also we prolly need to handle panics ourselves. Cause what if kernal go BRRR???
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// This is where it gets intresting. The _start is the default entry point
/// for most systems. We gotta tell the system, "Hey! Our program begins here!"
/// "no mangle" stuff tells the rust compiler to not do generate a cryptic name
/// for our function. Wihtout no mangle, it would generate something like
/// fjspa349029aas_start_asjodbob & system won't be able to find our _start. geddit?
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
