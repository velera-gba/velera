use crate::{
    constants,
    constants::registers,
    cpu::CPU,
    enums::{InstructionType, MnemonicARM, MnemonicARM::*},
};

/// Does nothing at all. used as a placeholder.
pub fn dummy_cycle(_cpu: &mut CPU) {}

// Start branch micro operations

/// Stores the program counter value in the link register
#[inline]
pub fn store_pc_to_lr(cpu: &mut CPU) {
    cpu.arm.registers[registers::LINK_REGISTER] = cpu.arm.registers[registers::PROGRAM_COUNTER];
}

/// Increases the program counter
pub fn increase_pc_by_offset(cpu: &mut CPU) {
    match &cpu.decoded_instruction {
        InstructionType::Thumb(_) => {
            unimplemented!();
        }

        InstructionType::ARM(instr) => {
            if let Some(decoded) = &instr.decoded_instruction {
                if let Some(offset) = decoded.offset {
                    cpu.arm.registers[registers::PROGRAM_COUNTER] += offset;
                } else {
                    eprintln!("Expected offset in branch instruction");
                }
            } else {
                eprintln!("Expected decoded instruction");
            }
        }
    }
}

/// Switches from arm mode to thumb or vice versa (Branch eXchange)
pub fn switch_mode(cpu: &mut CPU) {
    match &cpu.decoded_instruction {
        InstructionType::Thumb(_) => {
            unimplemented!();
        }

        InstructionType::ARM(instr) => {
            if let Some(decoded) = &instr.decoded_instruction {
                if let Some(rn) = decoded.rn {
                    let is_thumb = cpu.arm.registers[rn as usize] & 0x01;
                    cpu.arm.cpsr |= (is_thumb << constants::cpsr_flags::STATE_BIT) as u32;
                } else {
                    eprintln!("Expected to find rn");
                }
            } else {
                eprintln!("Expected decoded instruction");
            }
        }
    }
}

// End branch micro operations
// ------------------------------
// Start multiply micro operations

// TODO: treat overflows
// rd = rm * rs
pub fn multiply(cpu: &mut CPU) {
    let (rd, rm, rs);
    match &cpu.decoded_instruction {
        InstructionType::Thumb(_) => {
            unimplemented!();
        }

        InstructionType::ARM(instr) => {
            if let Some(decoded) = &instr.decoded_instruction {
                rd = decoded.rd.unwrap() as usize;
                rm = decoded.rm.unwrap() as usize;
                rs = decoded.rs.unwrap() as usize;
            } else {
                eprintln!("Expected decoded instruction at multiply instruction");
            }
        }
    }

    cpu.arm.registers[rd] = cpu.arm.registers[rm] * cpu.arm.registers[rs];
}

// rd = rm * rs + rn
pub fn multiply_accumulate(cpu: &mut CPU) {
    let (rd, rm, rs, rn);
    match &cpu.decoded_instruction {
        InstructionType::Thumb(_) => {
            unimplemented!();
        }

        InstructionType::ARM(instr) => {
            if let Some(decoded) = &instr.decoded_instruction {
                rd = decoded.rd.unwrap() as usize;
                rm = decoded.rm.unwrap() as usize;
                rs = decoded.rs.unwrap() as usize;
                rn = decoded.rn.unwrap() as usize;
            } else {
                eprintln!("Expected decoded instruction at multiply accumulate instruction");
            }
        }
    }

    cpu.arm.registers[rd] = cpu.arm.registers[rm] * cpu.arm.registers[rs] + cpu.arm.registers[rn];
}

pub fn signed_multiply(cpu: &mut CPU) {
    let (rm, rs, rd_low, rd_hi);
    match &cpu.decoded_instruction {
        InstructionType::Thumb(_) => {
            unimplemented!();
        }

        InstructionType::ARM(instr) => {
            if let Some(decoded) = &instr.decoded_instruction {
                rm = decoded.rm.unwrap() as usize;
                rs = decoded.rs.unwrap() as usize;
                rd_low = decoded.rn.unwrap() as usize;
                rd_hi = decoded.rd.unwrap() as usize;
            } else {
                eprintln!("Expected decoded instruction at multiply accumulate instruction");
            }
        }
    }

    let r = cpu.arm.registers[rm] as i64 * cpu.arm.registers[rs] as i64;

    cpu.arm.registers[rd_low] = (r & 0xFFFF_FFFF) as i32;
    cpu.arm.registers[rd_hi] = (r >> 32) as i32;
}

pub fn unsigned_multiply(cpu: &mut CPU) {
    let (rm, rs, rd_low, rd_hi);
    match &cpu.decoded_instruction {
        InstructionType::Thumb(_) => {
            unimplemented!();
        }

        InstructionType::ARM(instr) => {
            if let Some(decoded) = &instr.decoded_instruction {
                rm = decoded.rm.unwrap() as usize;
                rs = decoded.rs.unwrap() as usize;
                rd_low = decoded.rn.unwrap() as usize;
                rd_hi = decoded.rd.unwrap() as usize;
            } else {
                eprintln!("Expected decoded instruction at multiply accumulate instruction");
            }
        }
    }

    let r = cpu.arm.registers[rm] as u64 * cpu.arm.registers[rs] as u64;

    cpu.arm.registers[rd_low] = (r & 0xFFFF_FFFF) as i32;
    cpu.arm.registers[rd_hi] = (r >> 32) as i32;
}

