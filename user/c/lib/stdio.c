#include "stdio.h"

void print_str(const char *str) {
    size_t len = 0;
    const char *p = str;
    while (*p++) 
        len++;
    write(1, str, len);
}

void print_char(char c) {
    write(1, &c, 1);
}

void print_num(isize num) {
    if (num < 0) {
        write(1, "-", 1);
        num = -num;
    }
    if (num == 0) {
        write(1, "0", 1);
        return;
    }
    
    char buf[32];
    int i = 0;
    while (num > 0) {
        buf[i++] = '0' + (num % 10);
        num /= 10;
    }
    
    while (i > 0) {
        write(1, &buf[--i], 1);
    }
}

void print_hex(size_t num) {
    write(1, "0x", 2);
    
    if (num == 0) {
        write(1, "0", 1);
        return;
    }
    
    char buf[16];
    int i = 0;
    while (num > 0) {
        int digit = num % 16;
        buf[i++] = digit < 10 ? ('0' + digit) : ('a' + digit - 10);
        num /= 16;
    }
    
    while (i > 0) {
        write(1, &buf[--i], 1);
    }
}

void print_ptr(void *ptr) {
    print_hex((size_t)ptr);
}