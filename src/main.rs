#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
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
mod scheduling;
use core::alloc::Layout;
use x86_64::instructions::port::*;
use alloc::alloc::alloc_zeroed;
use init::init;
extern crate alloc;
use scheduling::{Scheduler, Thread};
use stivale_boot::v2::*;
use x86_64::VirtAddr;
use crate::initrd::get_initrd;
const COM1: u16 = 0x3F8;
fn thread_start() {
    println!("Hello From thread!");
}
extern "C" fn _start(boot_info: &'static StivaleStruct) -> ! {
    init(boot_info);
    let mut scheduler = Scheduler::new();
    scheduler.kernel_tasks.push(Thread {
        id: 0,
        is_kernel_thread: true,
        priority: scheduling::ThreadPriority::High,
        state: scheduling::ThreadState::Created{
            new_stack: unsafe {
                VirtAddr::from_ptr(alloc_zeroed(Layout::from_size_align(61440*16, 16).unwrap()))
            },
            start_code: thread_start,
        }
    });
    let thread = &scheduler.kernel_tasks[0];

    //let rsdp = boot_info.rsdp().expect("RSDP Not found");
    //println!("Regions: {:#?}", pci::get_pci_config_regions(rsdp.rsdp).unwrap().get_pci_functions());
    //let initrd = get_initrd(boot_info);
    //for file in initrd.entries() {
    //    println!("{}", file.filename())
    //}
    panic!("Kernel reached the end of the main function.")
}