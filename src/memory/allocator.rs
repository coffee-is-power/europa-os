use core::ptr::null_mut;
use stivale_boot::v2::{StivaleMemoryMapEntry, StivaleMemoryMapEntryType};
use x86_64::PhysAddr;
use crate::memory::bitmap::BitMap;

pub struct PageFrameAllocator {
    free_memory: u64,
    reserved_memory: u64,
    used_memory: u64,
    bitmap: BitMap
}
#[derive(Debug)]
#[allow(dead_code)]
pub enum AllocErr {
    FreeMemoryNotFound
}
impl PageFrameAllocator {
    pub fn free_memory(&self) -> u64 { self.free_memory }
    pub fn reserved_memory(&self) -> u64 { self.reserved_memory }
    pub fn used_memory(&self) -> u64 { self.used_memory }
    pub unsafe fn new(memory_map: *const stivale_boot::v2::StivaleMemoryMapTag) -> Self {
        let mut largest_free_mem_segment_addr = 0u64;
        let mut largest_free_mem_segment_size = 0u64;
        let mut memory_size = 0u64;
        let memmap_pointer = (*memory_map).entry_array.as_ptr();
        for i in 0..(*memory_map).entries_len {
            let entry = &(*memmap_pointer.offset(i as isize) as StivaleMemoryMapEntry);
            if entry.entry_type == StivaleMemoryMapEntryType::Usable {
                if entry.length > largest_free_mem_segment_size {
                    largest_free_mem_segment_addr = entry.base;
                    largest_free_mem_segment_size = entry.length;
                }
                memory_size+=entry.length;
            }
        }
        let bitmap_size = (memory_size / 4096) + 1;
        let bitmap = BitMap {
            size: bitmap_size as usize,
            bitmap: largest_free_mem_segment_addr as *mut u8
        };
        for i in 0..bitmap_size {
            *bitmap.bitmap.offset(i as isize) = 0
        }
        let mut new_allocator = Self{
            free_memory: memory_size,
            reserved_memory: 0,
            used_memory: 0,
            bitmap
        };
        new_allocator.lock_pages(new_allocator.bitmap.bitmap as u64, ((new_allocator.bitmap.size / 4096) + 1) as u64);
        new_allocator
    }
    pub fn lock_page(&mut self, address: u64) {
        let index = (address / 4096) as usize;
        if self.bitmap.get(index).unwrap() {
            return;
        }
        self.bitmap.set(index, true).unwrap();
        self.free_memory -= 4096;
        self.used_memory += 4096;
    }
    pub fn lock_pages(&mut self, address: u64, count: u64){
        for i in 0..count {
            self.lock_page(address + (i*4069))
        }
    }
    pub fn unreserve_page(&mut self, address: u64){
        let index = (address / 4096) as usize;
        if !self.bitmap.get(index).unwrap() {
            return;
        }
        self.bitmap.set(index, false).unwrap();
        self.free_memory += 4096;
        self.reserved_memory -= 4096;
    }
    pub fn unreserve_pages(&mut self, address: u64, count: u64){
        for i in 0..count {
            self.unreserve_page(address + (i*4069))
        }
    }

    pub fn free_page(&mut self, address: u64){
        let index = (address / 4096) as usize;
        if !self.bitmap.get(index).unwrap() {
            return;
        }
        self.bitmap.set(index, false).unwrap();
        self.free_memory += 4096;
        self.used_memory -= 4096;
    }

    pub fn free_pages(&mut self, address: u64, count: u64){
        for i in 0..count {
            self.free_page(address + (i*4069))
        }
    }
    pub fn reserve_page(&mut self, address: u64){
        let index = (address / 4096) as usize;
        if self.bitmap.get(index).unwrap() {
            return;
        }
        self.bitmap.set(index, true).unwrap();
        self.free_memory -= 4096;
        self.reserved_memory += 4096;
    }

    pub fn reserve_pages(&mut self, address: u64, count: u64){
        for i in 0..count {
            self.reserve_page(address + (i*4069))
        }
    }
    pub fn request_page(&mut self) -> Result<PhysAddr, AllocErr>{
        for i in 0..(self.bitmap.size * 8) {
            if self.bitmap.get(i).unwrap() { continue; }
            self.lock_page((i * 4096) as u64);
            return Ok(PhysAddr::new((i * 4096) as u64))
        }
        Err(AllocErr::FreeMemoryNotFound)
    }
}
static mut GLOBAL_ALLOCATOR: *mut PageFrameAllocator = null_mut();
pub fn set_global_allocator(allocator: &mut PageFrameAllocator) {
    unsafe {
        GLOBAL_ALLOCATOR = allocator as *mut PageFrameAllocator
    }
}
pub fn get_global_allocator() -> &'static mut PageFrameAllocator{
    return unsafe {
        &mut (*GLOBAL_ALLOCATOR)
    }
}