//! Process management syscalls
use alloc::sync::Arc;
use crate::{
    config::PAGE_SIZE, 
    app_loader::get_app_data_by_name, 
    mm::{copy_to_virt, translated_refmut, translated_str}, 
    task::{
        add_task, current_task, current_user_token, exit_current_and_run_next, mmap, munmap, 
        suspend_current_and_run_next,
    },
    timer::get_time_us,
};

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
    trace!("kernel:pid[{}] sys_exit", current_task().unwrap().pid.0);
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel:pid[{}] sys_yield", current_task().unwrap().pid.0);
    suspend_current_and_run_next();
    0
}

pub fn sys_getpid() -> isize {
    trace!("kernel: sys_getpid pid:{}", current_task().unwrap().pid.0);
    current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    trace!("kernel:pid[{}] sys_fork", current_task().unwrap().pid.0);
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    trace!("kernel:pid[{}] sys_exec", current_task().unwrap().pid.0);
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    trace!("kernel::pid[{}] sys_waitpid [{}]", current_task().unwrap().pid.0, pid);

    // TODO(Lab 6): 实现 waitpid 系统调用 —— 等待子进程结束并回收其资源
    //
    // 参数：
    //   pid:           要等待的子进程 pid
    //                  pid == -1 时等待任意一个子进程
    //                  pid >  0 时等待指定 pid 的子进程
    //   exit_code_ptr: 用户态指针，用于存储子进程的退出码
    //
    // 返回值：
    //   -1: 当前进程没有符合条件的子进程
    //   -2: 有符合条件的子进程但它尚未退出（仍在运行/就绪态）
    //   >0: 成功回收的子进程的 pid
    //
    // 实现步骤：
    // 1. 通过 current_task() 获取当前进程 TCB，再获取其 inner（独占访问）
    //
    // 2. 检查 inner.children 中是否存在 pid 匹配的子进程
    //    - pid == -1 匹配所有子进程
    //    - pid > 0  只匹配 getpid() == pid 的子进程
    //    - 提示: inner.children.iter().any(|p| pid == -1 || pid as usize == p.getpid())
    //    - 如果没有匹配的子进程，返回 -1
    //
    // 3. 在匹配的子进程中查找已变为 Zombie 状态的
    //    - 使用 inner.children.iter().enumerate().find(|(_, p)| { ... })
    //    - 通过 p.inner_exclusive_access().is_zombie() 判断是否为僵尸进程
    //    - 如果没有找到 zombie 子进程，返回 -2
    //
    // 4. 找到 zombie 子进程后：
    //    a. let child = inner.children.remove(idx);  // 从 children 列表中移除
    //    b. 获取 child.getpid() 作为返回值
    //    c. 获取 child.inner_exclusive_access().exit_code
    //    d. 用 translated_refmut(inner.memory_set.token(), exit_code_ptr)
    //       将退出码写入用户态指针所指向的位置
    //    e. 返回子进程的 pid

    todo!("Lab 6: implement sys_waitpid")
}


pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_get_time(ts: 0x{ts:x?})",
        current_task().unwrap().pid.0
    );
    let us = crate::timer::get_time_us();
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    copy_to_virt(&time_val, ts);
    0
}

pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_mmap)(start: 0x{start:x}, len: 0x{len:x}, port: 0x{port:x})",
        current_task().unwrap().pid.0
    );
    const PORT_MASK: usize = 0b111;
     
    let aligned_start = start % PAGE_SIZE == 0;
    let port_valid = (port & !PORT_MASK) == 0;
    let port_not_none = (port & PORT_MASK) != 0;
     
    trace!("each condition: aligned_start={}, port_valid={}, port_not_none={}", aligned_start, port_valid, port_not_none);
    if aligned_start && port_valid && port_not_none {
        return mmap(start, len, port)
    }
    -1
}


pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_munmap(start: 0x{start:x}, len: 0x{len:x})",
        current_task().unwrap().pid.0
    );
    let aligned_start = start % PAGE_SIZE == 0;
    if aligned_start {
        return munmap(start, len)
    }
    -1
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel:pid[{}] sys_sbrk", current_task().unwrap().pid.0);
    if let Some(old_brk) = current_task().unwrap().change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

pub fn sys_spawn(path: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_spawn(path: 0x{path:x?})",
        current_task().unwrap().pid.0
    );
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap().spwan(data);
        let new_pid = task.getpid();
        add_task(task);
        new_pid as isize
    } else {
        -1
    }
}


pub fn sys_set_priority(prio: isize) -> isize {
    debug!(
        "kernel:pid[{}] sys_set_priority(prio: {})",
        current_task().unwrap().pid.0,
        prio
    );
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if prio >= 2 {
        inner.priority = prio as usize;
        prio
    } else {
        -1
    }
}
