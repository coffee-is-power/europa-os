use core::cmp::Ordering;

use alloc::vec::Vec;
use x86_64::structures::paging::PageTable;
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
        for kernel_task in &$scheduler.kernel_tasks {
            $variable_name.push(kernel_task)
        }
        for process in &$scheduler.processes {
            for thread in &process.threads{
                $variable_name.push(thread)
            }
        }
    }
}
impl Scheduler {
    pub fn new() -> Self {
        Self { processes: Vec::new(), kernel_tasks: Vec::new() }
    }
    /// Returns a reference of the next thread to execute
    /// Returns: **Some** if a thread is found or **None* if no threads are suitable for a context switch
    pub fn next_thread(&self) -> Option<&Thread> {
        let mut threads = Vec::<&Thread>::new();
        fill_threads_variable!(self, threads);
        if threads.len() == 0 {
            return None
        }
        sort_threads(&mut threads);
        let current_thread_index = get_index_of_running_thread(&threads)?;
        let next_thread = if current_thread_index >= threads.len()-1 {
            0
        } else {
            current_thread_index+1
        };
        Some(threads[next_thread])
    }
    pub fn get_current_thread(&self) -> Option<&Thread>{
        let mut threads = Vec::<&Thread>::new();
        fill_threads_variable!(self, threads);
        let current_thread_index = get_index_of_running_thread(&threads)?;
        Some(threads[current_thread_index])
    }
}
fn get_index_of_running_thread(threads: &Vec<&Thread>) -> Option<usize>{
    for (i, thread) in threads.iter().enumerate(){
        if let ThreadState::Running = thread.state {
            return Some(i);
        }
    }
    None
}
/// Sorts the threads by priority and by ID
fn sort_threads(threads: &mut Vec<&Thread>) {
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
#[no_mangle]
unsafe extern "C" fn _tick(stack_frame_ex: Registers) {
    
}