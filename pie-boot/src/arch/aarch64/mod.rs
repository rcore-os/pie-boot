use core::arch::{global_asm, naked_asm};

mod cache;

use crate::start_code;
use kasm_aarch64 as kasm;

const FLAG_LE: usize = 0b0;
const FLAG_PAGE_SIZE_4K: usize = 0b10;
const FLAG_ANY_MEM: usize = 0b1000;

#[repr(C, align(64))]
pub struct BootArgs {
    pub fdt: u64,
    pub rsv1: u64,
    pub rsv2: u64,
    pub rsv3: u64,
}

pub static mut BOOT_ARGS: BootArgs = BootArgs {
    fdt: 0,
    rsv1: 0,
    rsv2: 0,
    rsv3: 0,
};

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".head.text")]
/// The header of the kernel.
///
/// # Safety
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        // code0/code1
        "nop",
        "bl {entry}",
        // text_offset
        ".quad 0",
        // image_size
        ".quad __kernel_load_end - _start",
        // flags
        ".quad {flags}",
        // Reserved fields
        ".quad 0",
        ".quad 0",
        ".quad 0",
        // magic - yes 0x644d5241 is the same as ASCII string "ARM\x64"
        ".ascii \"ARM\\x64\"",
        // Another reserved field at the end of the header
        ".byte 0, 0, 0, 0",
        flags = const FLAG_LE | FLAG_PAGE_SIZE_4K | FLAG_ANY_MEM,
        entry = sym primary_entry,
    )
}

global_asm!(
    r"
	.macro	adr_l, dst, sym
	adrp	\dst, \sym
	add	\dst, \dst, :lo12:\sym
	.endm
"
);

#[start_code]
fn primary_entry() -> ! {
    naked_asm!(
        "
        bl  {preserve_boot_args}
        ",
        preserve_boot_args = sym preserve_boot_args,
    )
}

#[start_code]
fn preserve_boot_args() -> ! {
    naked_asm!(
        "
	mov	x21, x0				// x21=FDT

	adr_l	x0, {boot_args}			// record the contents of
	stp	x21, x1, [x0]			// x0 .. x3 at kernel entry
	stp	x2, x3, [x0, #16]

	dmb	sy				// needed before dc ivac with
						// MMU off

	add	x1, x0, #0x20			// 4 x 8 bytes
	b	{dcache_inval_poc}		// tail call
        ",
    boot_args = sym BOOT_ARGS,
    dcache_inval_poc = sym cache::__dcache_inval_poc,
    )
}
