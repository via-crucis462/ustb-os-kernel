#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod config;
mod mm;
mod loader;
mod sync;
mod syscall;
mod trap;
mod utils;
mod batch;
mod boot;

use crate::utils::console;

/// clear BSS segment
pub fn clear_bss() {
    unsafe extern "C" {
        unsafe fn sbss();
        unsafe fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn main() {
    clear_bss();
    mm::init();
    println!("Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}
