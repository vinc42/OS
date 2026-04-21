#![no_main]
#![no_std]

pub mod shell;
pub mod io;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    io::print(b"On est la");
    let f: unsafe fn() = unsafe { core::mem::transmute(0x40200000 as usize) };
    unsafe { f() };
    io::print(b"On est plus la");
    //shell::shell();
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    io::print(b"Panic!\n");
    loop {}
}
