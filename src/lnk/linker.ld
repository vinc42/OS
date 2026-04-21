ENTRY(_start)
SECTIONS {
    . = 0x40100000;
    .text.init : {
        boot.o(.text)
    }
    . = 0x40200000;
    .text : {
        programs.o(.text)
        *(.text)
    }
    .data : {
        *(.data)
    }
    .bss : {
        *(.bss COMMON)
    }
    . = ALIGN(8);
    . += 0x1000;
    stack_top = .;
}
