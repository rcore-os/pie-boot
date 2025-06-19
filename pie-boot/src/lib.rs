#![cfg_attr(target_os = "none", no_std)]
#![cfg(target_os = "none")]

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;

mod loader;
mod staticcell;

use pie_boot_if::EarlyBootArgs;
pub use pie_boot_if::{BootInfo, MemoryRegion, MemoryRegionKind, MemoryRegions};
pub use pie_boot_macros::entry;
#[allow(unused)]
use pie_boot_macros::start_code;

#[allow(unused)]
static mut BOOT_ARGS: EarlyBootArgs = EarlyBootArgs::new();

unsafe extern "Rust" {
    fn __pie_boot_main(args: &BootInfo);
}

fn virt_entry(args: &BootInfo) {
    unsafe {
        __pie_boot_main(args);
    }
}
