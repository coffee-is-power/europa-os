#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

mod panic;
mod print;
mod idt;
mod memory;
mod pci;
mod initrd;
mod stivale;
mod init;
mod pit;
mod pic;
use init::init;
extern crate alloc;
use stivale_boot::v2::*;
use crate::initrd::get_initrd;

extern "C" fn _start(boot_info: &'static StivaleStruct) -> ! {
    init(boot_info);
    //let rsdp = boot_info.rsdp().expect("RSDP Not found");
    //println!("Regions: {:#?}", pci::get_pci_config_regions(rsdp.rsdp).unwrap().get_pci_functions());
    //let initrd = get_initrd(boot_info);
    //for file in initrd.entries() {
    //    println!("{}", file.filename())
    //}
    panic!("Kernel reached the end of the main function.")
}
