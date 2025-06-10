use core::arch::naked_asm;

def_adr_l!();

mod cache;

use crate::start_code;
use kasm_aarch64::{self as kasm, def_adr_l};

const FLAG_LE: usize = 0b0;
const FLAG_PAGE_SIZE_4K: usize = 0b10;
const FLAG_ANY_MEM: usize = 0b1000;

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

#[start_code(naked)]
fn primary_entry() -> ! {
    naked_asm!(
        "
    bl  {preserve_boot_args}
	adrp	x1, __early_stack_top
	mov	sp, x1
	mov	x29, xzr
	adrp	x0, init_idmap_pg_dir
	mov	x1, xzr
    bl   {create_idmap}
        ",
        preserve_boot_args = sym preserve_boot_args,
        create_idmap = sym create_init_idmap,
    )
}

#[start_code(naked)]
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
    boot_args = sym crate::BOOT_ARGS,
    dcache_inval_poc = sym cache::__dcache_inval_poc,
    )
}

#[start_code]
fn create_init_idmap() {
    let a = 1;
}
