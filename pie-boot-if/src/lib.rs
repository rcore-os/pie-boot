#![no_std]

use core::{fmt::Debug, mem::MaybeUninit};

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
pub struct BootArgs {
    /// 设备树物理地址
    pub fdt: usize,
    /// 内核镜像物理地址
    pub kimage_start_lma: usize,
    /// 内核镜像虚拟地址
    pub kimage_start_vma: usize,
    /// 页表开始物理地址
    pub pg_start: usize,
    /// 内存保留区域开始物理地址
    pub rsv_start: usize,
    /// 内存保留区域结束物理地址
    pub rsv_end: usize,
}

impl Debug for BootArgs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BootArgs")
            .field("fdt", &(self.fdt as *mut u8))
            .field("kimage_start_lma", &(self.kimage_start_lma as *mut u8))
            .field("kimage_start_vma", &(self.kimage_start_vma as *mut u8))
            .field("pg_start", &(self.pg_start as *mut u8))
            .field("rsv_start", &(self.rsv_start as *mut u8))
            .field("rsv_end", &(self.rsv_end as *mut u8))
            .finish()
    }
}

impl BootArgs {
    pub const fn new() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
    pub fn fdt_addr(&self) -> *mut u8 {
        self.fdt as *mut u8
    }
}

impl Default for BootArgs {
    fn default() -> Self {
        Self::new()
    }
}
