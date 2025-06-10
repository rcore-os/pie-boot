#![no_std]

#[repr(C, align(64))]
#[derive(Clone)]
pub struct BootArgs {
    pub args: [usize; 4],
    pub virt_entry: usize,
    pub kimage_addr_lma: usize,
    pub kimage_addr_vma: usize,
    pub kcode_end: usize,
}

impl BootArgs {
    pub const fn new() -> Self {
        Self {
            args: [0; 4],
            virt_entry: 0,
            kimage_addr_lma: 0,
            kimage_addr_vma: 0,
            kcode_end: 0,
        }
    }
}

impl Default for BootArgs {
    fn default() -> Self {
        Self::new()
    }
}
