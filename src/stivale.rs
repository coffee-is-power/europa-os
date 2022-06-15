
use stivale_boot::v2::*;
macro_rules! tag {
    ($tag_header:ident) => {
        convert_to_pointer(&($tag_header)).cast()
    }
}
const fn convert_to_pointer<T>(a: &T) -> *const T{
    a as *const T
}

#[used]
#[link_section = ".stack"]
static STACK: [u128; 61440] = [0; 61440];
static TERMINAL_TAG: StivaleTerminalHeaderTag  = StivaleTerminalHeaderTag::new();
static FB_TAG: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new().next(tag!(TERMINAL_TAG));

#[used]
#[link_section = ".stivale2hdr"]
pub static HDR : StivaleHeader = StivaleHeader::new()
    .stack((&STACK[61439] as *const u128).cast())
    .tags(tag!(FB_TAG))
    .entry_point(crate::_start);

