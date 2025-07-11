#![no_std]
#![cfg(target_os = "none")]

use core::hint::spin_loop;

use log::{debug, info};
use pie_boot::BootInfo;

use crate::debug::init_log;

extern crate pie_boot;

mod debug;
pub mod lang_items;

#[pie_boot::entry]
fn main(args: &BootInfo) -> ! {
    clean_bss();

    init_log(args.fdt.unwrap().as_ptr());

    debug!("boot args: {:?}", args);

    info!("All tests passed!");

    #[cfg(feature = "qemu")]
    exit();

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

#[cfg(feature = "qemu")]
fn exit() {
    use qemu_exit::QEMUExit;
    #[cfg(target_arch = "aarch64")]
    let qemu_exit_handle = qemu_exit::AArch64::new();

    // addr: The address of sifive_test.
    #[cfg(target_arch = "riscv64")]
    let qemu_exit_handle = qemu_exit::RISCV64::new(addr);

    // io_base:             I/O-base of isa-debug-exit.
    // custom_exit_success: A custom success code; Must be an odd number.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    let qemu_exit_handle = qemu_exit::X86::new(io_base, custom_exit_success);

    qemu_exit_handle.exit_success();
}
