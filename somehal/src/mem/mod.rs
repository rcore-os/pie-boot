pub fn clean_bss() {
    unsafe extern "C" {
        fn _sbss();
        fn _ebss();
    }
    unsafe {
        let bss =
            core::slice::from_raw_parts_mut(_sbss as *mut u8, _ebss as usize - _sbss as usize);
        bss.fill(0);
    }
}
