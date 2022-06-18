use core::arch::asm;

use stivale_boot::v2::StivaleStruct;
use crate::{*};

pub fn init(boot_info: &StivaleStruct){
    print::init();
    memory::init_heap(boot_info);
    idt::load_idt();
    
    pit::set_default_pit_mode();
    pit::set_frq(8000);
    // Sets the PIC Chip Offset to a higher value so it doesn't overlap with the exception interrupts
    pic::fix_pic();
    // Enable PIT interrupt
    pic::enable_interrupt(0x20);
    // Enables Maskable interrupts from the PIC chip
    unsafe {asm!("sti");}

    println!("Kernel initialized successfully!")
}