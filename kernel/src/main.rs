#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

#[macro_use]
mod utils;

mod boot;
mod config;
mod mm;
mod fs;
mod drivers;
mod sync;
mod syscall;
mod trap;
mod task;
mod timer;
mod app_loader;

use log::debug;
use crate::utils::console;


/// clear BSS segment
pub fn clear_bss() {
    unsafe extern "C" {
        unsafe fn sbss();
        unsafe fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn main() -> ! {
    debug!("call main");
    clear_bss();
    println!("Hello World!");
    //logging::init();
    mm::init();
    println!("after initproc!");
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    fs::list_apps();
    task::add_initproc();
    task::run_tasks();
    panic!("Unreachable in rust_main!");    
}
