use x86_64::instructions::port::{PortWrite, PortRead};
macro_rules! outb {
    ($port:expr, $data:expr) => {
        u8::write_to_port($port, $data);
	u8::write_to_port(0x80, 0);
    }
}
macro_rules! inb {
    ($port:expr) => {
        u8::read_from_port($port)
    }
}
const PIC1_COMMAND_PORT: u16 = 0x20;
const PIC1_DATA_PORT: u16 = 0x21;
const PIC2_COMMAND_PORT: u16 = 0xA0;
const PIC2_DATA_PORT: u16 = 0xA1;

pub const PIC1_INTERRUPT_OFFSET: u8 = 32;
const PIC2_INTERRUPT_OFFSET: u8 = 40;

/// End-Of-Interrupt Command
const PIC_EOI: u8 = 0x20;

pub fn send_eoi(slave: bool) {
	unsafe {
		if slave {
			outb!(PIC2_COMMAND_PORT, PIC_EOI);
		}
		outb!(PIC1_COMMAND_PORT, PIC_EOI);
	}
}
/// Remaps the PIC Interrupts so it doesn't call our exception handlers
/// The pic by default is mapped from 0-7, which overlaps with the trap gates
pub fn fix_pic(){
    unsafe {
		// Reinitialize PIC1 and PIC2.
		outb!(PIC1_COMMAND_PORT, 0x11);
		outb!(PIC2_COMMAND_PORT, 0x11);

		// Map PIC1 to interrupt numbers >= 32 and PIC2 to interrupt numbers >= 40.
		outb!(PIC1_DATA_PORT, PIC1_INTERRUPT_OFFSET);
		outb!(PIC2_DATA_PORT, PIC2_INTERRUPT_OFFSET);

		// Configure PIC1 as master and PIC2 as slave.
		outb!(PIC1_DATA_PORT, 0x04);
		outb!(PIC2_DATA_PORT, 0x02);

		// Start them in 8086 mode.
		outb!(PIC1_DATA_PORT, 0x01);
		outb!(PIC2_DATA_PORT, 0x01);

		// Disable all interrupts on both PICs.
		outb!(PIC1_DATA_PORT, 0xFF);
		outb!(PIC2_DATA_PORT, 0xFF);
	}
}
fn edit_mask(int_no: u8, insert: bool) {
	let port = if int_no >= 40 {
		PIC2_DATA_PORT
	} else {
		PIC1_DATA_PORT
	};
	let offset = if int_no >= 40 { 40 } else { 32 };

	unsafe {
		let mask = inb!(port);

		if insert {
			outb!(port, mask | 1 << (int_no - offset));
		} else {
			outb!(port, mask & !(1 << (int_no - offset)));
		}
	}
}

pub fn disable_interrupt(int_no: u8) {
	edit_mask(int_no, true);
}

pub fn enable_interrupt(int_no: u8) {
	edit_mask(int_no, false);
}
