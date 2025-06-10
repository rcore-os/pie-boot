. = {kimage_vaddr};
PROVIDE(PAGE_SIZE  = 0x0);

SECTIONS{
    .head.text : {
        _text = .;
        KEEP(*(.head.text))
    }
    .idmap.text : ALIGN(0x1000){
        _idmap_text = .;
        KEEP(*(.idmap.text))
    }
}

SECTIONS{ 
    .boot_loader  : {
        KEEP(*(.boot_loader))
    }
} INSERT BEFORE .data

SECTIONS{
    .init_idmap_pg_dir  : ALIGN(PAGE_SIZE){
        init_idmap_pg_dir = .;
        . += PAGE_SIZE;
    }
    .early_stack : {
        . += 0x1000;
        __early_stack_top = .;
    }

}  INSERT AFTER .bss

ASSERT(PAGE_SIZE != 0x0, "PAGE_SIZE must be defined");