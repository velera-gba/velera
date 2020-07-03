use crate::{
    thumb::ThumbDecodedInstruction,
    constants,
    enums::{MnemonicARM, ShiftType},
    utils::{get_bit_at, get_last_bits, get_shift_type},
};

pub fn base_to_decoded(instr: u16) -> ThumbDecodedInstruction {
    let instru32 = instr as u32;
    let bits15to13 = (instr >> 13) & 0b111;

    let bit12 = get_bit_at(instru32, 12);
    let bit11 = get_bit_at(instru32, 11);
    let bit10 = get_bit_at(instru32, 10);
    let bit9 = get_bit_at(instru32, 9);
    let bit8 = get_bit_at(instru32, 8);

    match (bits15to13, bit12, bit11, bit10, bit9, bit8) {
        (0b000, true, true, _, _, _) => add_subtract(instr),
        (0b000, _, _, _, _, _) => shifted(instr),
        (0b001, _, _, _, _, _) => immediate_compare(instr),
        (0b010, false, false, false, _, _) => alu_decode(instr),
        (0b010, false, false, true, _, _) => hireg_ops(instr),

        (0b010, false, true, _, _, _) | (0b010, true, _, _, _, _) | (0b100, _, _, _, _, _) => {
            load_store(instr)
        }

        (0b101, false, _, _, _, _) | (0b101, true, false, false, false, false) => add_decode(instr),

        (0b101, true, _, true, false, _) => push_pop(instr),
        (0b101, true, true, true, true, false) => breakpoint(instr),
        (0b110, false, _, _, _, _) => stm_ldm(instr),
        (0b110, true, _, _, _, _) => branch_cond(instr),
        (0b110, true, true, true, true, true) => interrupt(instr),
        (0b111, false, false, _, _, _) => branch_uncond(),
        (0b111, false, true, _, _, _) => blx(),
        (0b111, true, _, _, _, _) => bl(),
    }
}

fn add_subtract(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let imm = get_bit_at(instr, 10);
    let mnemonic = match get_bit_at(instr, 9) {
        true => MnemonicARM::SUB,
        false => MnemonicARM::ADD,
    };

    // rn or immediate value
    let val = Some(get_last_bits(instr >> 6, 3) as u8);
    let rs = Some(get_last_bits(instr >> 3, 3) as u8);
    let rd = Some(get_last_bits(instr, 3) as u8);

    if imm {
        ThumbDecodedInstruction {
            instr: mnemonic,
            imm: Some(true),
            val1: val,
            rs,
            rd,
            ..Default::default()
        }
    } else {
        ThumbDecodedInstruction {
            instr: mnemonic,
            imm: Some(false),
            rn: val,
            rs,
            rd,
            ..Default::default()
        }
    }
}

fn shifted(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    ThumbDecodedInstruction {
        instr: MnemonicARM::SHIFT,

        shift_type: Some(match get_last_bits(instr >> 11, 2) {
            0b00 => ShiftType::LSL,
            0b01 => ShiftType::LSR,
            0b10 => ShiftType::ASR,
        }),

        rs: Some(get_last_bits(instr >> 3, 3) as u8),
        rd: Some(get_last_bits(instr, 3) as u8),

        offset: Some(get_last_bits(instr >> 6, 4) as i32),

        ..Default::default()
    }
}

fn immediate_compare(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let opcode = match get_last_bits(instr >> 11, 2) {
        0b00 => MnemonicARM::MOVI,
        0b01 => MnemonicARM::CMPI,
        0b10 => MnemonicARM::ADDI,
        0b11 => MnemonicARM::SUBI,
    };
    let rd = Some(get_last_bits(instr >> 8, 3) as u8);
    let val1 = Some(get_last_bits(instr, 8) as u8);
    ThumbDecodedInstruction {
        instr: opcode,
        rd,
        val1,
        ..Default::default()
    }
}

