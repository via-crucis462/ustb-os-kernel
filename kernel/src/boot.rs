use core::arch::global_asm;

const BOOT_STACK_SIZE: usize = 4096 * 16;

global_asm!(include_str!("link_app.S"));

global_asm!(
    r#"
    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top  
    call {rust_main} 

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space {boot_stack_size}
    .globl boot_stack_top
boot_stack_top:
    "#,
    boot_stack_size = const BOOT_STACK_SIZE,
    rust_main = sym super::main,
);