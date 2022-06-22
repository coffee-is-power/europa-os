use core::{cmp::Ordering, arch::asm};

use alloc::vec::Vec;
use x86_64::{structures::paging::PageTable, VirtAddr};
use super::{Thread, Process, ThreadState, Registers};
pub struct Scheduler {
    /// User space processes
    pub processes: Vec<Process>,
    /// This is threads that are used in the kernel itself
    /// 
    /// Such as for drivers and things that require asynchrounous logic
    pub kernel_tasks: Vec<Thread>,
}
/// fill a threads variable with all threads of the scheduler
macro_rules! fill_threads_variable {
    ($scheduler:ident, $variable_name:ident) => {
        for kernel_task in &mut $scheduler.kernel_tasks {
            $variable_name.push(kernel_task)
        }
        for process in &mut $scheduler.processes {
            for thread in &mut process.threads{
                $variable_name.push(thread)
            }
        }
    }
}
impl Scheduler {
    pub const fn new() -> Self {
        Self { processes: Vec::new(), kernel_tasks: Vec::new() }
    }
    /// Returns a reference of the next thread to execute
    /// Returns: **Some** if a thread is found or **None* if no threads are suitable for a context switch
    pub fn next_thread(&mut self) -> Option<&mut Thread> {
        let mut threads = Vec::<&mut Thread>::new();
        fill_threads_variable!(self, threads);
        if threads.len() < 2 {
            return None
        }
        sort_threads(&mut threads);
        let current_thread_index_opt = get_index_of_running_thread(&threads);
        if let Some(current_thread_index) = current_thread_index_opt {
            let next_thread = if current_thread_index >= threads.len()-1 { 0 } else { current_thread_index+1 };
            Some(threads.remove(next_thread))
        } else {
            Some(threads.remove(0))
        }
    }
    pub fn get_current_thread(&mut self) -> Option<&mut Thread>{
        let mut threads = Vec::<&mut Thread>::new();
        fill_threads_variable!(self, threads);
        let current_thread_index = get_index_of_running_thread(&threads)?;
        Some(threads.remove(current_thread_index))
    }
}
fn get_index_of_running_thread(threads: &Vec<&mut Thread>) -> Option<usize>{
    for (i, thread) in threads.iter().enumerate(){
        if let ThreadState::Running = thread.state {
            return Some(i);
        }
    }
    None
}
/// Sorts the threads by priority and by ID
fn sort_threads(threads: &mut Vec<&mut Thread>) {
    threads.sort_by(|a, b| {
        if a.is_kernel_thread || b.is_kernel_thread {
            a.is_kernel_thread.cmp(&b.is_kernel_thread)
        } else {
            let a_priority: u32 = a.priority.into();
            let b_priority: u32 = b.priority.into();
            // The most important threads must be first in the list so we invert a and b
            let priority_comparison = b_priority.cmp(&a_priority);
            if priority_comparison == Ordering::Equal {
                a.id.cmp(&b.id)
            } else {
                priority_comparison
            }
        }
    })
}
static mut GLOBAL_SCHEDULER: Scheduler = Scheduler::new();
pub fn get_global_scheduler() -> &'static mut Scheduler{
    unsafe {
        &mut GLOBAL_SCHEDULER
    }
}

#[no_mangle]
unsafe extern "C" fn _tick(stack_frame_ex: &mut Registers) {
    //crate::println!("{:#?}", stack_frame_ex);

    // crate::print!("\x1b[H\x1b[0m\x1b[2J");
    if let Some(next_thread) = get_global_scheduler().next_thread() {
        if let Some(current_thread) = get_global_scheduler().get_current_thread(){
            current_thread.state = ThreadState::Pending{
                registers: stack_frame_ex.clone()
            };
        }
        match next_thread.state {
            ThreadState::Created{start_code,new_stack} => {
                let new_stack_addr = new_stack.as_u64();
                stack_frame_ex.stack_frame.instruction_pointer = VirtAddr::new(start_code as u64);
                stack_frame_ex.stack_frame.stack_pointer = VirtAddr::new(new_stack_addr);
                stack_frame_ex.rbp = new_stack_addr;
                next_thread.state = ThreadState::Running;
            },
            ThreadState::Pending{registers} => {
                stack_frame_ex.stack_frame.instruction_pointer = registers.stack_frame.instruction_pointer;
                stack_frame_ex.stack_frame.stack_pointer = registers.stack_frame.stack_pointer;
                stack_frame_ex.r15 = registers.r15;
                stack_frame_ex.r14 = registers.r14;
                stack_frame_ex.r13 = registers.r13;
                stack_frame_ex.r12 = registers.r12;
                stack_frame_ex.r11 = registers.r11;
                stack_frame_ex.r10 = registers.r10;
                stack_frame_ex.r9 = registers.r9;
                stack_frame_ex.r8 = registers.r8;
                stack_frame_ex.rax = registers.rax;
                stack_frame_ex.rbx = registers.rbx;
                stack_frame_ex.rcx = registers.rcx;
                stack_frame_ex.rdx = registers.rdx;
                stack_frame_ex.rsi = registers.rsi;
                stack_frame_ex.rdi = registers.rdi;
                stack_frame_ex.rbp = registers.rbp;
            },
            _ => {}
        }
    } else {
        //crate::println!("No thread to switch")
    }
}