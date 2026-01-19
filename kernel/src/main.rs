#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod boot;
mod config;
mod mm;
mod loader;
mod sync;
mod syscall;
mod trap;
mod utils;
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
    trap::init();
    app_loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    //为什么
    panic!("Unreachable in rust_main!");    
}
