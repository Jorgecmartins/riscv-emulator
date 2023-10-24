use std::fmt::{self};

#[derive(Debug)]
pub struct ShamtOrRegister {
    value: u32,
    is_register: bool,
}

impl ShamtOrRegister {
    pub fn new(value: u32, is_register: bool) -> Self {
        Self { value, is_register }
    }
    pub fn get_register(&self) -> u32 {
        assert!(self.is_register);
        self.value
    }

    pub fn get_shamt(&self) -> u32 {
        assert!(!self.is_register);
        self.value
    }
}

#[derive(Debug)]
pub struct ROpcodeHelper {
    src: u32,
    dest: u32,
    value: ShamtOrRegister,
}

impl ROpcodeHelper {
    pub fn new(src: u32, dest: u32, value: ShamtOrRegister) -> Self {
        Self { src, dest, value }
    }

    pub fn set_is_register(&mut self, is_register: bool) {
        self.value.is_register = is_register;
    }

    pub fn get_src1(&self) -> u32 {
        self.src
    }

    pub fn get_dest(&self) -> u32 {
        self.dest
    }

    pub fn get_shamt(&self) -> u32 {
        self.value.get_shamt()
    }

    pub fn get_src2(&self) -> u32 {
        self.value.get_register()
    }
}

#[derive(Debug)]
pub enum ROpcode {
    Slli(ROpcodeHelper),
    Srli(ROpcodeHelper),
    Srai(ROpcodeHelper),
    Add(ROpcodeHelper),
    Sub(ROpcodeHelper),
    Sll(ROpcodeHelper),
    Slti(ROpcodeHelper),
    Sltu(ROpcodeHelper),
    Xor(ROpcodeHelper),
    Srl(ROpcodeHelper),
    Sra(ROpcodeHelper),
    Or(ROpcodeHelper),
    And(ROpcodeHelper),
}