fn alu_decode(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let opcode = match get_last_bits(instr >> 6, 4) {
        0x0 => MnemonicARM::AND,
        0x1 => MnemonicARM::EOR,
        0x2 => MnemonicARM::LSLALU,
        0x3 => MnemonicARM::LSRALU,
        0x4 => MnemonicARM::ASRALU,
        0x5 => MnemonicARM::ADC,
        0x6 => MnemonicARM::SBC,
        0x7 => MnemonicARM::ROR,
        0x8 => MnemonicARM::TST,
        0x9 => MnemonicARM::NEG,
        0xA => MnemonicARM::CMP,
        0xB => MnemonicARM::CMN,
        0xC => MnemonicARM::ORR,
        0xD => MnemonicARM::MUL,
        0xE => MnemonicARM::BIC,
        0xF => MnemonicARM::MVN,
    };
    let rs = Some(get_last_bits(instr >> 3, 3) as u8);
    let rd = Some(get_last_bits(instr, 3) as u8);
    ThumbDecodedInstruction {
        instr: opcode,
        rd,
        rs,
        ..Default::default()
    }
}

fn hireg_ops(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let opcode = match get_last_bits(instr >> 8, 2) {
        0b00 => MnemonicARM::ADDHI,
        0b01 => MnemonicARM::CMPHI,
        0b10 => MnemonicARM::MOVHI,
        0b11 => MnemonicARM::BX,
    };

    let msbd = Some(get_bit_at(instr, 7) as u8);
    let msbs = get_bit_at(instr, 6) as u8;
    let rs = Some((get_last_bits(instr >> 3, 3) as u8 | (msbs << 3)) as u8);
    let rd = Some(get_last_bits(instr, 3) as u8);

    ThumbDecodedInstruction {
        instr: opcode,
        val1: msbd,
        rs,
        rd,
        ..Default::default()
    }
}

fn load_pc(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let rd = Some(get_last_bits(instr >> 8, 3) as u8);
    let offset = Some(get_last_bits(instr, 8) as u8);
    ThumbDecodedInstruction {
        instr: MnemonicARM::LDRPC,
        rd,
        val1: offset,
        ..Default::default()
    }
}

fn load_store_reg(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let opcode = match get_last_bits(instr >> 10, 2) {
        0b00 => MnemonicARM::STR,
        0b01 => MnemonicARM::STRB,
        0b10 => MnemonicARM::LDR,
        0b11 => MnemonicARM::LDRB,
    };
    let rn = Some(get_last_bits(instr >> 6, 3) as u8);
    let rm = Some(get_last_bits(instr >> 3, 3) as u8);
    let rd = Some(get_last_bits(instr, 3) as u8);
    ThumbDecodedInstruction {
        instr: opcode,
        rn,
        rm,
        rd,
        imm: Some(false),
        ..Default::default()
    }
}

fn load_store_sign_extended(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let opcode = match get_last_bits(instr >> 10, 2) {
        0b00 => MnemonicARM::STRH,
        0b01 => MnemonicARM::LDSB,
        0b10 => MnemonicARM::LDRH,
        0b11 => MnemonicARM::LDSH,
    };
    let rn = Some(get_last_bits(instr >> 6, 3) as u8);
    let rm = Some(get_last_bits(instr >> 3, 3) as u8);
    let rd = Some(get_last_bits(instr, 3) as u8);
    ThumbDecodedInstruction {
        instr: opcode,
        rn,
        rm,
        rd,
        ..Default::default()
    }
}

fn load_store_imm(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let opcode = match get_last_bits(instr >> 11, 2) {
        0b00 => MnemonicARM::STR,
        0b01 => MnemonicARM::STRB,
        0b10 => MnemonicARM::LDR,
        0b11 => MnemonicARM::LDRB,
    };

    let val1 = Some(get_last_bits(instr >> 6, 4) as u8);
    let rn = Some(get_last_bits(instr >> 3, 3) as u8);
    let rd = Some(get_last_bits(instr, 3) as u8);

    ThumbDecodedInstruction {
        instr: opcode,
        rn,
        rd,
        val1,
        imm: Some(true),
        ..Default::default()
    }
}

