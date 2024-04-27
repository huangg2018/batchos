OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x0;

SECTIONS
{
    . = BASE_ADDRESS;
    .text : {
        *(.text .text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4K);

    .data : {
        *(.data .data.*)
        }
    . = ALIGN(4K);
    
    .bss : {
        start_bss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        end_bss = .;
        }

    . = ALIGN(4K);

    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}
