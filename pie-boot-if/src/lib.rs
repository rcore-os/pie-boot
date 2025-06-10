#![no_std]

use core::mem::MaybeUninit;

#[repr(C, align(64))]
#[derive(Clone)]
pub struct BootArgs {
    pub args: [usize; 4],
    pub virt_entry: *mut (),
    pub kimage_addr_lma: *mut (),
    pub kimage_addr_vma: *mut (),
    pub kcode_end: *mut (),
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