fn load_store_half(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let mnemonic = match get_bit_at(instr, 11) {
        false => MnemonicARM::STRH,
        true => MnemonicARM::LDRH,
    };
    let val1 = Some(get_last_bits(instr >> 6, 5) as u8);
    let rn = Some(get_last_bits(instr >> 3, 3) as u8);
    let rd = Some(get_last_bits(instr, 3) as u8);

    ThumbDecodedInstruction {
        instr: mnemonic,
        rn,
        rd,
        val1,
        ..Default::default()
    }
}

fn load_store_sp_relative(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let mnemonic = match get_bit_at(instr, 11) {
        false => MnemonicARM::STRSPREL,
        true => MnemonicARM::LDRSPREL,
    };

    let rd = Some(get_last_bits(instr >> 8, 3) as u8);
    let val1 = Some(get_last_bits(instr, 8) as u8);
    ThumbDecodedInstruction {
        instr: mnemonic,
        rd,
        val1,
        ..Default::default()
    }
}

fn get_relative_address(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let mnemonic = match get_bit_at(instr, 11) {
        false => MnemonicARM::ADDPC,
        true => MnemonicARM::ADDSP,
    };

    let rd = Some(get_last_bits(instr >> 8, 3) as u8);
    let val1 = Some(get_last_bits(instr, 8) as u8);
    ThumbDecodedInstruction {
        instr: mnemonic,
        rd,
        val1,
        ..Default::default()
    }
}

fn add_offset(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let mnemonic = match get_bit_at(instr, 7) {
        false => MnemonicARM::OffsetADD,
        true => MnemonicARM::OffsetSUB,
    };

    let val1 = Some(get_last_bits(instr, 7) as u8);

    ThumbDecodedInstruction {
        instr: mnemonic,
        val1,
        ..Default::default()
    }
}

fn push_pop(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;
    let mnemonic = match (get_bit_at(instr, 11), get_bit_at(instr, 8)) {
        (false, false) => MnemonicARM::PUSH,
        (true, false) => MnemonicARM::POP,
        (false, true) => MnemonicARM::PUSHLR,
        (true, true) => MnemonicARM::POPPC,
    };

    // rlist
    let val1 = Some(get_last_bits(instr, 8) as u8);
    ThumbDecodedInstruction {
        instr: mnemonic,
        val1,
        ..Default::default()
    }
}


fn stm_ldm(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;

    let mnemonic = match get_bit_at(instr, 11) {
        false => MnemonicARM::STMIA,
        true => MnemonicARM::LDMIA,
    };

    let rn = Some(get_last_bits(instr >> 8, 3) as u8);
    // rlist
    let val1 = Some(get_last_bits(instr, 8) as u8);

    ThumbDecodedInstruction {
        instr: mnemonic,
        val1,
        rn,
        ..Default::default()
    }
}

fn breakpoint(instr: u16) -> ThumbDecodedInstruction {
    ThumbDecodedInstruction {
        instr: MnemonicARM::BKPT,
        ..Default::default()
    }
}

fn interrupt(instr: u16) -> ThumbDecodedInstruction {
    ThumbDecodedInstruction {
        instr: MnemonicARM::SWI,
        ..Default::default()
    }
}

// BOTAR TODOS OS POSSIVEIS CONDICIONAIS NO ENUMS.RS PQ SE A
// EXECUÇÃO NÃO ROLAR, O NUMERO DE CICLOS MUDA
// (E CONSEQUENTEMENTE TIRAR O opcode)
// @Arsukeey (Alice Micheloni)

fn branch_cond(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;

    // offset
    let val1 = Some(get_last_bits(instr, 8) as u8);

    // opcode
    let val2 = Some(get_last_bits(instr >> 8, 4) as u8);

    ThumbDecodedInstruction {
        instr: MnemonicARM::BCond,
        val1,
        val2,
        ..Default::default()
    }
}

fn branch_uncond(instr: u16) -> ThumbDecodedInstruction {
    let instr = instr as u32;

    // offset lo
    let val1 = Some(get_last_bits(instr, 8) as u8);

    // offset hi
    let val2 = Some(get_last_bits(instr >> 8, 3) as u8);

    ThumbDecodedInstruction {
        instr: MnemonicARM::B,
        val1,
        val2,
        ..Default::default()
    }
}
