#![no_std]
#![no_main]
mod panic;
use stivale_boot::v2::*;
static STACK: [u8; 4096] = [0; 4096];
static TERMINAL_TAG: StivaleTerminalHeaderTag  = StivaleTerminalHeaderTag::new();
static FB_TAG: StivaleFramebufferHeaderTag  = StivaleFramebufferHeaderTag::new().next((&TERMINAL_TAG as *const StivaleTerminalHeaderTag).cast());
#[used]
#[link_section = ".stivale2hdr"]
static HDR : StivaleHeader = StivaleHeader::new().stack(&STACK[4095] as *const u8).tags((&FB_TAG as *const StivaleFramebufferHeaderTag).cast()).flags((1 << 1) | (1 << 2) | (1 << 3) | (1 << 4));

#[no_mangle]
extern "C" fn _start(_header_addr: usize) -> ! {
    let boot_info = unsafe {
        (_header_addr as *const StivaleStruct).as_ref().unwrap()
    };
    let terminal_tag = boot_info.terminal();
    
    let terminal = terminal_tag.unwrap();
    terminal.term_write()("Hello World!");
    
    loop {}
}