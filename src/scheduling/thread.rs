use x86_64::VirtAddr;

#[derive(Copy, Clone, Debug)]
pub enum ThreadPriority {
    /// The thread will be executed at last
    Low = 0,
    /// Balanced priority, this is the default priority
    Mid = 1,
    /// Threads with this priority will be executed ASAP
    High = 2
}
impl Into<u32> for ThreadPriority {
    fn into(self) -> u32 {
        self as u32
    }
}
#[derive(Debug)]
pub enum ThreadState {
    /// The thread's code is on memory and it's ready for execution
    Created {
        start_code: fn (),
        new_stack: VirtAddr,
    },
    /// The thread is currently running
    Running,
    /// Waiting for the kernel to make a context switch for this thread
    /// Contains all the state of the thread that were stored on the last context switch
    Pending {registers: super::Registers},
    /// The thread is in a state that it can't do a context switch without falling apart
    Blocked,
    /// The thread's code ended and it can be freed up
    Ended
}
#[derive(Debug)]
pub struct Thread {
    /// Id of the task
    pub id: usize,
    /// The task's priority over other tasks
    pub priority: ThreadPriority,
    /// The state of the process
    pub state: ThreadState,
    pub is_kernel_thread: bool
}