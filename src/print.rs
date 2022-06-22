use core::arch::asm;
use x86_64::instructions::port::{PortWrite, PortRead};
macro_rules! outb {
    ($port:expr, $data:expr) => {
        u8::write_to_port($port, $data)
    };
}
macro_rules! inb {
    ($port:expr) => {
        u8::read_from_port($port)
    };
}
const COM1: u16 = 0x3F8;

fn is_transmit_empty() -> bool
{
	unsafe{inb!(COM1 + 5) & 0x20 > 0}
}
pub fn init() {
    unsafe {
        outb!(COM1 + 1, 0x00);
        outb!(COM1 + 3, 0x80);
        outb!(COM1 + 0, 0x03);
        outb!(COM1 + 1, 0x00);
        outb!(COM1 + 3, 0x03);
        outb!(COM1 + 2, 0xC7);
        outb!(COM1 + 4, 0x0B);
        // Clear terminal
        crate::print!("\x1b[H\x1b[0m\x1b[2J");
    }
}

use core::fmt;

struct Writer {}
impl Writer {
    fn write_string(s: &str) {
        unsafe {
            for c in s.as_bytes() {
                // while !is_transmit_empty() {}
                outb!(COM1, c.clone());
            }
        }
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Self::write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = Writer {};
    writer.write_fmt(args).unwrap();
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (crate::print!("[\x1b[33mINFO\x1b[0m] {}\n", format_args!($($arg)*)));
}