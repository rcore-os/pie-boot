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
use pie_boot_if::{BootArgs, EarlyBootArgs};
use pie_boot_loader_macros::println;

const MB: usize = 1024 * 1024;

#[unsafe(link_section = ".stack")]
static STACK: [u8; 0x8000] = [0; 0x8000];

static mut RUTERN: BootArgs = BootArgs::new();

/// The header of the kernel.
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[unsafe(link_section = ".text.init")]
unsafe extern "C" fn _start(_args: &EarlyBootArgs) -> ! {
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

        "
        adrp x0, {res}
        add  x0, x0, :lo12:{res}
        br   x8
        ",
        stack = sym STACK,
        stack_size = const STACK.len(),
        switch_to_elx = sym switch_to_elx,
        entry = sym entry,
        res = sym RUTERN,
    )
}

fn entry(bootargs: &EarlyBootArgs) -> *mut () {
    enable_fp();
    unsafe {
        clean_bss();
        RUTERN.fdt = bootargs.args[0];

        #[cfg(feature = "console")]
        debug::fdt::init_debugcon(RUTERN.fdt as _);
        println!("fdt            : {}", RUTERN.fdt);

        println!("EL             : {}", CurrentEL.read(CurrentEL::EL));

        println!("_start         : {}", bootargs.kimage_addr_vma);

        let loader_at = loader_at();

        println!(
            "loader         : [{}, {})",
            loader_at,
            loader_at.add(loader_size())
        );

        enable_mmu(bootargs);

        println!("mmu ok");
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
