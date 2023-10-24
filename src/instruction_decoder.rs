use crate::{
    instructions::{
        BOpcode, BOpcodeHelper, IOpcode, IOpcodeHelper, InstructionFormat, JOpcode, JOpcodeHelper,
        ROpcode, ROpcodeHelper, SOpcode, SOpcodeHelper, ShamtOrRegister, UOpcode, UOpcodeHelper,
    },
    utils::{get_bits, sign_extend_number},
};

fn decode_r_shift(func3: u32, imm: u32, opcode_helper: ROpcodeHelper) -> ROpcode {
    if func3 == 0b001 && imm == 0 {
        ROpcode::Slli(opcode_helper)
    } else if func3 == 0b101 {
        if imm == 0 {
            ROpcode::Srli(opcode_helper)
        } else if imm == 0b0100000 {
            ROpcode::Srai(opcode_helper)
        } else {
            panic!("Shift instruction unknown {} {}", func3, imm);
        }
    } else {
        panic!("Shift instruction unknown {} {}", func3, imm);
    }
}

fn decode_r_others(func3: u32, imm: u32, opcode_helper: ROpcodeHelper) -> ROpcode {
    if func3 == 0 && imm == 0 {
        ROpcode::Add(opcode_helper)
    } else if func3 == 0 && imm == 0b0100000 {
        ROpcode::Sub(opcode_helper)
    } else if func3 == 1 && imm == 0 {
        ROpcode::Sll(opcode_helper)
    } else if func3 == 2 && imm == 0 {
        ROpcode::Slti(opcode_helper)
    } else if func3 == 3 && imm == 0 {
        ROpcode::Sltu(opcode_helper)
    } else if func3 == 4 && imm == 0 {
        ROpcode::Xor(opcode_helper)
    } else if func3 == 5 && imm == 0 {
        ROpcode::Srl(opcode_helper)
    } else if func3 == 5 && imm == 0b0100000 {
        ROpcode::Sra(opcode_helper)
    } else if func3 == 6 && imm == 0 {
        ROpcode::Or(opcode_helper)
    } else if func3 == 7 && imm == 0 {
        ROpcode::And(opcode_helper)
    } else {
        panic!(
            "R-type instruction not supported 0b0110011 {} {}",
            func3, imm
        )
    }
}

fn decode_r(instruction: u32) -> ROpcode {
    let opcode = get_bits(instruction, 0, 6);

    let mut opcode_helper = ROpcodeHelper::new(
        get_bits(instruction, 15, 19),
        get_bits(instruction, 7, 11),
        ShamtOrRegister::new(get_bits(instruction, 20, 24), true),
    );

    let func3 = get_bits(instruction, 12, 14);
    let imm = get_bits(instruction, 25, 31);

    if opcode == 0b0010011 {
        opcode_helper.set_is_register(false);
        decode_r_shift(func3, imm, opcode_helper)
    } else if opcode == 0b0110011 {
        decode_r_others(func3, imm, opcode_helper)
    } else {
        panic!(
            "R-type instruction not supported 0b0110011 {} {}",
            func3, imm
        )
    }
}

fn decode_b(instruction: u32) -> BOpcode {
    let opcode_helper = BOpcodeHelper::new(
        get_bits(instruction, 15, 19),
        get_bits(instruction, 20, 24),
        sign_extend_number(
            get_bits(instruction, 31, 31) << 12
                | get_bits(instruction, 7, 7) << 11
                | get_bits(instruction, 25, 30) << 5
                | get_bits(instruction, 8, 11) << 1,
            12,
        ),
    );

    let func3 = get_bits(instruction, 12, 14);

    match func3 {
        0 => BOpcode::Beq(opcode_helper),
        1 => BOpcode::Bne(opcode_helper),
        4 => BOpcode::Blt(opcode_helper),
        5 => BOpcode::Bge(opcode_helper),
        6 => BOpcode::Bltu(opcode_helper),
        7 => BOpcode::Bgeu(opcode_helper),
        _ => panic!("B-type instruction not supported {}", func3),
    }
}

