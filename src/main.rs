#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
mod panic;
mod print;
mod idt;
use stivale_boot::v2::*;
static STACK: [u8; 1048576] = [0; 1048576];
static TERMINAL_TAG: StivaleTerminalHeaderTag  = StivaleTerminalHeaderTag::new();
static FB_TAG: StivaleFramebufferHeaderTag  = StivaleFramebufferHeaderTag::new().next((&TERMINAL_TAG as *const StivaleTerminalHeaderTag).cast());
#[used]
#[link_section = ".stivale2hdr"]
static HDR : StivaleHeader = StivaleHeader::new().stack(&STACK[1048575] as *const u8).tags((&FB_TAG as *const StivaleFramebufferHeaderTag).cast()).entry_point(_start);

extern "C" fn _start(boot_info: &StivaleStruct) -> ! {
    let terminal_tag = boot_info.terminal();
    
    let terminal = terminal_tag.unwrap();
    print::init(terminal);
    idt::load_idt();

    panic!("Kernel reached the final of the main function.")
}