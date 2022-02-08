use core::panic::PanicInfo;

use crate::{print, println};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
