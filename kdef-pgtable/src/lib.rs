#![cfg_attr(not(test), no_std)]

include!(concat!(env!("OUT_DIR"), "/constant.rs"));

pub const SZ_1G: usize = 1024 * SZ_1M;
pub const SZ_2G: usize = 2 * SZ_1G;
pub const SZ_1M: usize = 1024 * 1024;
pub const SZ_2M: usize = 2 * SZ_1M;
pub const SZ_8M: usize = 8 * SZ_1M;
pub const SZ_16M: usize = 16 * SZ_1M;

pub const PAGE_SIZE: usize = 1usize << PAGE_SHIFT;

const MODULES_VADDR: usize = _page_end(PG_VA_BITS);
const MODULES_VSIZE: usize = (1usize << PG_VA_BITS) / 0x10 * 0x8;

pub const KIMAGE_VADDR: usize = MODULES_VADDR + MODULES_VSIZE;

const fn _page_offset(va: usize) -> usize {
    !((1usize << va) - 1)
}

const fn _page_end(va: usize) -> usize {
    !((1usize << va) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
