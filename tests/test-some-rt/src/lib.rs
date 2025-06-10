#![no_std]

use core::hint::spin_loop;

use log::debug;

use crate::debug::init_log;

extern crate pie_boot;

mod debug;
pub mod lang_items;

#[unsafe(no_mangle)]
pub extern "C" fn _main() -> ! {
    clean_bss();
    

    init_log(0x44000000 as _);

    loop {
        spin_loop();
    }
}

fn clean_bss() {
    unsafe extern "C" {
        fn _sbss();
        fn _ebss();
    }
    unsafe {
        let bss =
            core::slice::from_raw_parts_mut(_sbss as *mut u8, _ebss as usize - _sbss as usize);
        bss.fill(0);
    }
}
