#include "syscalls.h"
#include <string.h>

int syscall_read(uint8_t* buffer, uint32_t buffer_size) {
  return interrupt(SYSCALL_READ_INPUT, buffer, buffer_size);
}

void syscall_exit(int exit_code) {
  (void)interrupt(SYSCALL_EXIT, exit_code);
}

void syscall_puts(char* str) {
  (void)interrupt(SYSCALL_PUTS, str, strlen(str));
}