impl fmt::Display for ROpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ROpcode::Add(helper) => {
                let assembly = format!(
                    "Add {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Sub(helper) => {
                let assembly = format!(
                    "Sub {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Sll(helper) => {
                let assembly = format!(
                    "Sll {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Slti(helper) => {
                let assembly = format!(
                    "Slti {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Sltu(helper) => {
                let assembly = format!(
                    "Sltu {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Xor(helper) => {
                let assembly = format!(
                    "Xor {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Srl(helper) => {
                let assembly = format!(
                    "Srl {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Sra(helper) => {
                let assembly = format!(
                    "Sra {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Or(helper) => {
                let assembly = format!(
                    "Or {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::And(helper) => {
                let assembly = format!(
                    "And {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_register()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Slli(helper) => {
                let assembly = format!(
                    "And {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_shamt()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Srli(helper) => {
                let assembly = format!(
                    "And {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_shamt()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
            ROpcode::Srai(helper) => {
                let assembly = format!(
                    "And {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.value.get_shamt()),
                    get_register_name(helper.dest)
                );

                write!(f, "{}", assembly)
            }
        }
    }
}

#[derive(Debug)]
pub struct IOpcodeHelper {
    src: u32,
    dst: u32,
    imm: u32,
}

impl IOpcodeHelper {
    pub fn new(src: u32, dst: u32, imm: u32) -> Self {
        Self { src, dst, imm }
    }

    pub fn get_src(&self) -> u32 {
        self.src
    }

    pub fn get_dst(&self) -> u32 {
        self.dst
    }

    pub fn get_imm(&self) -> u32 {
        self.imm
    }
}

#[derive(Debug)]
pub enum IOpcode {
    Jalr(IOpcodeHelper),
    Lb(IOpcodeHelper),
    Lh(IOpcodeHelper),
    Lw(IOpcodeHelper),
    Lbu(IOpcodeHelper),
    Lhu(IOpcodeHelper),
    Addi(IOpcodeHelper),
    Slti(IOpcodeHelper),
    Sltiu(IOpcodeHelper),
    Xori(IOpcodeHelper),
    Ori(IOpcodeHelper),
    Andi(IOpcodeHelper),
}

impl fmt::Display for IOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOpcode::Jalr(helper) => {
                let assembly = format!("jalr {} + {}", get_register_name(helper.src), helper.imm);
                write!(f, "{}", assembly)
            }
            IOpcode::Lb(helper) => {
                let assembly = format!(
                    "lb {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Lh(helper) => {
                let assembly = format!(
                    "lh {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Lw(helper) => {
                let assembly = format!(
                    "lw {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Lbu(helper) => {
                let assembly = format!(
                    "lbu {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Lhu(helper) => {
                let assembly = format!(
                    "lhu {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Addi(helper) => {
                let assembly = format!(
                    "addi {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Slti(helper) => {
                let assembly = format!(
                    "slti {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Sltiu(helper) => {
                let assembly = format!(
                    "sltiu {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Xori(helper) => {
                let assembly = format!(
                    "xori {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Ori(helper) => {
                let assembly = format!(
                    "ori {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
            IOpcode::Andi(helper) => {
                let assembly = format!(
                    "andi {}, {}, {}",
                    get_register_name(helper.dst),
                    get_register_name(helper.src),
                    helper.imm
                );
                write!(f, "{}", assembly)
            }
        }
    }
}

#[derive(Debug)]
pub struct SOpcodeHelper {
    src: u32,
    base: u32,
    offset: u32,
}

impl SOpcodeHelper {
    pub fn new(src: u32, base: u32, offset: u32) -> Self {
        Self { src, base, offset }
    }

    pub fn get_src(&self) -> u32 {
        self.src
    }

    pub fn get_base(&self) -> u32 {
        self.base
    }

    pub fn get_offset(&self) -> u32 {
        self.offset
    }
}

#[derive(Debug)]
pub enum SOpcode {
    Sb(SOpcodeHelper),
    Sh(SOpcodeHelper),
    Sw(SOpcodeHelper),
}

impl fmt::Display for SOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SOpcode::Sb(helper) => {
                let assembly = format!(
                    "sb {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.base),
                    helper.offset
                );

                write!(f, "{}", assembly)
            }
            SOpcode::Sh(helper) => {
                let assembly = format!(
                    "sh {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.base),
                    helper.offset
                );

                write!(f, "{}", assembly)
            }
            SOpcode::Sw(helper) => {
                let assembly = format!(
                    "sw {}, {}, {}",
                    get_register_name(helper.src),
                    get_register_name(helper.base),
                    helper.offset
                );

                write!(f, "{}", assembly)
            }
        }
    }
}

#[derive(Debug)]
pub struct BOpcodeHelper {
    src1: u32,
    src2: u32,
    offset: u32,
}

impl BOpcodeHelper {
    pub fn new(src1: u32, src2: u32, offset: u32) -> Self {
        Self { src1, src2, offset }
    }

    pub fn get_src1(&self) -> u32 {
        self.src1
    }
    pub fn get_src2(&self) -> u32 {
        self.src2
    }
    pub fn get_offset(&self) -> u32 {
        self.offset
    }
}

#[derive(Debug)]
pub enum BOpcode {
    Beq(BOpcodeHelper),
    Bne(BOpcodeHelper),
    Blt(BOpcodeHelper),
    Bge(BOpcodeHelper),
    Bltu(BOpcodeHelper),
    Bgeu(BOpcodeHelper),
}

impl fmt::Display for BOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BOpcode::Beq(helper) => {
                let assembly = format!(
                    "beq {}, {}, {}",
                    get_register_name(helper.src1),
                    get_register_name(helper.src2),
                    helper.offset
                );

                write!(f, "{}", assembly)
            }

            BOpcode::Bne(helper) => {
                let assembly = format!(
                    "bne {}, {}, {}",
                    get_register_name(helper.src1),
                    get_register_name(helper.src2),
                    helper.offset
                );

                write!(f, "{}", assembly)
            }

            BOpcode::Blt(helper) => {
                let assembly = format!(
                    "blt {}, {}, {}",
                    get_register_name(helper.src1),
                    get_register_name(helper.src2),
                    helper.offset
                );

                write!(f, "{}", assembly)
            }
            BOpcode::Bltu(helper) => {
                let assembly = format!(
                    "beq {}, {}, {}",
                    get_register_name(helper.src1),
                    get_register_name(helper.src2),
                    helper.offset
                );

                write!(f, "{}", assembly)
            }
            BOpcode::Bge(helper) => {
                let assembly = format!(
                    "bge {}, {}, {}",
                    get_register_name(helper.src1),
                    get_register_name(helper.src2),
                    helper.offset
                );

                write!(f, "{}", assembly)
            }
            BOpcode::Bgeu(helper) => {
                let assembly = format!(
                    "bgeu {}, {}, {}",
                    get_register_name(helper.src1),
                    get_register_name(helper.src2),
                    helper.offset
                );

                write!(f, "{}", assembly)
            }
        }
    }
}

#[derive(Debug)]
pub struct UOpcodeHelper {
    dest: u32,
    imm: u32,
}

impl UOpcodeHelper {
    pub fn new(dest: u32, imm: u32) -> Self {
        Self { dest, imm }
    }

    pub fn get_dest(&self) -> u32 {
        self.dest
    }

    pub fn get_imm(&self) -> u32 {
        self.imm
    }
}

#[derive(Debug)]
pub enum UOpcode {
    Lui(UOpcodeHelper),
    Auipc(UOpcodeHelper),
}

impl fmt::Display for UOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UOpcode::Auipc(helper) => {
                let assembly = format!("auipc {}, {}", get_register_name(helper.dest), helper.imm);

                write!(f, "{}", assembly)
            }
            UOpcode::Lui(helper) => {
                let assembly = format!("lui {}, {}", get_register_name(helper.dest), helper.imm);

                write!(f, "{}", assembly)
            }
        }
    }
}

#[derive(Debug)]
pub struct JOpcodeHelper {
    dest: u32,
    offset: u32,
}

impl JOpcodeHelper {
    pub fn new(dest: u32, offset: u32) -> Self {
        Self { dest, offset }
    }

    pub fn get_dest(&self) -> u32 {
        self.dest
    }

    pub fn get_offset(&self) -> u32 {
        self.offset
    }
}

#[derive(Debug)]
pub enum JOpcode {
    Jal(JOpcodeHelper),
}

impl fmt::Display for JOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JOpcode::Jal(helper) => {
                let assembly = format!("jal {}", helper.offset);

                write!(f, "{}", assembly)
            }
        }
    }
}

#[derive(Debug)]
pub enum InstructionFormat {
    R(ROpcode),
    I(IOpcode),
    S(SOpcode),
    B(BOpcode),
    U(UOpcode),
    J(JOpcode),
    ECALL,
}

impl fmt::Display for InstructionFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionFormat::R(opcode) => write!(f, "{}", opcode),
            InstructionFormat::I(opcode) => write!(f, "{}", opcode),
            InstructionFormat::S(opcode) => write!(f, "{}", opcode),
            InstructionFormat::B(opcode) => write!(f, "{}", opcode),
            InstructionFormat::U(opcode) => write!(f, "{}", opcode),
            InstructionFormat::J(opcode) => write!(f, "{}", opcode),
            InstructionFormat::ECALL => write!(f, "ecall"),
        }
    }
}

fn get_register_name(register: u32) -> String {
    format!("x{register}")
}
