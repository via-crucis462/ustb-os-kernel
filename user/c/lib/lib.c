#include "lib.h"

extern char start_bss[];
extern char end_bss[];

void clear_bss() {
    char *p;
    for (p = start_bss; p < end_bss; p++) {
        *p = 0;
    }
}

extern int main();

void __attribute__((section(".text.entry"))) _start() {
    clear_bss();
    exit(main());
}

isize syscall(size_t id, size_t arg0, size_t arg1, size_t arg2) {
    register size_t a0 asm("x10") = arg0;
    register size_t a1 asm("x11") = arg1;
    register size_t a2 asm("x12") = arg2;
    register size_t a7 asm("x17") = id;
    
    asm volatile (
        "ecall"
        : "+r" (a0)
        : "r" (a1), "r" (a2), "r" (a7)
        : "memory"
    );
    return a0;
}

isize write(int fd, const void *buf, size_t count) {
    return syscall(SYSCALL_WRITE, (size_t)fd, (size_t)buf, count);
}

void exit(int code) {
    syscall(SYSCALL_EXIT, (size_t)code, 0, 0);
    while(1);
}

isize yield(void) {
    return syscall(SYSCALL_YIELD, 0, 0, 0);
}

isize get_time(TimeVal *ts) {
    return syscall(SYSCALL_GET_TIME, (size_t)ts, 0, 0);
}

isize trace(size_t request, size_t id, size_t data) {
    return syscall(SYSCALL_TRACE, request, id, data);
}
