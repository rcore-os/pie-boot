use core::ptr::NonNull;

use fdt_parser::Fdt;
use pie_boot_if::{MemoryRegion, MemoryRegionKind};

use crate::MEMORY_REGIONS;

pub fn setup(ptr: NonNull<u8>) -> Option<()> {
    let fdt = Fdt::from_ptr(ptr).ok()?;
    for memory in fdt.memory() {
        for region in memory.regions() {
            let start = region.address as _;
            let v = MemoryRegion {
                start,
                end: start + region.size,
                kind: MemoryRegionKind::Ram,
            };

            MEMORY_REGIONS.as_mut().push(v).ok()?;
        }
    }

    for region in fdt.memory_reservation_block() {
        let start = region.address as _;
        let v = MemoryRegion {
            start,
            end: start + region.size,
            kind: MemoryRegionKind::Ram,
        };
        MEMORY_REGIONS.as_mut().push(v).ok()?;
    }

    Some(())
}
