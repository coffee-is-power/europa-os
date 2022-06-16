// PIT means Programmable Interval Timer
// It's used to create timers and clocks
// It'll call an interrupt every X time
// It's ´Programmable´ because we can change the interval to whatever we want

pub use x86_64::instructions::port::PortWrite;

use crate::println;
unsafe fn io_wait() {
    u8::write_to_port(0x80, 0);
}
/// This is the amount of cycles/interrupts that the PIT does per second
pub const BASE_FREQUENCY: usize = 1193182;
static mut CYCLES_INTERVAL: u16 = 0;
/// Set the cycles interval
/// Every `x` PIT cycles the PIT will emit an interrupt to the CPU
pub fn set_cycles_interval(mut new_cycles_interval: u16) {
    if new_cycles_interval < 100 {
        new_cycles_interval = 100
    }
    unsafe {
        CYCLES_INTERVAL = new_cycles_interval;
        u8::write_to_port(0x40, (new_cycles_interval & 0x00ff) as u8);
        u8::write_to_port(0x40, ((new_cycles_interval & 0xff00) >> 8) as u8);
        io_wait();
    }
}
/// Set the frequency
pub fn set_frq(frq: usize) {
    set_cycles_interval((BASE_FREQUENCY/frq) as u16)
}
/// This function is called every PIT tick/cycle
pub fn tick() {
    println!("Tick!");
    
}
/// Returns the current frequency
fn get_frq() -> usize{
    unsafe{CYCLES_INTERVAL as usize * BASE_FREQUENCY}
}