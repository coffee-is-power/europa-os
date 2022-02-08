
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(pagefault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.double_fault.set_handler_fn(doublefault_handler);
        idt
    };
}
pub fn load_idt(){
    IDT.load();
}
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn pagefault_handler(
    stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode)
{
    panic!("EXCEPTION: Page Fault\n{:#?}\nErr: {:#?}", stack_frame, error_code);
}

extern "x86-interrupt" fn doublefault_handler(
    stack_frame: InterruptStackFrame, error_code: u64) -> !
{
    panic!("EXCEPTION: Double Fault\n{:#?}\nErr: {}", stack_frame, error_code);
}


extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame, err: u64)
{
    panic!("EXCEPTION: GP fault\n{:#?}\n Err: {}", stack_frame, err);
}