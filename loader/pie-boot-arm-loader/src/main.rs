#![no_std]
#![no_main]

use core::arch::{asm, naked_asm};

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
        adr   x0, {stack}
        add   x0, x0, {stack_size}
        mov   sp, x0

        mov   x0, x19
        BL    {switch_to_elx}",

        "mov   x0, x19",
        "BL     {entry}",
        "mov   x8,  x0",

        "mov   x0, x19",
        "BR    x8",
        stack = sym STACK,
        stack_size = const STACK.len(),
        switch_to_elx = sym switch_to_elx,
        entry = sym entry,
    )
}

fn entry(bootargs: &BootArgs) -> *mut () {
    enable_fp();
    unsafe {
        clean_bss();

        #[cfg(feature = "console")]
        debug::fdt::init_debugcon(bootargs.args[0] as _);

        println!("EL {}", CurrentEL.read(CurrentEL::EL));
        println!("bootargs : {}", bootargs as *const _ as usize);
        println!("_start   : {}", bootargs.kimage_addr_vma);
        println!("_end     : {}", bootargs.kcode_end);
        let loader_at = loader_at();

        println!(
            "loader   : [{}, {})",
            loader_at,
            loader_at.add(loader_size())
        );

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

fn loader_size() -> usize {
    unsafe extern "C" {
        fn _stext();
        fn _end();
    }
    _end as usize - _stext as usize
}
fn loader_at() -> *mut u8 {
    let at;
    unsafe {
        asm!("
        adrp {0}, _stext
        add  {0}, {0}, :lo12:_stext
        ",
        out(reg) at
        );
    }
    at
}

#[unsafe(link_section = ".stack")]
static STACK: [u8; 0x1000] = [0; 0x1000];
