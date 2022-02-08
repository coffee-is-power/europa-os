#![no_std]
#![no_main]
mod panic;
mod print;
use stivale_boot::v2::*;
static STACK: [u8; 4096] = [0; 4096];
static TERMINAL_TAG: StivaleTerminalHeaderTag  = StivaleTerminalHeaderTag::new();
static FB_TAG: StivaleFramebufferHeaderTag  = StivaleFramebufferHeaderTag::new().next((&TERMINAL_TAG as *const StivaleTerminalHeaderTag).cast());
#[used]
#[link_section = ".stivale2hdr"]
static HDR : StivaleHeader = StivaleHeader::new().stack(&STACK[4095] as *const u8).tags((&FB_TAG as *const StivaleFramebufferHeaderTag).cast()).entry_point(_start);

extern "C" fn _start(boot_info: &StivaleStruct) -> ! {
    let terminal_tag = boot_info.terminal();
    
    let terminal = terminal_tag.unwrap();
    print::init(terminal);
    loop{}
}