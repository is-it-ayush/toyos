use x86_64::instructions::port::Port;

pub mod vga_buffer;
pub mod serial;


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
