#![no_std]

use core::{mem::MaybeUninit, num::NonZero};

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
    /// 页表结束物理地址
    pub pg_end: usize,
}

impl BootArgs {
    pub const fn new() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl Default for BootArgs {
    fn default() -> Self {
        Self::new()
    }
}
