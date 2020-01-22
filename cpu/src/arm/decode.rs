use crate::enums::MnemonicARM;

/// get bit in a certain position
#[inline]
fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        return input & (1 << n) != 0;
    }
    false
}

/// gets n last bits
#[inline]
pub fn get_last_bits(input: u32, n: u8) -> u32 {
    if n < 32 {
        return input & ((1 << n) - 1);
    }
    panic!("tried to get >32 last bits");
}

#[inline]
fn bool_2_u8(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct DecodedInstruction {
    pub cond: u8,
    pub instr: MnemonicARM,
    pub rn: Option<u8>,   // index register
    pub rm: Option<u8>,   // second index register
    pub rd: Option<u8>,   // destination register
    pub rs: Option<u8>,   // source register
    pub val1: Option<u8>, // multi-purpose value (can be a shift to apply, etc)
    pub val2: Option<u8>, // ^
    pub val3: Option<u8>,
    pub offset: Option<u32>, // offset for branching

    pub set_cond: Option<bool>, // choose if should set condition codes
    pub imm: Option<bool>,      // whether the values come from registers or not
    pub acc: Option<bool>,      // whether the values should accumulate
}

pub fn data_processing(instruction: u32, cond: u8) -> DecodedInstruction {
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
        return DecodedInstruction {
            cond,
            instr,
            rn,
            rd,
            val1,
            val2,
            set_cond,
            ..Default::default()
        };
    }

    let val1 = Some(get_last_bits(instruction >> 4, 8) as u8); // shift applied to rm
    let rm = Some(get_last_bits(instruction, 4) as u8);

    // if val2 is none/rm is not none, the instruction is immediate
    DecodedInstruction {
        cond,
        instr,
        rn,
        rm,
        rd,
        val1,
        set_cond,
        ..Default::default()
    }
}

/// decodes BX, BLX instructions.
pub fn branch_exchange(instruction: u32, cond: u8) -> DecodedInstruction {
    let rn = Some(get_last_bits(instruction, 4) as u8);

    DecodedInstruction {
        cond,
        rn,
        instr: MnemonicARM::BX,
        ..Default::default()
    }
}

/// decodes B, BL instructions.
pub fn branch(instruction: u32, cond: u8) -> DecodedInstruction {
    let link = (instruction >> 24 & 1) as u8; // to link or not to link, that is the question...
    let instr = if link != 0 {
        MnemonicARM::BX
    } else {
        MnemonicARM::B
    };

    let offset = Some(get_last_bits(instruction, 24));

    DecodedInstruction {
        cond,
        instr,
        val1: Some(link),
        offset,
        ..Default::default()
    }
}

/// Reads PSR transfer statements
pub fn psr_transfer(instruction: u32, cond: u8) -> DecodedInstruction {
    let psr = Some(bool_2_u8(get_bit_at(instruction, 22)));

    let imm = get_bit_at(instruction, 25);
    // MRS (move to register from special register)
    if get_last_bits(instruction, 11) == 0 && !get_bit_at(instruction, 21) {
        let rd = Some(get_last_bits(instruction >> 12, 4) as u8);
        return DecodedInstruction {
            cond,
            instr: MnemonicARM::MRS,
            rd,
            val1: psr,
            imm: Some(false),
            ..Default::default()
        };
    }

    let instr = MnemonicARM::MSR;

    // MSR (transfer register contents to PSR)
    if get_bit_at(instruction, 16) {
        let rm = Some(get_last_bits(instruction, 4) as u8);
        return DecodedInstruction {
            cond,
            instr,
            rm,
            val1: psr,
            imm: Some(false),
            ..Default::default()
        };
    }

    // MSR (transfer register contents or imm value to PSR flag bits only)
    if imm {
        // the actual immediate value
        let value = Some(get_last_bits(instruction, 8) as u8);
        // shift applied to immediate value
        let shift = Some(get_last_bits(instruction >> 8, 4) as u8);

        return DecodedInstruction {
            cond,
            instr,
            imm: Some(true),
            val1: psr,
            val2: value,
            val3: shift,
            ..Default::default()
        };
    }

    // NSR w/ registers
    let rm = Some(get_last_bits(instruction, 4) as u8);
    DecodedInstruction {
        cond,
        instr,
        rm,
        val1: psr,
        imm: Some(false),
        ..Default::default()
    }
}

