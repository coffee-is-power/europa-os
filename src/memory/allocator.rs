use stivale_boot::{v2::{StivaleStruct, }};
use linked_list_allocator::LockedHeap;
pub struct MemSection {
    pub addr: usize,
    pub size: usize
}
#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
pub fn init_heap(boot_info: &StivaleStruct) {
    unsafe {
        let section = super::get_largest_memory_section(boot_info);
        ALLOCATOR.lock().init(section.addr as usize, section.size as usize);
    }
}
#[alloc_error_handler]
fn handle_allocation_error(layout: core::alloc::Layout) -> !{
    panic!("Couldn't alloc memory; Layout: {:#?}", layout);
}