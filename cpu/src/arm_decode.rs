use crate::{
    arm::ARMDecodedInstruction,
    enums::MnemonicARM,
    utils::{get_bit_at, get_last_bits, get_shift_type},
};

pub fn data_processing(instruction: u32, cond: u8) -> ARMDecodedInstruction {
    use crate::constants::dp_opcodes::*;

    let imm = get_bit_at(instruction, 25);
    let set_cond = Some(get_bit_at(instruction, 20));

    let instr = get_last_bits(instruction >> 21, 4) as u8;
    let rn = Some(get_last_bits(instruction >> 16, 4) as u8);
    let rd = Some(get_last_bits(instruction >> 12, 4) as u8);

    // is it possible to change this to a stringify! statement?
    let instr = match instr {
        AND => MnemonicARM::AND,
        EOR => MnemonicARM::EOR,
        SUB => MnemonicARM::SUB,
        RSB => MnemonicARM::RSB,
        ADD => MnemonicARM::ADD,
        ADC => MnemonicARM::ADC,
        SBC => MnemonicARM::SBC,
        RSC => MnemonicARM::RSC,
        TST => MnemonicARM::TST,
        TEQ => MnemonicARM::TEQ,
        CMP => MnemonicARM::CMP,
        CMN => MnemonicARM::CMN,
        ORR => MnemonicARM::ORR,
        MOV => MnemonicARM::MOV,
        BIC => MnemonicARM::BIC,
        MVN => MnemonicARM::MVN,
        _ => unreachable!(),
    };

    if imm {
        let val1 = Some(get_last_bits(instruction >> 8, 4) as u8); // shift applied to imm
        let val2 = Some(get_last_bits(instruction, 8) as u8); // immediate value
        return ARMDecodedInstruction {
            cond,
            instr,
            rn,
            rd,
            val1,
            val2,
            imm: Some(true),
            set_cond,
            ..Default::default()
        };
    }

    // Register as second operand
    let rm = Some(get_last_bits(instruction, 4) as u8);
    let shift_type = get_last_bits(instruction >> 5, 2);
    let shift_type = Some(get_shift_type(shift_type));

    let shift_by_register = get_bit_at(instruction, 4);
    if shift_by_register {
        let rs = Some(get_last_bits(instruction >> 8, 4) as u8);
        ARMDecodedInstruction {
            cond,
            instr,
            rn,
            rm,
            rd,
            rs,
            shift_type,
            set_cond,
            imm: Some(false),
            ..Default::default()
        }
    } else {
        // immediate amount to shift
        let val1 = Some(get_last_bits(instruction >> 7, 5) as u8);
        ARMDecodedInstruction {
            cond,
            instr,
            rn,
            rm,
            rd,
            val1,
            shift_type,
            set_cond,
            imm: Some(false),
            ..Default::default()
        }
    }
}

/// decodes BX, BLX instructions.
pub fn branch_exchange(instruction: u32, cond: u8) -> ARMDecodedInstruction {
    let rn = Some(get_last_bits(instruction, 4) as u8);

    ARMDecodedInstruction {
        cond,
        rn,
        instr: MnemonicARM::BX,
        ..Default::default()
    }
}

/// decodes B, BL instructions.
pub fn branch(instruction: u32, cond: u8) -> ARMDecodedInstruction {
    let link = (instruction >> 24 & 1) as u8; // to link or not to link, that is the question...
    let instr = if link != 0 {
        MnemonicARM::BX
    } else {
        MnemonicARM::B
    };

    let offset = Some(get_last_bits(instruction, 24) as i32);

    ARMDecodedInstruction {
        cond,
        instr,
        val1: Some(link),
        offset,
        ..Default::default()
    }
}

/// Reads PSR transfer statements
pub fn psr_transfer(instruction: u32, cond: u8) -> ARMDecodedInstruction {
    let psr = Some(get_bit_at(instruction, 22) as u8);

    let imm = get_bit_at(instruction, 25);
    // MRS (move to register from special register)
    if get_last_bits(instruction, 11) == 0 && !get_bit_at(instruction, 21) {
        let rd = Some(get_last_bits(instruction >> 12, 4) as u8);
        return ARMDecodedInstruction {
            cond,
            instr: MnemonicARM::MRS,
            rd,
            val1: psr,
            imm: Some(false),
            ..Default::default()
        };
    }

    let instr = MnemonicARM::MSR;

    // flags
    let offset = Some(get_last_bits(instruction >> 16, 4) as i32);

    // MSR (transfer register contents to PSR)
    if get_bit_at(instruction, 16) {
        let rm = Some(get_last_bits(instruction, 4) as u8);
        return ARMDecodedInstruction {
            cond,
            instr,
            rm,
            val1: psr,
            imm: Some(false),
            offset,
            ..Default::default()
        };
    }

    // MSR (transfer register contents or imm value to PSR flag bits only)
    if imm {
        // the actual immediate value
        let value = Some(get_last_bits(instruction, 8) as u8);
        // shift applied to immediate value
        let shift = Some(get_last_bits(instruction >> 8, 4) as u8);

        return ARMDecodedInstruction {
            cond,
            instr,
            imm: Some(true),
            val1: psr,
            val2: value,
            val3: shift,
            offset,
            ..Default::default()
        };
    }

    // MSR w/ registers
    let rm = Some(get_last_bits(instruction, 4) as u8);
    ARMDecodedInstruction {
        cond,
        instr,
        rm,
        val1: psr,
        imm: Some(false),
        offset,
        ..Default::default()
    }
}

