use std::env;
use std::fs::File;
use std::io::Read;

use vm::VM;

pub mod instruction_decoder;
pub mod instructions;
mod memory;
mod register;
mod syscalls;
mod utils;
mod vm;

fn main() {
    let binary = env::args().nth(1).unwrap();

    let mut f = File::open(binary).unwrap();
    let mut binary = Vec::new();
    f.read_to_end(&mut binary).unwrap();

    let mut vm = VM::new(binary);

    vm.init_execution();
    vm.start_execution();
}
