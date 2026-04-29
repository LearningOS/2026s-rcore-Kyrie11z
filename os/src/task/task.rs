//! Types related to task management

use super::TaskContext;

/// Max syscall id tracked for each task.
pub const MAX_SYSCALL_NUM: usize = 500;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// Syscall statistics indexed by syscall id
    pub syscall_times: [usize; MAX_SYSCALL_NUM],
    /// Accumulated running time of the task
    pub total_time: usize,
    /// The timestamp when the task was last scheduled in
    pub last_start_time: usize,
}

/// Syscall information exported to user space.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize,
}

/// Task information exported to user space.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize,
}

/// The status of a task
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
