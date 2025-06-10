ENTRY(_start)

__page_size = {PAGE_SIZE};

SECTIONS
{
    /* . = 0xF40202000; */
    . = 0x40203000;
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
        . += __page_size;
        __stack_top = .;
    } 

	/DISCARD/ : {
        *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) 
        *(.hash)
		*(.interp)
    }
} 

