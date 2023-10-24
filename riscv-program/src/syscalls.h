#include <stdint.h>

#define SYSCALL_READ_INPUT 0
#define SYSCALL_EXIT 1
#define SYSCALL_PUTS 2

// to communicate with the kernel/emulator the first argument is the syscall
// identifier
extern int interrupt(uint8_t id, ...);

int syscall_read(uint8_t* buffer, uint32_t buffer_size);
void syscall_exit(int exit_code);
void syscall_puts(char* str);