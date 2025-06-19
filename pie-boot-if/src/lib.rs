#![no_std]

use core::{fmt::Debug, mem::MaybeUninit, ptr::NonNull};

mod memregions;

pub use memregions::*;

#[repr(C, align(64))]
#[derive(Clone)]
pub struct EarlyBootArgs {
    pub args: [usize; 4],
    pub virt_entry: *mut (),
    pub kimage_addr_lma: *mut (),
    pub kimage_addr_vma: *mut (),
    pub kcode_end: *mut (),
}

impl EarlyBootArgs {
    pub const fn new() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl Default for EarlyBootArgs {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(align(64))]
#[derive(Clone)]
pub struct BootInfo {
    /// CPU 硬件ID
    pub cpu_id: usize,
    /// 内核镜像物理地址
    pub kimage_start_lma: usize,
    /// 内核镜像虚拟地址
    pub kimage_start_vma: usize,
    /// 设备树虚拟地址
    pub fdt: Option<NonNull<u8>>,
    /// 页表开始物理地址
    pub pg_start: usize,
    /// 内存区域
    pub memory_regions: MemoryRegions,
}

impl Debug for BootInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BootInfo")
            .field("cpu_id", &self.cpu_id)
            .field("fdt", &self.fdt)
            .field("kimage_start_lma", &(self.kimage_start_lma as *mut u8))
            .field("kimage_start_vma", &(self.kimage_start_vma as *mut u8))
            .field("pg_start", &(self.pg_start as *mut u8))
            .field("memory_regions", &self.memory_regions)
            .finish()
    }
}

unsafe impl Send for BootInfo {}
unsafe impl Sync for BootInfo {}

impl BootInfo {
    pub const fn new() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl Default for BootInfo {
    fn default() -> Self {
        Self::new()
    }
}
