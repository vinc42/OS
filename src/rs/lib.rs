#![no_main]
#![no_std]

pub mod shell;
pub mod io;

use core::panic::PanicInfo;



#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    shell::shell();
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    io::print(b"Panic!\n");
    loop {}
}