/// Stores/Writes to a register
pub fn data_transfer(instruction: u32, cond: u8) -> ARMDecodedInstruction {
    // bitmasks
    let imm = get_bit_at(instruction, 25);

    let pre = (get_bit_at(instruction, 24) as u8) << 3;
    let up_down = (get_bit_at(instruction, 23) as u8) << 2;
    let byte = (get_bit_at(instruction, 22) as u8) << 1;
    let writeback = get_bit_at(instruction, 21) as u8;

    let load = get_bit_at(instruction, 20);

    let rn = Some(get_last_bits(instruction >> 16, 4) as u8);

    let val1 = Some(pre | up_down | byte | writeback);

    let is_block_data = get_bit_at(instruction, 27);
    if is_block_data {
        let instr = if load {
            MnemonicARM::LDM
        } else {
            MnemonicARM::STM
        };

        let offset = Some(get_last_bits(instruction, 16) as i32);

        return ARMDecodedInstruction {
            cond,
            instr,
            val1,
            offset,
            rn,
            ..Default::default()
        };
    }
    let rd = Some(get_last_bits(instruction >> 12, 4) as u8);

    // checks if it is a single data transfer (simple)
    if get_bit_at(instruction, 26) {
        let instr = if load {
            MnemonicARM::LDR
        } else {
            MnemonicARM::STR
        };

        if !imm {
            let offset = Some(get_last_bits(instruction, 12) as i32);
            return ARMDecodedInstruction {
                cond,
                instr,
                rd,
                rn,
                val1,
                offset,
                imm: Some(true),
                ..Default::default()
            };
        }

        let rm = Some(get_last_bits(instruction, 4) as u8);

        // shift applied to rm
        let val2 = Some(get_last_bits(instruction >> 7, 5) as u8);
        let shift_type = get_last_bits(instruction >> 5, 2);
        let shift_type = Some(get_shift_type(shift_type));

        return ARMDecodedInstruction {
            cond,
            instr,
            rn,
            rd,
            rm,
            val1,
            val2,
            shift_type,
            imm: Some(false),
            ..Default::default()
        };
    }

    let signed = get_bit_at(instruction, 6);
    let halfword = get_bit_at(instruction, 5);

    if !signed && !halfword {
        return swap(instruction, cond);
    }

    let instr = match (load, signed, halfword) {
        (false, _, _) => MnemonicARM::STRH,
        (true, true, true) => MnemonicARM::LDRSH,
        (true, false, true) => MnemonicARM::LDRH,
        (true, true, false) => MnemonicARM::LDRSB,
        _ => unreachable!(),
    };

    // if not immediate
    if !get_bit_at(instruction, 22) {
        let rm = Some(get_last_bits(instruction, 4) as u8);

        return ARMDecodedInstruction {
            cond,
            instr,
            rn,
            rd,
            rm,
            val1,
            imm: Some(false),
            ..Default::default()
        };
    }

    let mut offset = (get_last_bits(instruction >> 8, 4) as i32) << 4;
    offset |= get_last_bits(instruction, 4) as i32;

    ARMDecodedInstruction {
        cond,
        instr,
        rn,
        rd,
        val1,
        offset: Some(offset),
        imm: Some(true),
        ..Default::default()
    }
}

/// Reads multiply/mul long/mul half statements.
pub fn multiply(instruction: u32, cond: u8) -> ARMDecodedInstruction {
    let rd = Some(get_last_bits(instruction >> 16, 4) as u8);
    let rn = Some(get_last_bits(instruction >> 12, 4) as u8);
    let rs = Some(get_last_bits(instruction >> 8, 4) as u8);
    let rm = Some(get_last_bits(instruction, 4) as u8);

    // is it a long instruction?
    let long = get_bit_at(instruction, 23);
    let unsigned = get_bit_at(instruction, 22);
    let acc = get_bit_at(instruction, 21);
    let set_cond = Some(get_bit_at(instruction, 20));

    let instr;

    if long {
        if acc {
            instr = if unsigned {
                MnemonicARM::UMLAL
            } else {
                MnemonicARM::SMLAL
            };
        } else {
            instr = if unsigned {
                MnemonicARM::UMULL
            } else {
                MnemonicARM::SMULL
            };
        }
    } else {
        instr = if acc {
            MnemonicARM::MLA
        } else {
            MnemonicARM::MUL
        };
    }

    ARMDecodedInstruction {
        cond,
        instr,
        rd,
        rn,
        rs,
        rm,
        set_cond,
        ..Default::default()
    }
}

