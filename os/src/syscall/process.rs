//! Process management syscalls
use crate::{
    task::{
        current_task_id, exit_current_and_run_next, fill_task_info, suspend_current_and_run_next,
        TaskInfo,
    },
    timer::get_time_us,
};

const MIN_VALID_USER_PTR: usize = 4096;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// Get task information by task id.
pub fn sys_task_info(id: usize, ts: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let (id, ts) = if (ts as usize) < MIN_VALID_USER_PTR {
        (current_task_id(), id as *mut TaskInfo)
    } else {
        (id, ts)
    };
    if ts.is_null() {
        return -1;
    }
    if fill_task_info(id, ts) {
        0
    } else {
        -1
    }
}
