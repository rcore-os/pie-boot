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
use num_align::NumAlign;
use paging::TableGeneric;
use pie_boot_loader_macros::println;

const MB: usize = 1024 * 1024;
const STACK_SIZE: usize = 4 * MB;

/// The header of the kernel.
///
/// # Safety
///
/// ## arguments
///
/// x0: dtb
///
/// x1: kernel code end
///
/// x2: virt entry
///
/// x3: code offset
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
        "mov   x19, x0", // dtb
        "mov   x21, x2", // virt entry,
        "mov   x22, x3",
        "mov   x23, x4",

        "LDR    x16, =__page_size",
        "sub    x16, x16, #1",
        "ADD    x1, x1, x16",
        "bic    x1, x1, x16",
        "mov    x20, x1", // kernel code end align to page size

        "mov    x16, x20",
        "add   x16, x16, {stack_size}",
        "mov   sp,  x16",

        "BL    {switch_to_elx}",

        "mov   x0, x19",
        "mov   x1, x20",
        "mov   x2, x22",
        "mov   x3, x23",
        "BL     {entry}",
        "mov   x1, x0",

        "mov   x0, x19",
        "BR    x21",
        stack_size = const STACK_SIZE,
        switch_to_elx = sym switch_to_elx,
        entry = sym entry,
    )
}

fn entry(
    dtb: *mut u8,
    kernel_code_end: *mut u8,
    code_offset: *mut u8,
    kernel_start_lma: *mut u8,
) -> usize {
    enable_fp();
    unsafe {
        clean_bss();

        #[cfg(feature = "console")]
        debug::fdt::init_debugcon(dtb);

        println!("EL {}", CurrentEL.read(CurrentEL::EL));

        enable_mmu(
            code_offset as _,
            kernel_start_lma as _,
            kernel_code_end as _,
        );

        (kernel_code_end as usize).align_up(Table::PAGE_SIZE) + STACK_SIZE
    }
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
