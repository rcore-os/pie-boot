#![no_std]
#![no_main]

use core::arch::naked_asm;

mod console;
#[cfg(feature = "console")]
mod debug;
#[cfg(el = "1")]
mod el1;
#[cfg(el = "2")]
mod el2;
mod lang_items;
mod mmu;
mod paging;

use aarch64_cpu::{asm::barrier, registers::*};
#[cfg(el = "1")]
use el1::*;
#[cfg(el = "2")]
use el2::*;
use mmu::enable_mmu;
use pie_boot_if::BootArgs;
use pie_boot_loader_macros::println;

const MB: usize = 1024 * 1024;

/// The header of the kernel.
///
/// # Safety
///
/// ## arguments
///
/// x0: bootargs
///
/// ## return
///
/// x0: dtb
///
/// x1: tmp boot table end address
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[unsafe(link_section = ".text.init")]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        "
        mov   x19, x0 
        adr   x0, __stack_top
        mov   sp, x0

        mov   x0, x19
        BL    {switch_to_elx}",

        "mov   x0, x19",
        "BL     {entry}",
        "mov   x8,  x0",

        "mov   x0, x19",
        "BR    x8",
        switch_to_elx = sym switch_to_elx,
        entry = sym entry,
    )
}

fn entry(args: *mut BootArgs) -> *mut () {
    let bootargs = unsafe { &*args }.clone();
    enable_fp();
    unsafe {
        clean_bss();

        #[cfg(feature = "console")]
        debug::fdt::init_debugcon(bootargs.args[0] as _);

        println!("EL {}", CurrentEL.read(CurrentEL::EL));

        println!("_start   : {}", bootargs.kimage_addr_vma);

        let code_offset = bootargs.kimage_addr_vma as usize - bootargs.kimage_addr_lma as usize;

        enable_mmu(
            code_offset as _,
            bootargs.kimage_addr_lma as _,
            bootargs.kcode_end as _,
        );
    }
    bootargs.virt_entry
}

#[inline]
fn enable_fp() {
    CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
    barrier::isb(barrier::SY);
}

unsafe fn clean_bss() {
    unsafe extern "C" {
        fn __start_boot_bss();
        fn __stop_boot_bss();
    }
    unsafe {
        let start = __start_boot_bss as *mut u8;
        let end = __stop_boot_bss as *mut u8;
        let len = end as usize - start as usize;
        for i in 0..len {
            start.add(i).write(0);
        }
    }
}