pub fn swap(instruction: u32, cond: u8) -> ARMDecodedInstruction {
    let is_byte = Some((get_bit_at(instruction, 22)) as u8);
    let rn = Some(get_last_bits(instruction >> 16, 4) as u8);
    let rd = Some(get_last_bits(instruction >> 12, 4) as u8);
    let rm = Some(get_last_bits(instruction, 4) as u8);

    ARMDecodedInstruction {
        cond,
        instr: MnemonicARM::SWP,
        rn,
        rd,
        rm,
        val1: is_byte,
        ..Default::default()
    }
}

pub fn interrupt(instruction: u32, cond: u8) -> ARMDecodedInstruction {
    // comment field
    let val1 = Some(get_last_bits(instruction >> 16, 8) as u8);
    let val2 = Some(get_last_bits(instruction >> 8, 8) as u8);
    let val3 = Some(get_last_bits(instruction, 8) as u8);

    ARMDecodedInstruction {
        cond,
        instr: MnemonicARM::SWI,
        val1,
        val2,
        val3,
        ..Default::default()
    }
}

pub fn undefined(_instruction: u32, cond: u8) -> ARMDecodedInstruction {
    ARMDecodedInstruction {
        cond,
        instr: MnemonicARM::Undefined,
        ..Default::default()
    }
}

pub fn base_to_decoded(instruction: u32) -> ARMDecodedInstruction {
    // believe me its a lot better than the other solution
    let cond = get_last_bits(instruction >> 28, 4) as u8;

    let bits27to25 = get_last_bits(instruction >> 25, 3);

    let bit24 = get_bit_at(instruction, 24);
    let bit23 = get_bit_at(instruction, 23);
    let bit22 = get_bit_at(instruction, 22);
    let bit21 = get_bit_at(instruction, 21);
    let bit20 = get_bit_at(instruction, 20);
    let bit7 = get_bit_at(instruction, 7);
    let bit4 = get_bit_at(instruction, 4);

    let byte4 = get_last_bits(instruction >> 16, 4);
    let byte5 = get_last_bits(instruction >> 12, 4);
    let byte6 = get_last_bits(instruction >> 8, 4);
    let byte7 = get_last_bits(instruction >> 4, 4);
    let byte8 = get_last_bits(instruction, 4);

    match (
            cond, bits27to25, bit24, bit23, bit22, bit21, bit20, byte4, byte5, byte6, byte7, byte8,
            bit7, bit4,
        ) {
            // BX, BLX
            (_, 0b000, true, false, true, false, false, 0b1111, 0b1111, 0b1111, 0b0001, _, _, _) => {
                branch_exchange(instruction, cond)
            }

            // SWI
            (_, 0b111, true, _, _, _, _, _, _, _, _, _, _, _) => interrupt(instruction, cond),

            // B, BL, BLX
            (_, 0b101, _, _, _, _, _, _, _, _, _, _, _, _) => branch(instruction, cond),

            // TransReg9
            (_, 0b011, _, _, _, _, _, _, _, _, _, _, _, false) |
            // TransImm9
            (_, 0b010, _, _, _, _, _, _, _, _, _, _, _, _) |
            // Block Trans
            (_, 0b100, _, _, _, _, _, _, _, _, _, _, _, _) |
            // TransImm10, TransReg10, TransSwp12
            (_, 0b000, _, _, _, _, _, _, _, 0b0000, _, _, true, true) => data_transfer(instruction, cond),

            // Multiply
            (_, 0b000, false, false, false, _, _, _, _, _, 0b1001, _, _, _) |
            // MulLong
            (_, 0b000, false, true, _, _, _, _, _, _, 0b1001, _, _, _) |
            // MulHalf
            (_, 0b000, true, false, _, _, false, _, _, _, _, _, true, false) => multiply(instruction, cond),

            // PSR Imm
            (_, 0b001, true, false, _, true, false, _, _, _, _, _, _, _) |
            // PSR Reg
            (_, 0b000, true, false, _, _, false, _, _, 0b0000, 0b0000, _, _, _) => psr_transfer(instruction, cond),

            (_, 0b000, _, _, _, _, _, _, _, _, _, _, _, false) |
            (_, 0b000, _, _, _, _, _, _, _, _, _, _, false, true) |
            (_, 0b001, _, _, _, _, _, _, _, _, _, _, _, _) => data_processing(instruction, cond),

            (_, 0b011, _, _, _, _, _, _, _, _, _, _, _, true) => undefined(instruction, cond),

            _ => panic!(format!("Undefined instruction at decode: {}!", instruction)),
        }
}
