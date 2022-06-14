use stivale_boot::v2::StivaleMemoryMapEntryType;

use stivale_boot::v2::StivaleMemoryMapEntry;

use super::MemSection;

use stivale_boot::v2::StivaleStruct;

pub(super) unsafe fn get_largest_memory_section(boot_info: &StivaleStruct) -> MemSection{
    let memory_map = boot_info.memory_map().unwrap();
    let mut largest_free_mem_segment_addr = 0usize;
    let mut largest_free_mem_segment_size = 0usize;
    let memmap_pointer = (*memory_map).entry_array.as_ptr();
    for i in 0..(*memory_map).entries_len {
        let entry = &(*memmap_pointer.offset(i as isize) as StivaleMemoryMapEntry);
        if entry.entry_type == StivaleMemoryMapEntryType::Usable {
            if entry.length as usize > largest_free_mem_segment_size {
                largest_free_mem_segment_addr = entry.base as usize;
                largest_free_mem_segment_size = entry.length as usize;
            }
            // This maybe useful later so i'm keeping it
            // memory_size+=entry.length;
        }
    }
    MemSection { addr: largest_free_mem_segment_addr, size: largest_free_mem_segment_size }
}