#ifndef _LIB_H
#define _LIB_H

#include <stddef.h>

typedef long isize;

#define SYSCALL_WRITE 64
#define SYSCALL_EXIT 93
#define SYSCALL_YIELD 124
#define SYSCALL_GET_TIME 169
#define SYSCALL_TRACE 410

#define TRACE_GET_SYSCALL_COUNT 0x00
#define TRACE_GET_TASK_INFO 0x01
#define TRACE_GET_TOTAL_SYSCALLS 0x02

#define TASK_STATUS_UNINIT 0
#define TASK_STATUS_READY 1
#define TASK_STATUS_RUNNING 2
#define TASK_STATUS_EXITED 3

typedef struct {
    size_t status;
    size_t syscall_times;
} TaskInfo;

typedef struct {
    size_t sec;
    size_t usec;
} TimeVal;

isize syscall(size_t id, size_t arg0, size_t arg1, size_t arg2);
isize write(int fd, const void *buf, size_t count);
void exit(int code);
isize yield(void);
isize get_time(TimeVal *ts);
isize trace(size_t request, size_t id, size_t data);

// Include stdio functions for convenience
#include "stdio.h"

#endif
