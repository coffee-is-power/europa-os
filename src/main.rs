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
use alloc::alloc::alloc_zeroed;
use init::init;
extern crate alloc;
use memory::paging::get_current_page_table;
use stivale_boot::v2::*;
use x86_64::{VirtAddr, structures::idt::InterruptStackFrameValue};
use scheduling::*;
fn thread_start() {
    println!("Hello From thread!");
    for i in 0..5000 {
        println!("Hello From thread! {}x", i);
    }
}
fn thread2_start() {
    println!("Hello From thread2!");
    for i in 0..5000 {
        println!("Hello From thread2! {}x", i);
    }
}
static STACK1: [u128; 61440] = [0; 61440];
static STACK2: [u128; 61440] = [0; 61440];

extern "C" fn _start(boot_info: &'static StivaleStruct) -> ! {
    unsafe {core::arch::asm!("cli");}
    init(boot_info);
    let scheduler = get_global_scheduler();
    scheduler.kernel_tasks.push(Thread {
        id: 0,
        is_kernel_thread: true,
        priority: scheduling::ThreadPriority::High,
        state: ThreadState::Created{
            new_stack: VirtAddr::from_ptr(&STACK1[61439]),
            start_code: thread_start,
        }
    });
    scheduler.kernel_tasks.push(Thread {
        id: 1,
        is_kernel_thread: true,
        priority: scheduling::ThreadPriority::High,
        state: ThreadState::Created{
            new_stack: VirtAddr::from_ptr(&STACK2[61439]),
            start_code: thread2_start,
        }
    });

    // Enables Maskable interrupts from the PIC chip
    unsafe {core::arch::asm!("sti");}
    //let rsdp = boot_info.rsdp().expect("RSDP Not found");
    //println!("Regions: {:#?}", pci::get_pci_config_regions(rsdp.rsdp).unwrap().get_pci_functions());
    //let initrd = get_initrd(boot_info);
    //for file in initrd.entries() {
    //    println!("{}", file.filename())
    //}
    panic!("Kernel reached the end of the main function.")
}
