use crate::{
    paging::{GB, MB, MapConfig, PageTableRef, PhysAddr, TableGeneric},
    ram::Ram,
    *,
};
use kdef_pgtable::KLINER_OFFSET;
use num_align::{NumAlign, NumAssertAlign};

pub fn enable_mmu(args: &EarlyBootArgs) {
    setup_table_regs();

    let addr = new_boot_table(args);
    set_table(addr);
    setup_sctlr();
}

/// `rsv_space` 在 `boot stack` 之后保留的空间到校
pub fn new_boot_table(args: &EarlyBootArgs) -> PhysAddr {
    let kcode_offset = args.kimage_addr_vma as usize - args.kimage_addr_lma as usize;

    let mut alloc = Ram {};

    let access = &mut alloc;

    let table_start = access.current();

    printkv!("BootTable space", "[{:p} --)", table_start);

    let mut table = early_err!(PageTableRef::<'_, Table>::create_empty(access));
    unsafe {
        let align = if kcode_offset.is_aligned_to(GB) {
            GB
        } else {
            2 * MB
        };

        let code_start_phys = args.kimage_addr_lma.align_down(align) as usize;

        let code_start = args.kimage_addr_vma as usize;
        let mut code_end: usize = (table_start as usize + kcode_offset).align_up(align);
        code_end = code_end.align_up(512 * MB);

        let size = (code_end - code_start).max(align);

        printkv!(
            "code",
            "[{:#x}, {:#x}) -> [{:#x}, {:#x})",
            code_start,
            code_start + size,
            code_start_phys,
            code_start_phys + size
        );

        early_err!(table.map(
            MapConfig {
                vaddr: code_start.into(),
                paddr: code_start_phys.into(),
                size,
                pte: Pte::new(CacheKind::Normal),
                allow_huge: true,
                flush: false,
            },
            access,
        ));

        if debug::reg_base() > 0 {
            let paddr = debug::reg_base();
            let vaddr = paddr + KLINER_OFFSET;
            printkv!("debug", "{:#x}-> {:#x}", vaddr, paddr);
            early_err!(table.map(
                MapConfig {
                    vaddr: vaddr.into(),
                    paddr: paddr.into(),
                    size,
                    pte: Pte::new(CacheKind::Device),
                    allow_huge: true,
                    flush: false,
                },
                access,
            ));
        }

        let size = if table.entry_size() == table.max_block_size() {
            table.entry_size() * (Table::TABLE_LEN / 2)
        } else {
            table.max_block_size() * Table::TABLE_LEN
        };
        let start = 0x0usize;

        printkv!("eq", "[{:#x}, {:#x})", start, start + size);
        early_err!(table.map(
            MapConfig {
                vaddr: start.into(),
                paddr: start.into(),
                size,
                pte: Pte::new(CacheKind::Normal),
                allow_huge: true,
                flush: false,
            },
            access,
        ));
    }

    let pg = table.paddr().raw() as _;
    RUTERN.as_mut().pg_start = pg;
    printkv!("Table", "{pg:#p}");
    printkv!(
        "Table size",
        "{:#x}",
        access.current() as usize - table_start as usize
    );

    table.paddr()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheKind {
    Normal,
    Device,
}
