#include "boot.h"
#include "syscalls.h"
#include <stdio.h>

int keep_me;

extern int main(void);

void reset_handler()
{
    syscall_exit(main());
}

handler_t handlers __attribute__((section(".handler_table"))) = {
    .reset = reset_handler,
    .idk1 = NULL,
    .idk2 = NULL,
    .idk3 = NULL,
};


