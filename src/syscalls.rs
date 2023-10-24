pub enum Syscalls {
    ReadInput,
    Exit,
    Puts,
}

impl Syscalls {
    // todo this is not maitainable
    pub fn from_u32(syscall_id: u32) -> Syscalls {
        match syscall_id {
            0 => Syscalls::ReadInput,
            1 => Syscalls::Exit,
            2 => Syscalls::Puts,
            _ => panic!("Syscall id {} not supported", syscall_id),
        }
    }
}
