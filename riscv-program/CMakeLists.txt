cmake_minimum_required(VERSION 3.17)
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_VERSION 1)

set(ELF_NAME test.elf)
set(BINARY_NAME test.bin)
set(LINKER_SCRIPT ./script.ld)
set(RISCV_LIBC /home/u22/riscv-gnu-toolchain/build/riscv32-unknown-elf/lib/libc.a)

set(CMAKE_C_COMPILER_WORKS 1)
set(CMAKE_CXX_COMPILER_WORKS 1)
set(CMAKE_C_COMPILER riscv32-unknown-elf-gcc)
set(CMAKE_CXX_COMPILER riscv32-unknown-elf-g++)
set(CMAKE_ASM_COMPILER riscv32-unknown-elf-gcc)
set(CMAKE_LINKER riscv32-unknown-elf-ld)
set(CMAKE_AR riscv32-unknown-elf-gcc-ar)
set(CMAKE_OBJCOPY riscv32-unknown-elf-objcopy)
set(CMAKE_OBJDUMP riscv32-unknown-elf-objdump)
set(SIZE riscv32-unknown-elf-size)
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)

set(CMAKE_C_LINK_EXECUTABLE  "<CMAKE_LINKER> <FLAGS> <CMAKE_C_LINK_FLAGS> <LINK_FLAGS> <OBJECTS> -o <TARGET> <LINK_LIBRARIES>")

project(RISCV C ASM)

add_compile_options(
    -march=rv32i
    -mabi=ilp32
    -ggdb2
)

add_link_options(
    -T ${CMAKE_SOURCE_DIR}/${LINKER_SCRIPT}
    -Map=dbg.map
)

add_executable(${ELF_NAME}
    src/test.c
    src/boot.c
    src/syscalls.c
    src/interrupt.s
)

# include libc
target_link_libraries(${ELF_NAME} PUBLIC ${RISCV_LIBC})

target_include_directories(${ELF_NAME} PUBLIC ./src)

add_custom_command(TARGET ${ELF_NAME}
    POST_BUILD
    COMMAND
    ${CMAKE_OBJCOPY}
    -O binary
    ${CMAKE_BINARY_DIR}/${ELF_NAME}
    ${CMAKE_BINARY_DIR}/${BINARY_NAME}
)