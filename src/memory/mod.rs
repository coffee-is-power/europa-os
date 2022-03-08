use x86_64::structures::paging::{PageTable, PageTableFlags};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::idt::ExceptionVector::Page;
use crate::active_level_4_table;
pub mod allocator;
pub mod paging;
mod bitmap;
use crate::memory::allocator::get_global_allocator;


fn alloc_page() -> PhysAddr{
    get_global_allocator().request_page().expect("Couldn't allocate a page")
}
pub unsafe fn map_mem(virtual_memory: VirtAddr, physical_memory: PhysAddr) {
    let l4_index = virtual_memory.p4_index();
    let l3_index = virtual_memory.p3_index();
    let l2_index = virtual_memory.p2_index();
    let l1_index = virtual_memory.p1_index();
    let l4 = active_level_4_table();
    let l3 = &mut l4[l4_index];
    if !l3.flags().contains(PageTableFlags::PRESENT) {
        l3.set_addr(alloc_page(), PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
    }
    let l2 = &mut (*(l3.addr().as_u64() as *mut PageTable))[l3_index];
    if !l2.flags().contains(PageTableFlags::PRESENT) {
        l2.set_addr(alloc_page(), PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
    }

    let l1 = &mut (*(l2.addr().as_u64() as *mut PageTable))[l2_index];
    if !l1.flags().contains(PageTableFlags::PRESENT) {
        l1.set_addr(alloc_page(), PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
    }

    let l0 = &mut (*(l1.addr().as_u64() as *mut PageTable))[l1_index];
    l0.set_addr(physical_memory, PageTableFlags::PRESENT | PageTableFlags::WRITABLE);

}
