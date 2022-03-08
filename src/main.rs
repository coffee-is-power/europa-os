#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod panic;
mod print;
mod idt;
mod memory;

use stivale_boot::v2::*;
use memory::paging::active_level_4_table;
use crate::memory::allocator::{PageFrameAllocator, set_global_allocator};

static mut STACK: [u8; 1048576] = [0; 1048576];
static TERMINAL_TAG: StivaleTerminalHeaderTag  = StivaleTerminalHeaderTag::new();
static FB_TAG: StivaleFramebufferHeaderTag  = StivaleFramebufferHeaderTag::new().next((&TERMINAL_TAG as *const StivaleTerminalHeaderTag).cast());
#[used]
#[link_section = ".stivale2hdr"]
static HDR : StivaleHeader = StivaleHeader::new().stack(unsafe{ &STACK[1048575] } as *const u8).tags((&FB_TAG as *const StivaleFramebufferHeaderTag).cast()).entry_point(_start);
extern "C" fn _start(boot_info: &StivaleStruct) -> ! {
    let terminal_tag = boot_info.terminal();
    let terminal = terminal_tag.unwrap();
    print::init(terminal);
    let mut allocator = unsafe {
        PageFrameAllocator::new(boot_info.memory_map().unwrap())
    };
    set_global_allocator(&mut allocator);
    idt::load_idt();
    panic!("Kernel reached the end of the main function.")
}