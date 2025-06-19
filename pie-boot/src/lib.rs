#![cfg_attr(target_os = "none", no_std)]
#![cfg(target_os = "none")]

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;

mod fdt;
mod loader;
mod staticcell;

use heapless::Vec;
use pie_boot_if::EarlyBootArgs;
pub use pie_boot_if::{BootInfo, MemoryRegion, MemoryRegionKind, MemoryRegions};
pub use pie_boot_macros::entry;
#[allow(unused)]
use pie_boot_macros::start_code;
use staticcell::StaticCell;

#[allow(unused)]
static mut BOOT_ARGS: EarlyBootArgs = EarlyBootArgs::new();

#[unsafe(link_section = ".data")]
static BOOT_INFO: StaticCell<BootInfo> = StaticCell::new(BootInfo::new());
#[unsafe(link_section = ".data")]
static MEMORY_REGIONS: StaticCell<Vec<MemoryRegion, 128>> = StaticCell::new(Vec::new());

unsafe extern "Rust" {
    fn __pie_boot_main(args: &BootInfo);
}

fn virt_entry(args: &BootInfo) {
    unsafe {
        MEMORY_REGIONS.as_mut().clear();
        let _ = MEMORY_REGIONS
            .as_mut()
            .extend_from_slice(&args.memory_regions);

        *BOOT_INFO.as_mut() = args.clone();

        if let Some(ptr) = BOOT_INFO.fdt {
            fdt::setup(ptr);
        }

        let regions = core::slice::from_raw_parts_mut(
            MEMORY_REGIONS.as_mut().as_mut_ptr(),
            MEMORY_REGIONS.len(),
        );

        BOOT_INFO.as_mut().memory_regions = regions.into();

        __pie_boot_main(&BOOT_INFO);
    }
}

pub fn boot_info() -> &'static BootInfo {
    &BOOT_INFO
}
