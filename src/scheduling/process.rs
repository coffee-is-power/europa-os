use alloc::vec::Vec;

/// Represents a Process/Task
/// 
/// Processes can have 1 or multiple threads which can be created at runtime by the program itself
pub struct Process {
    /// ID of the process AKA pid
    pub id: usize,
    /// All the threads/tasks associated with it
    /// 
    /// It will have at least 1 main thread when running
    pub threads: Vec<super::Thread>
}