/// Stores/Writes to a register
pub fn data_transfer(instruction: u32, cond: u8) -> DecodedInstruction {
    // bitmasks
    let imm = get_bit_at(instruction, 25);
    let index = bool_2_u8(get_bit_at(instruction, 24)) << 3;
    let up_down = bool_2_u8(get_bit_at(instruction, 23)) << 2;
    let byte_or_word = bool_2_u8(get_bit_at(instruction, 22)) << 1;
    let write_back = bool_2_u8(get_bit_at(instruction, 21));

    let load = get_bit_at(instruction, 20);

    let rn = Some(get_last_bits(instruction >> 16, 4) as u8);

    let val1 = Some(0 | index | up_down | byte_or_word | write_back);

    let is_block_data = get_bit_at(instruction, 27);
    if is_block_data {
        let instr = if load {
            MnemonicARM::LDM
        } else {
            MnemonicARM::STM
        };

        // register list is 16 bits long, so we'll divide it into two parts, val2 being the high byte
        let val2 = Some(get_last_bits(instruction >> 8, 8) as u8);
        let val3 = Some(get_last_bits(instruction, 8) as u8);

        return DecodedInstruction {
            cond,
            instr,
            rn,
            val1,
            val2,
            val3,
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

        if imm {
            let val2 = Some(get_last_bits(instruction, 8) as u8);
            let val3 = Some(get_last_bits(instruction >> 8, 4) as u8);
            return DecodedInstruction {
                cond,
                instr,
                rn,
                rd,
                val1,
                val2,
                val3,
                imm: Some(true),
                ..Default::default()
            };
        }

        let rm = Some(get_last_bits(instruction, 4) as u8);

        // shift applied to rm
        let val2 = Some(get_last_bits(instruction >> 4, 8) as u8);

        return DecodedInstruction {
            cond,
            instr,
            rn,
            rd,
            rm,
            val1,
            val2,
            imm: Some(false),
            ..Default::default()
        };
    }

    let instr;
    let signed = get_bit_at(instruction, 6);
    let halfword = get_bit_at(instruction, 5);

    if !signed && !halfword {
        return swap(instruction, cond);
    }

    if load {
        instr = if signed {
            if halfword {
                MnemonicARM::LDRSH
            } else {
                MnemonicARM::LDRSB
            }
        } else {
            if halfword {
                MnemonicARM::LDRH
            } else {
                unreachable!()
            }
        }
    } else {
        instr = MnemonicARM::STRH;
    }

    // if not immediate
    if !get_bit_at(instruction, 22) {
        let rm = Some(get_last_bits(instruction, 4) as u8);

        return DecodedInstruction {
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

    let mut offset = (get_last_bits(instruction >> 8, 4) as u8) << 4;
    offset |= get_last_bits(instruction, 4) as u8;

    DecodedInstruction {
        cond,
        instr,
        rn,
        rd,
        val1,
        val2: Some(offset),
        imm: Some(true),
        ..Default::default()
    }
}

/// Reads multiply/mul long/mul half statements.
pub fn multiply(instruction: u32, cond: u8) -> DecodedInstruction {
    let rd = Some(get_last_bits(instruction >> 16, 4) as u8);
    let rn = Some(get_last_bits(instruction >> 12, 4) as u8);
    let rs = Some(get_last_bits(instruction >> 8, 4) as u8);
    let rm = Some(get_last_bits(instruction, 4) as u8);

    let acc = get_bit_at(instruction, 21);
    let set_cond = Some(get_bit_at(instruction, 20));

    // is it a long instruction?
    let long = get_bit_at(instruction, 23);

    let unsigned = get_bit_at(instruction, 22);
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

    DecodedInstruction {
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

pub fn swap(instruction: u32, cond: u8) -> DecodedInstruction {
    let is_byte = Some(bool_2_u8(get_bit_at(instruction, 22)));
    let rn = Some(get_last_bits(instruction >> 16, 4) as u8);
    let rd = Some(get_last_bits(instruction >> 12, 4) as u8);
    let rm = Some(get_last_bits(instruction, 4) as u8);

    DecodedInstruction {
        cond,
        instr: MnemonicARM::SWP,
        rn,
        rd,
        rm,
        val1: is_byte,
        ..Default::default()
    }
}

pub fn interrupt(instruction: u32, cond: u8) -> DecodedInstruction {
    // comment field
    let val1 = Some(get_last_bits(instruction >> 16, 8) as u8);
    let val2 = Some(get_last_bits(instruction >> 8, 8) as u8);
    let val3 = Some(get_last_bits(instruction, 8) as u8);

    DecodedInstruction {
        cond,
        instr: MnemonicARM::SWI,
        val1,
        val2,
        val3,
        ..Default::default()
    }
}

/// This enum describes each function defined above.
/// It is used to get the appropriate function for each binary without having to check through each function
#[derive(Debug, PartialEq)]
pub enum BaseInstruction {
    BranchAndExchange,
    Interrupt,
    Branch,
    DataTransfer,
    Multiply,
    DataProcessing,
    PSR,
}

impl BaseInstruction {
    // TODO: fucking do something about this lmao
    /// This function will get an instruction without the condition field (upper 4 bits of the 32).
    /// This function exists only to feed the decode functions, that will transform it into a decoded instruction
    pub fn get_instr(instruction: u32) -> BaseInstruction {
        use BaseInstruction::*;

        let instr: BaseInstruction;
        if get_last_bits(instruction >> 4, 24) == 0b0001_0010_1111_1111_1111_0001 {
            instr = BranchAndExchange;
        } else if get_last_bits(instruction >> 24, 4) == 0b1111 {
            instr = Interrupt;
        } else if get_last_bits(instruction >> 25, 3) == 0b101 {
            instr = Branch;
        } else if get_last_bits(instruction >> 25, 3) == 0b011
            || get_last_bits(instruction >> 25, 3) == 0b010
            || get_last_bits(instruction >> 25, 3) == 0b100
        {
            instr = DataTransfer;
        } else if (get_bit_at(instruction, 7) && get_bit_at(instruction, 4))
            && !get_bit_at(instruction, 25)
            || get_last_bits(instruction >> 23, 2) == 0b10
                && get_last_bits(instruction >> 18, 4) == 0
            || !get_bit_at(instruction, 22) && get_last_bits(instruction >> 18, 4) == 0
            || get_bit_at(instruction, 22)
        {
            instr = DataTransfer;
        } else if (get_last_bits(instruction >> 23, 5) == 0b10
            && get_bit_at(instruction, 7)
            && get_bit_at(instruction, 4))
            || get_last_bits(instruction >> 4, 4) == 0b1001
                && (get_last_bits(instruction >> 23, 5) == 0b1
                    || get_last_bits(instruction >> 23, 6) == 0b0)
        {
            instr = Multiply;
        } else if !get_bit_at(instruction, 20)
            && ((get_last_bits(instruction >> 23, 5) == 0b00010
                && get_last_bits(instruction >> 4, 8) == 0)
                || (get_last_bits(instruction >> 23, 5) == 0b00110 && get_bit_at(instruction, 21)))
        {
            instr = PSR;
        }
        // TODO: move this to an else if and handle unknown instructions in the else
        else {
            instr = DataProcessing;
        }

        instr
    }

    pub fn base_to_decoded(base: BaseInstruction, instr: u32, cond: u8) -> DecodedInstruction {
        use BaseInstruction::*;
        return match base {
            BranchAndExchange => branch_exchange(instr, cond),
            Interrupt => interrupt(instr, cond),
            Branch => branch(instr, cond),
            DataTransfer => data_transfer(instr, cond),
            Multiply => multiply(instr, cond),
            PSR => psr_transfer(instr, cond),
            DataProcessing => data_processing(instr, cond),
        };
    }
}
