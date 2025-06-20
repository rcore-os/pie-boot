use core::ptr::NonNull;

use fdt_parser::Fdt;
use kdef_pgtable::PAGE_SIZE;
use num_align::NumAlign;

use crate::debug::REG_BASE;

pub fn init_debugcon(fdt: *mut u8) -> Option<()> {
    fn phys_to_virt(p: usize) -> *mut u8 {
        p as _
    }
    let fdt = Fdt::from_ptr(NonNull::new(fdt)?).ok()?;
    let choson = fdt.chosen()?;
    let node = choson.debugcon()?;

    let uart = any_uart::Uart::new_by_fdt_node(&node, phys_to_virt)?;

    let reg = node.reg()?.next()?;

    unsafe {
        REG_BASE = reg.address.align_down(PAGE_SIZE) as _;
    }

    super::set_uart(uart)?;

    Some(())
}
