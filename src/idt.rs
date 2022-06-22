
use core::arch::asm;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::{println, pic::send_eoi};
use lazy_static::lazy_static;
extern "x86-interrupt" fn invalid_opcode(isf: InterruptStackFrame){
    panic!("INVALID OPCODE: {:#?}", isf);
}
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(pagefault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.double_fault.set_handler_fn(doublefault_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode);
        idt[0x20].set_handler_fn(timer_int_handler);
        idt[0x20+7].set_handler_fn(master_strange_int_handler);
        idt[0x28+7].set_handler_fn(slave_strange_int_handler);
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
#[naked]
extern "x86-interrupt" fn timer_int_handler(_: InterruptStackFrame){
    unsafe {
        asm! {
            r#"
                push rbp
                push rax
                push rbx
                push rcx
                push rdx
                push r8
                push r9
                push r10
                push r11
                push r12
                push r13
                push r14
                push r15
                push 0
                push 0
                mov rdi, rsp
                call _tick
                pop rsi
                pop rdi
                pop r15
                pop r14
                pop r13
                pop r12
                pop r11
                pop r10
                pop r9
                pop r8
                pop rdx
                pop rcx
                pop rbx
                mov ax, 0x20
                out 0x20, ax
                pop rax
                pop rbp
                iretq
            "#,
            options(noreturn)
        }
    }
}
#[no_mangle]
extern "x86-interrupt" fn master_strange_int_handler(_: InterruptStackFrame){
    println!("strange int");
    send_eoi(false)
}
extern "x86-interrupt" fn slave_strange_int_handler(_: InterruptStackFrame){
    println!("strange int");
}