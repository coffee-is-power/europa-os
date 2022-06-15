use core::panic::PanicInfo;

use crate::println;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Kernel Panic: {}", _info);
    loop {}
}
