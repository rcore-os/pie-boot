. = {kimage_vaddr};
PAGE_SIZE = {page_size};

SECTIONS{
    .head.text : {
        _text = .;
        KEEP(*(.head.text))
    }
    .idmap.text : ALIGN(64){
        _idmap_text = .;
        KEEP(*(.idmap.text))
    }
}

SECTIONS{ 
    .boot_loader  : {
        KEEP(*(.boot_loader))
    }
} INSERT BEFORE .data

ASSERT(PAGE_SIZE != 0x0, "PAGE_SIZE must be defined");