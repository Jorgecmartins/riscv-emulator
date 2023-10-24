use std::{
    io::{self, Read},
    process::exit,
};

use crate::{
    instruction_decoder::decode,
    instructions::{BOpcode, IOpcode, InstructionFormat, JOpcode, ROpcode, SOpcode, UOpcode},
    memory::Memory,
    register::Register,
    syscalls::Syscalls,
    utils::sign_extend_number,
};

const MEMORY_SIZE: usize = 0x4000;

const FLASH_ADDRESS: usize = 0x40000;
const FLASH_INTERRUPT_TABLE_ADDRESS: usize = FLASH_ADDRESS;
const FLASH_INTERRUPT_TABLE_RESET_ADDRESS: usize = FLASH_INTERRUPT_TABLE_ADDRESS;

// 16 byte aligned
const STACK_ADDRESS: usize = 0xfffffff0;

pub struct VM {
    regs: Vec<Register>,
    pc: Register,
    flash: Memory,
    stack: Memory,
}

impl VM {
    pub fn new(flash_data: Vec<u8>) -> Self {
        assert!(flash_data.len() < MEMORY_SIZE);

        let mut registers = Vec::<Register>::new();
        for i in 0..32 {
            registers.push(Register::new(0, i, format!("x{}", i)));
        }

        let stack = vec![0; MEMORY_SIZE];

        Self {
            regs: registers,
            pc: Register::new(0, 90, "pc".to_string()),
            flash: Memory::new(FLASH_ADDRESS, flash_data),
            stack: Memory::new(STACK_ADDRESS - MEMORY_SIZE, stack),
        }
    }

    fn get_register_value(&self, register_index: u32) -> u32 {
        self.regs[register_index as usize].get_value()
    }

    fn set_register_value(&mut self, register_index: u32, value: u32) {
        self.regs[register_index as usize].set_value(value);
    }

