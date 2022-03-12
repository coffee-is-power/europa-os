#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::string::{ToString};
use acpi::{AcpiTables};
use linked_list_allocator::LockedHeap;
use stivale_boot::v2::*;
use tar_no_std::TarArchiveRef;

use memory::paging::active_level_4_table;
use crate::pci::{AcpiHandlerImpl, PciConfigRegions};


mod panic;
mod print;
mod idt;
mod memory;
mod pci;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut STACK: [u8; 1048576] = [0; 1048576];
static TERMINAL_TAG: StivaleTerminalHeaderTag  = StivaleTerminalHeaderTag::new();
static FB_TAG: StivaleFramebufferHeaderTag  = StivaleFramebufferHeaderTag::new().next((&TERMINAL_TAG as *const StivaleTerminalHeaderTag).cast());
#[used]
#[link_section = ".stivale2hdr"]
static HDR : StivaleHeader = StivaleHeader::new().stack(unsafe{ &STACK[1048575] } as *const u8).tags((&FB_TAG as *const StivaleFramebufferHeaderTag).cast()).entry_point(_start);
fn init_heap(boot_info: &StivaleStruct){
    unsafe {
        let memory_map = boot_info.memory_map().unwrap();
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
        ALLOCATOR.lock().init(largest_free_mem_segment_addr as usize, largest_free_mem_segment_size as usize);
    }
}
#[alloc_error_handler]
fn handle_allocation_error(layout: core::alloc::Layout) -> !{
    panic!("Couldn't alloc memory; Layout: {:#?}", layout);
}
extern "C" fn _start(boot_info: &StivaleStruct) -> ! {
    let terminal_tag = boot_info.terminal();
    let terminal = terminal_tag.unwrap();
    print::init(terminal);
    init_heap(boot_info);
    idt::load_idt();
    let rsdp = boot_info.rsdp().unwrap().rsdp;
    unsafe {
        let tables = AcpiTables::from_rsdp(AcpiHandlerImpl, rsdp as usize).expect("Couldn't load tables");

        let pci_config_regions = PciConfigRegions::new(&tables).unwrap();
        println!("Regions: {:#?}", pci_config_regions.get_pci_functions());


        let module = &*(boot_info.modules().unwrap().modules_array.as_ptr());
        let initramfs_archive = TarArchiveRef::new(core::slice::from_raw_parts(module.start as *const u8, (module.end - module.start) as usize));

    }
    panic!("Kernel reached the end of the main function.")
}