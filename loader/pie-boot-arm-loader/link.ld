ENTRY(_start)

__page_size = 0x1000;

SECTIONS
{
    /* . = 0xF00000000000; */
    . = 0x40200040;
    .text : ALIGN(__page_size) {
        *(.text.init)
        *(.text .text.*)
    } 

    .rodata : ALIGN(__page_size) {
        *(.rodata .rodata.*)
        *(.srodata*)
        *(.got .got.*)
    } 
    
    .data : ALIGN(__page_size) {
        *(.data .data.*)
        . = ALIGN(__page_size);
        *(.sdata.*)
    } 

    .bss (NOLOAD) : ALIGN(__page_size)  {
        __start_boot_bss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        . = ALIGN(__page_size);
        __stop_boot_bss = .;
    } 

	/DISCARD/ : {
        *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) 
        *(.hash)
		*(.interp)
    }
} 

