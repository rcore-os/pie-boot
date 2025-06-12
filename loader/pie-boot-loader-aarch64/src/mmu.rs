use crate::{
    paging::{Access, GB, MB, MapConfig, PageTableRef, PhysAddr, TableGeneric},
    *,
};
use num_align::{NumAlign, NumAssertAlign};

pub struct LineAllocator {
    pub start: *mut u8,
    iter: *mut u8,
    pub end: *mut u8,
}

impl LineAllocator {
    pub fn new(start: *mut u8, size: usize) -> Self {
        Self {
            start,
            iter: start,
            end: unsafe { start.add(size) },
        }
    }

    pub fn alloc(&mut self, layout: core::alloc::Layout) -> Option<usize> {
        let start = self.iter.align_up(layout.align());
        if start as usize + layout.size() > self.end as usize {
            return None;
        }
        self.iter = unsafe { start.add(layout.size()) };

        Some(start as usize)
    }

    fn highest_address(&self) -> usize {
        self.iter as _
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
    let start = args.kcode_end.align_up(Table::PAGE_SIZE) as *mut u8;
    let size = GB;
    let kcode_offset = args.kimage_addr_vma as usize - args.kimage_addr_lma as usize;

    let mut alloc = LineAllocator::new(start, size);

    let access = &mut alloc;

    printkv!("BootTable space", "[{:p}, {:#x})", access.start, {
        access.start as usize + size
    });

    let mut table = early_err!(PageTableRef::<'_, Table>::create_empty(access));
    unsafe {
        let align = if kcode_offset.is_aligned_to(GB) {
            GB
        } else {
            2 * MB
        };

        let code_start_phys = args.kimage_addr_lma.align_down(align) as usize;

        let code_start = args.kimage_addr_vma as usize;
        let code_end: usize = (args.kcode_end as usize + kcode_offset).align_up(align);

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
                pte: Pte::new(),
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
                pte: Pte::new(),
                allow_huge: true,
                flush: false,
            },
            access,
        ));
    }

    printkv!("Table size", "{:#x}", access.used());
    unsafe { RUTERN.pg_end = access.highest_address() };
    printkv!("Table end", "{:#x}", access.highest_address());
    table.paddr()
}
