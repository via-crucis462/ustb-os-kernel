//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, get_syscall_times, suspend_current_and_run_next, 
           get_current_task_info, get_total_syscall_count},
    timer::get_time_us,

};
use log::trace;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct TaskInfo {
    pub status: usize,
    pub syscall_times: usize,
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

/// simple syscall for tracing
pub fn sys_trace(request: usize, syscall_id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace request={}, syscall_id={:#x}", request, syscall_id);    
    
    const TRACE_GET_SYSCALL_COUNT: usize = 0x00;
    const TRACE_GET_TASK_INFO: usize = 0x01;
    const TRACE_GET_TOTAL_SYSCALLS: usize = 0x02;

    match request {
        TRACE_GET_SYSCALL_COUNT => {
            let syscall_id = syscall_id;
            get_syscall_times(syscall_id) as isize
        },
        TRACE_GET_TASK_INFO => {
            if data == 0 {
                return -1;
            }
            let info_ptr = data as *mut TaskInfo;
            match get_current_task_info() {
                Some((status, syscall_times)) => {
                    unsafe {
                        (*info_ptr).status = status;
                        (*info_ptr).syscall_times = syscall_times;
                    }
                    0
                },
                None => -1,
            }
        },
        TRACE_GET_TOTAL_SYSCALLS => {
            get_total_syscall_count() as isize
        },
        _ => {
            trace!("Invalid trace request: {}", request);
            -1
        },
    }
}
