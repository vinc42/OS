#![no_main]
#![no_std]

use core::panic::PanicInfo;

const UART: *mut u8 = 0x0900_0000 as *mut u8;
const UART_FR: *mut u32 = (0x0900_0000 + 0x18) as *mut u32;


#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    print(b"Hello, from Rust!\n");
    loop {
        let c = getchar();
        print(b"my char : ");
        putchar(c);
        print(b"\n");
    }
}

fn putchar(c: u8) {
    unsafe {
        *UART = c;
    }
}

fn print(s: &[u8]) {
    for &c in s {
        putchar(c);
    }
}

fn getchar() -> u8   {
    unsafe {
        while (*UART_FR & 0x10) != 0 {
            core::arch::asm!("nop");
        }
        *UART
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    print(b"Panic!\n");
    loop {}
}
