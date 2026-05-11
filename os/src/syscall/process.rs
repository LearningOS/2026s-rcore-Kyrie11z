//! Process management syscalls
use crate::{
    config::CLOCK_FREQ,
    task::{current_syscall_times, exit_current_and_run_next, suspend_current_and_run_next},
    timer::{get_time, get_time_us},
};

const CLOCK_MONOTONIC: usize = 1;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// Seconds.
    pub sec: usize,
    /// Microseconds.
    pub usec: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct TimeSpec {
    /// Seconds.
    pub tv_sec: usize,
    /// Nanoseconds.
    pub tv_nsec: usize,
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

/// get time with second and nanosecond
pub fn sys_clock_get_time(clock_id: usize, ts: *mut TimeSpec) -> isize {
    trace!("kernel: sys_clock_get_time");
    if clock_id != CLOCK_MONOTONIC {
        return -1;
    }
    let ns = get_time() * 1_000_000_000 / CLOCK_FREQ;
    unsafe {
        *ts = TimeSpec {
            tv_sec: ns / 1_000_000_000,
            tv_nsec: ns % 1_000_000_000,
        };
    }
    0
}

/// Trace current task memory and syscall counters.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => unsafe { *(id as *const u8) as isize },
        1 => {
            unsafe {
                *(id as *mut u8) = data as u8;
            }
            0
        }
        2 => current_syscall_times(id).map_or(-1, |times| times as isize),
        _ => -1,
    }
}