pub fn signed_multiply_accumulate(cpu: &mut CPU) {
    let (rd_hi, rd_low, rm, rs);
    match &cpu.decoded_instruction {
        InstructionType::Thumb(_) => {
            unimplemented!();
        }

        InstructionType::ARM(instr) => {
            if let Some(decoded) = &instr.decoded_instruction {
                rm = decoded.rm.unwrap() as usize;
                rs = decoded.rs.unwrap() as usize;
                rd_hi = decoded.rd.unwrap() as usize;
                rd_low = decoded.rn.unwrap() as usize;
            } else {
                eprintln!("Expected decoded instruction at multiply accumulate instruction");
            }
        }
    }

    let r = cpu.arm.registers[rm] as i64 * cpu.arm.registers[rs] as i64;

    cpu.arm.registers[rd_low] = ((r + rd_low as i64) & 0xFFFF_FFFF) as i32;
    cpu.arm.registers[rd_hi] = (cpu.arm.registers[rd_hi] + (r >> 32) as i32) as i32;
}

pub fn unsigned_multiply_accumulate(cpu: &mut CPU) {
    let (rd_hi, rd_low, rm, rs);
    match &cpu.decoded_instruction {
        InstructionType::Thumb(_) => {
            unimplemented!();
        }

        InstructionType::ARM(instr) => {
            if let Some(decoded) = &instr.decoded_instruction {
                rm = decoded.rm.unwrap() as usize;
                rs = decoded.rs.unwrap() as usize;
                rd_hi = decoded.rd.unwrap() as usize;
                rd_low = decoded.rn.unwrap() as usize;
            } else {
                eprintln!("Expected decoded instruction at multiply accumulate instruction");
            }
        }
    }

    let r = cpu.arm.registers[rm] as u64 * cpu.arm.registers[rs] as u64;

    cpu.arm.registers[rd_low] = ((r + rd_low as u64) & 0xFFFF_FFFF) as i32;
    cpu.arm.registers[rd_hi] = (cpu.arm.registers[rd_hi] as u32 + (r >> 32) as u32) as i32;
}

// End multiply micro operations
// -----------------------------
// Start Load/Store micro operations

// implement privilege modes

// End Load/Store micro operations
// -----------------------------
// Start ALU micro operations

// TODO (Alice Micheloni): set condition codes
pub fn alu_master(cpu: &mut CPU) {
    let (rn, rd);
    let op2: i32;
    let mnemonic: MnemonicARM;
    match &cpu.decoded_instruction {
        InstructionType::Thumb(_) => {
            unimplemented!();
        }

        InstructionType::ARM(instr) => {
            if let Some(decoded) = &instr.decoded_instruction {
                rn = cpu.arm.registers[decoded.rn.unwrap() as usize] as i32;
                rd = decoded.rn.unwrap() as usize;
                mnemonic = decoded.instr.clone();

                // implement the barrel shifter
                if decoded.imm.unwrap() {
                    // the immediate shift can only represent even numbers, so we multiply it by 2.
                    let shift = decoded.val1.unwrap() as u32 * 2;
                    let imm_value = decoded.val2.unwrap() as u32;
                    op2 = imm_value.rotate_right(shift) as i32;
                } else {
                    let rm = decoded.rm.unwrap();
                    let to_shift = cpu.arm.registers[rm as usize];

                    let shift_type = decoded.shift_type.unwrap();

                    let shift_amount;

                    // if rs is defined, we have to shift by register
                    if let Some(rs) = decoded.rs {
                        shift_amount = cpu.arm.registers[rs as usize];
                    } else {
                        shift_amount = decoded.val1.unwrap() as i32;
                    }

                    // TODO (Alice Micheloni): Handle Zero Shift Amount and PC.

                    /* shift types:
                     * 0 - LSL
                     * 1 - LSR
                     * 2 - ASR
                     * 3 - ROR
                     */
                    match shift_type {
                        0 => op2 = (to_shift) << shift_amount,
                        1 => op2 = (to_shift) >> shift_amount,
                        2 => op2 = to_shift >> shift_amount,
                        3 => op2 = (to_shift).rotate_right(shift_amount as u32),
                        _ => unreachable!(),
                    }
                }
            } else {
                eprintln!("Expected decoded instruction at multiply accumulate instruction");
            }
        }
    }

    match mnemonic {
        AND => cpu.arm.registers[rd] = rn & op2,
        EOR => cpu.arm.registers[rd] = rn ^ op2,
        ORR => cpu.arm.registers[rd] = rn | op2,
        BIC => cpu.arm.registers[rd] = rn & !op2,

        ADD => cpu.arm.registers[rd] = rn + op2,

        SUB => cpu.arm.registers[rd] = rn - op2,
        RSB => cpu.arm.registers[rd] = op2 - rn,

        MOV => cpu.arm.registers[rd] = op2,
        MVN => cpu.arm.registers[rd] = !op2,

        x => panic!("Unexpected instruction in ALU, {:?}", x),
    }
}

// End ALU micro operations
// -------------------------
// Start misc operations
