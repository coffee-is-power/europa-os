use lateinit::LateInit;
use stivale_boot::v2::*;

static GLOBAL_TERMINAL : LateInit<&StivaleTerminalTag> = LateInit::new();
pub fn init(terminal: *const StivaleTerminalTag) {
    unsafe {
        GLOBAL_TERMINAL.init(terminal.as_ref().unwrap());
    }
}

use core::fmt;
struct Writer {}
impl Writer {
    fn write_string(s: &str){
        GLOBAL_TERMINAL.term_write()(s);
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
    ($($arg:tt)*) => (crate::print!("{}\n", format_args!($($arg)*)));
}