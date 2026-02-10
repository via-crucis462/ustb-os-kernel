#ifndef _STDIO_H
#define _STDIO_H

#include <stddef.h>

// Forward declarations (avoid circular dependency with lib.h)
typedef long isize;
isize write(int fd, const void *buf, size_t count);

void print_str(const char *str);
void print_char(char c);
void print_num(isize num);
void print_hex(size_t num);
void print_ptr(void *ptr);

#endif