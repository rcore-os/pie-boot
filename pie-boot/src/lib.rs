#![cfg_attr(target_os = "none", no_std)]
#![cfg(target_os = "none")]

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;

mod fdt;
mod loader;
mod staticcell;

use heapless::Vec;
pub use kdef_pgtable::{KIMAGE_VADDR, KLINER_OFFSET};
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

        mainmem_start_rsv(args);
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

fn mainmem_start_rsv(args: &BootInfo) {
    let lma = args.kimage_start_lma as usize;

    let mainmem = MEMORY_REGIONS.iter().find(|r| {
        let is_ram = matches!(r.kind, MemoryRegionKind::Ram);
        let in_range = r.start <= lma && r.end > lma;
        is_ram && in_range
    });

    let Some(mainmem) = mainmem else {
        return;
    };

    let mut start = mainmem.start;
    unsafe extern "C" {
        fn _idmap_text_end();
    }
    let mut end = _idmap_text_end as usize - args.kcode_offset();

    // 收集需要移除的 reserved 区域的索引
    let mut indices_to_remove: heapless::Vec<usize, 16> = heapless::Vec::new();

    // 遍历现有的 reserved 区域，调整新区域的范围以排除重叠部分
    for (i, r) in MEMORY_REGIONS.iter().enumerate() {
        if !matches!(r.kind, MemoryRegionKind::Reserved) {
            continue;
        }

        // 检查是否有重叠
        if !(end <= r.start || start >= r.end) {
            // 如果现有 reserved 区域完全包含了新区域，则无需添加
            if r.start <= start && r.end >= end {
                return;
            }

            // 如果现有 reserved 区域完全在新区域中间，标记移除
            if r.start >= start && r.end <= end {
                let _ = indices_to_remove.push(i);
                continue;
            }

            // 如果现有 reserved 区域与新区域的开始部分重叠
            if r.start <= start && r.end > start && r.end < end {
                start = r.end;
            }

            // 如果现有 reserved 区域与新区域的结束部分重叠
            if r.start > start && r.start < end && r.end >= end {
                end = r.start;
            }
        }
    }

    // 从后往前移除标记的区域（避免索引变化问题）
    for &i in indices_to_remove.iter().rev() {
        MEMORY_REGIONS.as_mut().swap_remove(i);
    }

    // 检查调整后的区域是否仍然有效
    if start >= end {
        return;
    }

    // 添加新的 reserved 区域
    let _ = MEMORY_REGIONS.as_mut().push(MemoryRegion {
        kind: MemoryRegionKind::Reserved,
        start,
        end,
    });
}
