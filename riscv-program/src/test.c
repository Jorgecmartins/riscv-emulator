#include "syscalls.h"

int main() {
  char data;

  syscall_puts("hello");

  syscall_read(&data, sizeof(data));

  if (data == 1) {
    syscall_puts("Received value 1");
  } else {
    syscall_puts("Received another value");
  }

  return 4 + 1;
}