    fn execute_instruction_r(&mut self, opcode: ROpcode) -> bool {
        match opcode {
            ROpcode::Add(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                let (result, _) = src1_value.overflowing_add(src2_value);
                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Sub(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                let (result, _) = src1_value.overflowing_sub(src2_value);
                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Slti(helper) => {
                let src1_value = self.get_register_value(helper.get_src1()) as i32;
                let src2_value = self.get_register_value(helper.get_src2()) as i32;

                let result = if src1_value < src2_value { 1 } else { 0 };

                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Sltu(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                let result = if src1_value < src2_value { 1 } else { 0 };

                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Xor(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                let result = src1_value ^ src2_value;

                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Or(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                let result = src1_value | src2_value;

                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Sll(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                let (result, _) = src1_value.overflowing_shl(src2_value);

                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Srl(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                let (result, _) = src1_value.overflowing_shr(src2_value);

                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Sra(helper) => {
                let src1_value = self.get_register_value(helper.get_src1()) as i32;
                let src2_value = self.get_register_value(helper.get_src2());

                // rust will perform arithmetic shift when dealing with signed numbers(i32)
                let (result, _) = src1_value.overflowing_shr(src2_value);

                self.set_register_value(helper.get_dest(), result as u32)
            }
            ROpcode::And(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                let result = src1_value & src2_value;

                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Slli(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let shift_value = helper.get_shamt();

                let (result, _) = src1_value.overflowing_shl(shift_value);

                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Srli(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let shift_value = helper.get_shamt();

                let (result, _) = src1_value.overflowing_shr(shift_value);

                self.set_register_value(helper.get_dest(), result)
            }
            ROpcode::Srai(helper) => {
                let src1_value = self.get_register_value(helper.get_src1()) as i32;
                let shift_value = helper.get_shamt();

                // rust will perform arithmetic shift when dealing with signed numbers(i32)
                let (result, _) = src1_value.overflowing_shr(shift_value);

                self.set_register_value(helper.get_dest(), result as u32)
            }
        }
        false
    }
    fn execute_instruction_i(&mut self, opcode: IOpcode) -> bool {
        let mut pc_changed = false;

        match opcode {
            IOpcode::Addi(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();

                let result = src_value.overflowing_add(imm_value).0;

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Andi(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();
                let result = src_value & imm_value;

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Xori(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();
                let result = src_value ^ imm_value;

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Ori(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();
                let result = src_value | imm_value;

                self.set_register_value(helper.get_dst(), result)
            }
            // todo Loads with a destination of x0 must still raise any exceptions and action any other side effects even though the load value is discarded.
            IOpcode::Slti(helper) => {
                let src_value = self.get_register_value(helper.get_src()) as i32;
                let imm_value = helper.get_imm() as i32;
                let result = if src_value < imm_value { 1 } else { 0 };

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Sltiu(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();
                let result = if src_value < imm_value { 1 } else { 0 };

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Lw(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();

                let address = src_value.overflowing_add(imm_value).0;
                let result = self.read_u32(address as usize);

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Lh(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();

                let address = src_value.overflowing_add(imm_value).0;
                let result = sign_extend_number(self.read_u16(address as usize) as u32, 16);

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Lhu(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();

                let address = src_value.overflowing_add(imm_value).0;
                let result = self.read_u16(address as usize) as u32;

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Lb(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();

                let address = src_value.overflowing_add(imm_value).0;
                let result = sign_extend_number(self.read_u8(address as usize) as u32, 8);

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Lbu(helper) => {
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();

                let address = src_value.overflowing_add(imm_value).0;
                let result = self.read_u8(address as usize) as u32;

                self.set_register_value(helper.get_dst(), result)
            }
            IOpcode::Jalr(helper) => {
                // todo The JAL and JALR instructions will generate a misaligned instruction fetch exception if the target address is not aligned to a four-byte boundary
                let src_value = self.get_register_value(helper.get_src());
                let imm_value = helper.get_imm();

                // save return address
                self.set_register_value(helper.get_dst(), self.pc.get_value() + 4);

                let target = src_value + imm_value;
                // set the least-significant bit to zero
                let target = target & !1;

                self.pc.set_value(target);

                pc_changed = true;
            }
        }

        pc_changed
    }
    fn execute_instruction_s(&mut self, opcode: SOpcode) -> bool {
        match opcode {
            SOpcode::Sw(helper) => {
                let src_value = self.get_register_value(helper.get_src());

                let base_value = self.get_register_value(helper.get_base());
                let imm_value = helper.get_offset();

                let address = base_value.overflowing_add(imm_value).0;
                self.write_u32(address as usize, src_value)
            }
            SOpcode::Sh(helper) => {
                let src_value = self.get_register_value(helper.get_src()) & 0xffff;

                let base_value = self.get_register_value(helper.get_base());
                let imm_value = helper.get_offset();

                let address = base_value + imm_value;
                self.write_u16(address as usize, src_value as u16)
            }
            SOpcode::Sb(helper) => {
                let src_value = self.get_register_value(helper.get_src()) & 0xff;

                let base_value = self.get_register_value(helper.get_base());
                let imm_value = helper.get_offset();

                let address = base_value + imm_value;
                self.write_u8(address as usize, src_value as u8)
            }
        }
        false
    }
    fn execute_instruction_b(&mut self, opcode: BOpcode) -> bool {
        let mut pc_changed = false;
        match opcode {
            BOpcode::Beq(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                if src1_value == src2_value {
                    let offset_value = helper.get_offset();
                    let target = self.pc.get_value().overflowing_add(offset_value).0;
                    self.pc.set_value(target);

                    pc_changed = true;
                }
            }
            BOpcode::Bne(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                if src1_value != src2_value {
                    let offset_value = helper.get_offset();
                    let target = self.pc.get_value() + offset_value;
                    self.pc.set_value(target);

                    pc_changed = true;
                }
            }
            BOpcode::Bge(helper) => {
                let src1_value = self.get_register_value(helper.get_src1()) as i32;
                let src2_value = self.get_register_value(helper.get_src2()) as i32;

                if src1_value >= src2_value {
                    let offset_value = helper.get_offset();
                    let target = self.pc.get_value() + offset_value;
                    self.pc.set_value(target);

                    pc_changed = true;
                }
            }
            BOpcode::Bgeu(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                if src1_value >= src2_value {
                    let offset_value = helper.get_offset();
                    let target = self.pc.get_value() + offset_value;
                    self.pc.set_value(target);

                    pc_changed = true;
                }
            }
            BOpcode::Blt(helper) => {
                let src1_value = self.get_register_value(helper.get_src1()) as i32;
                let src2_value = self.get_register_value(helper.get_src2()) as i32;

                if src1_value < src2_value {
                    let offset_value = helper.get_offset();
                    let target = self.pc.get_value() + offset_value;
                    self.pc.set_value(target);

                    pc_changed = true;
                }
            }
            BOpcode::Bltu(helper) => {
                let src1_value = self.get_register_value(helper.get_src1());
                let src2_value = self.get_register_value(helper.get_src2());

                if src1_value < src2_value {
                    let offset_value = helper.get_offset();
                    let target = self.pc.get_value() + offset_value;
                    self.pc.set_value(target);

                    pc_changed = true;
                }
            }
        }
        pc_changed
    }
    fn execute_instruction_u(&mut self, opcode: UOpcode) -> bool {
        match opcode {
            // todo these might be buggy, not sure
            UOpcode::Auipc(helper) => {
                let imm = helper.get_imm();
                let target = self.pc.get_value() + imm;

                self.set_register_value(helper.get_dest(), target);
            }
            UOpcode::Lui(helper) => {
                let imm = helper.get_imm();

                self.set_register_value(helper.get_dest(), imm);
            }
        }
        false
    }
    fn execute_instruction_j(&mut self, opcode: JOpcode) -> bool {
        match opcode {
            JOpcode::Jal(helper) => {
                let offset = helper.get_offset();

                let return_address = self.pc.get_value() + 4;
                // save return address
                self.set_register_value(helper.get_dest(), return_address);

                let target = self.pc.get_value().overflowing_add(offset).0;

                self.pc.set_value(target);

                true
            }
        }
    }

    fn read_input(read_size: u32) -> Vec<u8> {
        // really don't like this
        let mut buffer = Vec::new();
        let mut stdin = io::stdin();

        let mut holder = [0; 1];
        for _ in 0..read_size {
            let result = stdin.read_exact(&mut holder);
            match result {
                Ok(_) => {
                    buffer.push(holder[0]);
                }
                Err(_) => {
                    break;
                }
            }
        }
        buffer
    }

    fn execute_ecall(&mut self) -> bool {
        let syscall_id = self.regs[10].get_value();

        match Syscalls::from_u32(syscall_id) {
            Syscalls::ReadInput => {
                let address = self.regs[11].get_value() as usize;
                let size = self.regs[12].get_value();

                // sanitity check
                if size as usize > MEMORY_SIZE {
                    panic!("Requesting more data than the memory can hold");
                }

                let input = VM::read_input(size);
                self.write_n(address, input);
            }
            Syscalls::Exit => {
                let exit_code = self.regs[11].get_value() as i32;
                println!("exit({exit_code})");
                exit(exit_code);
            }
            Syscalls::Puts => {
                let address = self.regs[11].get_value() as usize;
                let size = self.regs[12].get_value() as usize;

                let data = self.read_n(address, size);

                for b in data {
                    print!("{}", b as char);
                }

                println!();
            }
        }

        false
    }

    // whether executing instruction changed the pc
    pub fn execute_instruction(&mut self, instruction: InstructionFormat) -> bool {
        match instruction {
            InstructionFormat::R(opcode) => self.execute_instruction_r(opcode),
            InstructionFormat::I(opcode) => self.execute_instruction_i(opcode),
            InstructionFormat::S(opcode) => self.execute_instruction_s(opcode),
            InstructionFormat::B(opcode) => self.execute_instruction_b(opcode),
            InstructionFormat::U(opcode) => self.execute_instruction_u(opcode),
            InstructionFormat::J(opcode) => self.execute_instruction_j(opcode),
            InstructionFormat::ECALL => self.execute_ecall(),
        }
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        if self.flash.belongs(address, 1) {
            self.flash.write_8(address, value)
        } else if self.stack.belongs(address, 1) {
            self.stack.write_8(address, value)
        } else {
            panic!("Invalid 1-byte write address {:x}", address)
        }
    }

    fn read_u8(&self, address: usize) -> u8 {
        if self.flash.belongs(address, 1) {
            self.flash.read_u8(address)
        } else if self.stack.belongs(address, 1) {
            self.stack.read_u8(address)
        } else {
            panic!("Invalid 1-byte read address {:x}", address)
        }
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        if self.flash.belongs(address, 2) {
            self.flash.write_16(address, value)
        } else if self.stack.belongs(address, 2) {
            self.stack.write_16(address, value)
        } else {
            panic!("Invalid 2-byte write address {:x}", address)
        }
    }

    fn read_u16(&self, address: usize) -> u16 {
        if self.flash.belongs(address, 2) {
            self.flash.read_u16(address)
        } else if self.stack.belongs(address, 2) {
            self.stack.read_u16(address)
        } else {
            panic!("Invalid 2-byte read address {:x}", address)
        }
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        if self.flash.belongs(address, 4) {
            self.flash.write_u32(address, value)
        } else if self.stack.belongs(address, 4) {
            self.stack.write_u32(address, value)
        } else {
            panic!("Invalid 4-byte write address {:x}", address)
        }
    }

    pub fn read_u32(&self, address: usize) -> u32 {
        if self.flash.belongs(address, 4) {
            self.flash.read_u32(address)
        } else if self.stack.belongs(address, 4) {
            self.stack.read_u32(address)
        } else {
            panic!("Invalid 4-byte read address {:x}", address)
        }
    }

    fn write_n(&mut self, address: usize, data: Vec<u8>) {
        let nb_bytes = data.len();
        if self.flash.belongs(address, nb_bytes) {
            self.flash.write_n(address, data)
        } else if self.stack.belongs(address, nb_bytes) {
            self.stack.write_n(address, data)
        } else {
            panic!("Invalid {}-byte write address {:x}", nb_bytes, address)
        }
    }

    pub fn read_n(&mut self, address: usize, size: usize) -> Vec<u8> {
        if self.flash.belongs(address, size) {
            self.flash.read_n(address, size)
        } else if self.stack.belongs(address, size) {
            self.stack.read_n(address, size)
        } else {
            panic!("Invalid {}-byte read address {:x}", size, address)
        }
    }

    pub fn init_execution(&mut self) {
        let reset_handler_entry = FLASH_INTERRUPT_TABLE_RESET_ADDRESS;
        let reset_handler = self.flash.read_u32(reset_handler_entry);

        self.pc.set_value(reset_handler);

        // x2 is the stack register
        self.regs[2].set_value(STACK_ADDRESS as u32);
    }

    pub fn start_execution(&mut self) {
        loop {
            let instruction = self.flash.read_u32(self.pc.get_value() as usize);

            let decoded_instruction = decode(instruction);

            eprintln!("{:x} {}", self.pc.get_value(), decoded_instruction);

            // execute instruction
            let pc_changed = self.execute_instruction(decoded_instruction);

            if !pc_changed {
                self.pc.set_value(self.pc.get_value() + 4);
            }
        }
    }
}
