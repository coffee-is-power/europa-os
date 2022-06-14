use stivale_boot::v2::StivaleStruct;
use crate::*;

pub fn init(boot_info: &StivaleStruct){
    let terminal = boot_info.terminal().unwrap();
    print::init(terminal);
    memory::init_heap(boot_info);
    idt::load_idt();
    println!("Kernel initialized successfully!")
}