fn decode_s(instruction: u32) -> SOpcode {
    let opcode_helper = SOpcodeHelper::new(
        get_bits(instruction, 20, 24),
        get_bits(instruction, 15, 19),
        sign_extend_number(
            get_bits(instruction, 25, 31) << 5 | get_bits(instruction, 7, 11),
            12,
        ),
    );

    let func3 = get_bits(instruction, 12, 14);

    match func3 {
        0 => SOpcode::Sb(opcode_helper),
        1 => SOpcode::Sh(opcode_helper),
        2 => SOpcode::Sw(opcode_helper),
        _ => panic!("S-type instruction not supported {}", func3),
    }
}

fn decode_u(instruction: u32) -> UOpcode {
    let opcode = get_bits(instruction, 0, 6);

    let opcode_helper = UOpcodeHelper::new(
        get_bits(instruction, 7, 11),
        get_bits(instruction, 12, 31) << 12,
    );

    match opcode {
        0b0110111 => UOpcode::Lui(opcode_helper),
        0b0010111 => UOpcode::Auipc(opcode_helper),
        _ => panic!("U-type instruction not supported {}", opcode),
    }
}

fn decode_j(instruction: u32) -> JOpcode {
    let opcode_helper = JOpcodeHelper::new(
        get_bits(instruction, 7, 11),
        sign_extend_number(
            get_bits(instruction, 31, 31) << 20
                | get_bits(instruction, 12, 19) << 12
                | get_bits(instruction, 20, 20) << 11
                | get_bits(instruction, 21, 30) << 1,
            21,
        ),
    );
    JOpcode::Jal(opcode_helper)
}

fn decode_i(instruction: u32) -> IOpcode {
    let opcode = get_bits(instruction, 0, 6);

    let opcode_helper = IOpcodeHelper::new(
        get_bits(instruction, 15, 19),
        get_bits(instruction, 7, 11),
        sign_extend_number(get_bits(instruction, 20, 31), 12),
    );

    let func3 = get_bits(instruction, 12, 14);

    if opcode == 0b1100111 {
        IOpcode::Jalr(opcode_helper)
    } else if opcode == 0b0000011 && func3 == 0 {
        IOpcode::Lb(opcode_helper)
    } else if opcode == 0b0000011 && func3 == 1 {
        IOpcode::Lh(opcode_helper)
    } else if opcode == 0b0000011 && func3 == 2 {
        IOpcode::Lw(opcode_helper)
    } else if opcode == 0b0000011 && func3 == 4 {
        IOpcode::Lbu(opcode_helper)
    } else if opcode == 0b0000011 && func3 == 5 {
        IOpcode::Lhu(opcode_helper)
    } else if opcode == 0b0010011 && func3 == 0 {
        IOpcode::Addi(opcode_helper)
    } else if opcode == 0b0010011 && func3 == 2 {
        IOpcode::Slti(opcode_helper)
    } else if opcode == 0b0010011 && func3 == 3 {
        IOpcode::Sltiu(opcode_helper)
    } else if opcode == 0b0010011 && func3 == 4 {
        IOpcode::Xori(opcode_helper)
    } else if opcode == 0b0010011 && func3 == 6 {
        IOpcode::Ori(opcode_helper)
    } else if opcode == 0b0010011 && func3 == 7 {
        IOpcode::Andi(opcode_helper)
    } else {
        panic!("I-type instruction not supported {} {}", opcode, func3)
    }
}

pub fn decode(instruction: u32) -> InstructionFormat {
    let opcode = get_bits(instruction, 0, 6);
    let func3 = get_bits(instruction, 12, 14);

    if opcode == 0b0010011 && (func3 == 1 || func3 == 5) || opcode == 0b0110011 {
        InstructionFormat::R(decode_r(instruction))
    } else if opcode == 0b1100111 || opcode == 0b0000011 || opcode == 0b0010011 {
        InstructionFormat::I(decode_i(instruction))
    } else if opcode == 0b0100011 {
        InstructionFormat::S(decode_s(instruction))
    } else if opcode == 0b1100011 {
        InstructionFormat::B(decode_b(instruction))
    } else if opcode == 0b0110111 || opcode == 0b0010111 {
        InstructionFormat::U(decode_u(instruction))
    } else if opcode == 0b1101111 {
        InstructionFormat::J(decode_j(instruction))
    } else if opcode == 0b1110011 {
        InstructionFormat::ECALL
    } else {
        panic!("Unknown instruction type {}", opcode)
    }
}
