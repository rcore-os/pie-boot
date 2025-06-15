#![no_std]
#![cfg(target_os = "none")]

use core::hint::spin_loop;

use log::{debug, info};
use pie_boot::BootArgs;

use crate::debug::init_log;

extern crate pie_boot;

mod debug;
pub mod lang_items;

#[pie_boot::entry]
fn main(args: &BootArgs) -> ! {
    clean_bss();

    init_log(args.fdt as _);

    debug!("boot args: {:?}", args);

    info!("All tests passed!");

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
