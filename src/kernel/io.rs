const UART: *mut u8 = 0x0900_0000 as *mut u8;
const UART_FR: *mut u32 = (0x0900_0000 + 0x18) as *mut u32;


pub fn putchar(c: u8) {
    unsafe {
        *UART = c;
    }
}

pub fn print(s: &[u8]) {
    for &c in s {
        putchar(c);
    }
}

pub fn getchar() -> u8   {
    unsafe {
        while (*UART_FR & 0x10) != 0 {
            core::arch::asm!("nop");
        }
        *UART
    }
}
