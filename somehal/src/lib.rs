#![no_std]

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;

pub mod early_debug;
mod fdt;
mod mem;
pub mod power;
mod static_once;
pub use pie_boot::BootInfo;

#[pie_boot::entry]
fn main(args: &BootInfo) -> ! {
    mem::clean_bss();
    fdt::init_debugcon(args.fdt);
    arch::power::init_by_fdt(args.fdt);

    unsafe extern "Rust" {
        fn __somehal_main(args: &BootInfo);
    }
    unsafe {
        __somehal_main(args);
    }
    power::shutdown();
}
