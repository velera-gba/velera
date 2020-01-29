use crate::{constants, constants::registers, cpu::CPU, enums::InstructionType};

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
                    cpu.arm.cpsr = (is_thumb << constants::cpsr_flags::STATE_BIT) as u32;
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

// End multiply micro operations
// -----------------------------
// Start Load/Store micro operations

// End Load/Store micro operations
// -----------------------------
// Start ALU micro operations
