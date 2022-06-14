
use stivale_boot::v2::*;

static mut STACK: [u8; 1048576] = [0; 1048576];
static TERMINAL_TAG: StivaleTerminalHeaderTag  = StivaleTerminalHeaderTag::new();
static FB_TAG: StivaleFramebufferHeaderTag  = StivaleFramebufferHeaderTag::new().next((&TERMINAL_TAG as *const StivaleTerminalHeaderTag).cast());

#[used]
#[link_section = ".stivale2hdr"]
pub static HDR : StivaleHeader = StivaleHeader::new().stack(unsafe{ &STACK[1048575] } as *const u8).tags((&FB_TAG as *const StivaleFramebufferHeaderTag).cast()).entry_point(crate::_start);
