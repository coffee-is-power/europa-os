use core::arch::asm;

use stivale_boot::v2::StivaleStruct;
use crate::{*};

pub fn init(boot_info: &StivaleStruct){
    print::init();
    memory::init_heap(boot_info);
    idt::load_idt();
    
    // Sets the PIC Chip Offset to a higher value so it doesn't overlap with the exception interrupts
    pic::fix_pic();
    pit::set_frq(5000);
    // Enable PIT interrupt
    pic::enable_interrupt(0x20);
    

    println!("Kernel initialized successfully!")
}