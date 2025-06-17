use crate::{
    paging::{Access, GB, MB, MapConfig, PageTableRef, PhysAddr, TableGeneric},
    *,
};
use num_align::{NumAlign, NumAssertAlign};

pub struct LineAllocator {
    start: *mut u8,
}

impl LineAllocator {
    pub fn new() -> Self {
        Self {
            start: rsv_end() as _,
        }
    }

    pub fn alloc(&mut self, layout: core::alloc::Layout) -> Option<usize> {
        let ptr = crate::alloc_memory(layout.size(), layout.align());
        Some(ptr as usize)
    }

    fn highest_address(&self) -> usize {
        rsv_end()
    }

    fn used(&self) -> usize {
        self.highest_address() - self.start as usize
    }
}

pub fn enable_mmu(args: &EarlyBootArgs) {
    setup_table_regs();

    let addr = new_boot_table(args);
    set_table(addr);
    setup_sctlr();
}

impl Access for LineAllocator {
    #[inline(always)]
    unsafe fn alloc(&mut self, layout: core::alloc::Layout) -> Option<PhysAddr> {
        LineAllocator::alloc(self, layout).map(|p| p.into())
    }

    #[inline(always)]
    unsafe fn dealloc(&mut self, _ptr: PhysAddr, _layout: core::alloc::Layout) {}

    #[inline(always)]
    fn phys_to_mut(&self, phys: PhysAddr) -> *mut u8 {
        phys.raw() as _
    }
}

/// `rsv_space` 在 `boot stack` 之后保留的空间到校
pub fn new_boot_table(args: &EarlyBootArgs) -> PhysAddr {
    let kcode_offset = args.kimage_addr_vma as usize - args.kimage_addr_lma as usize;

    let mut alloc = LineAllocator::new();

    let access = &mut alloc;

    printkv!("BootTable space", "[{:p} --)", access.start);

    let mut table = early_err!(PageTableRef::<'_, Table>::create_empty(access));
    unsafe {
        let align = if kcode_offset.is_aligned_to(GB) {
            GB
        } else {
            2 * MB
        };

        let code_start_phys = args.kimage_addr_lma.align_down(align) as usize;

        let code_start = args.kimage_addr_vma as usize;
        let code_end: usize = (rsv_end() + kcode_offset).align_up(align);

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
                pte: Pte::new(CacheKind::Device),
                allow_huge: true,
                flush: false,
            },
            access,
        ));
    }

    unsafe {
        let pg = table.paddr().raw();
        RUTERN.pg_start = pg;
        printkv!("Table", "{pg:#x}");
    }
    printkv!("Table size", "{:#x}", access.used());

    table.paddr()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheKind {
    Normal,
    Device,
}
