//! File and filesystem-related syscalls
use crate::fs::{link_at, open_file, unlink_at, OpenFlags, Stat};
use crate::mm::{copy_to_virt, translated_byte_buffer, translated_str, UserBuffer};
use crate::task::{current_task, current_user_token};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                let str = core::str::from_utf8(buffer).unwrap();
                print!("{}", str);
            }
            len as isize
        }
        _ => {
            todo!("Lab 6: implement sys_write for all file descriptors")
        }
    }
}

/// Read from a file descriptor
pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    todo!("Lab 6: implement sys_read")
}

/// Open a file and return a file descriptor
pub fn sys_open(path: *const u8, flags: u32) -> isize {
    todo!("Lab 6: implement sys_open")
}

/// Close a file descriptor
pub fn sys_close(fd: usize) -> isize {
    todo!("Lab 6: implement sys_close")
}

/// Get fstat
pub fn sys_fstat(fd: usize, st: *mut Stat) -> isize {
    debug!(
        "kernel:pid[{}] sys_fstat(fd: {}, st: 0x{:x?})",
        current_task().unwrap().pid.0, fd, st
    );
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        let stat = file.state().unwrap();
        copy_to_virt(&stat, st);
        return 0
    }
    -1
}

/// Syscall linkat.
pub fn sys_linkat(old_name: *const u8, new_name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_linkat(old_name: 0x{:x?}, new_name: 0x{:x?})",
        current_task().unwrap().pid.0, old_name, new_name
    );
    let token = current_user_token();
    let old_path = translated_str(token, old_name);
    let new_path = translated_str(token, new_name);
    link_at(&old_path, &new_path)
}

/// Syscall unlinkat.
pub fn sys_unlinkat(name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_unlinkat(name: 0x{:x?})",
        current_task().unwrap().pid.0, name
    );
    let token = current_user_token();
    let path = translated_str(token, name);
    unlink_at(&path)
}
