use crate::constants::{cond_arm, registers, thumb_bitmasks};
use crate::cpu::CPU;
use crate::micro_ops::*;
use std::collections::VecDeque;

// WARNING!
// This file contains lots of code that could be in the CPU impl, but in order to decrease cacheing of
// CPU (and consequently increasing speed when executing a non-cached path), they are separate functions.

/// Thumb bitmasks for decoding.
struct ThumbOpPack {
    op_bitmask: u16,
    opcode_bitmask: u16,
}

/// Decodes already-fetched thumb instruction.
pub fn decode_thumb(cpu: &mut CPU, instruction: u16) -> VecDeque<fn(&mut CPU)> {
    let mut operation: bool = false;
    let mut queue: VecDeque<fn(&mut CPU)> = VecDeque::new();

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::MOVE_SHIFTED_REG_OFFSET_MASK,
        thumb_bitmasks::LSR => {
            dummy_cycle
        },
        thumb_bitmasks::LSL => {
            dummy_cycle
        },
        thumb_bitmasks::ASR => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::ADDSUB_OP_MASK,
        thumb_bitmasks::ADD => {
            dummy_cycle
        },
        thumb_bitmasks::SUB => {
            dummy_cycle
        },
        thumb_bitmasks::ADDI => {
            dummy_cycle
        },
        thumb_bitmasks::SUBI => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::IMMEDIATE_OP_MASK,
        thumb_bitmasks::MOV => {
            dummy_cycle
        },
        thumb_bitmasks::CMP => {
            dummy_cycle
        },
        thumb_bitmasks::ADDRI => {
            dummy_cycle
        },
        thumb_bitmasks::SUBRI => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::ALU_OP_MASK,
        thumb_bitmasks::ALU_AND => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_EOR => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_LSL => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_LSR => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_ASR => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_ADC => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_SBC => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_ROR => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_TST => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_NEG => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_CMP => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_CMN => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_ORR => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_MUL => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_BIC => {
            dummy_cycle
        },
        thumb_bitmasks::ALU_MVN => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::HI_OP_MASK,
        thumb_bitmasks::HI_ADD => {
            dummy_cycle
        },
        thumb_bitmasks::HI_CMP => {
            dummy_cycle
        },
        thumb_bitmasks::HI_MOV => {
            dummy_cycle
        },
        thumb_bitmasks::HI_NOP => {
            dummy_cycle
        },
        thumb_bitmasks::BX => {
            dummy_cycle
        },
        thumb_bitmasks::BLX => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LDPCR_MASK,
        thumb_bitmasks::LDPCR => {
            dummy_cycle
        }
    );

    // here (and in other places) I use the source temporary register as the base register
    // and the index register as the offset register
    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LS_REG_OFFSET_OPCODE_MASK,
        thumb_bitmasks::STR => {
            dummy_cycle
        },
        thumb_bitmasks::STRB => {
            dummy_cycle
        },
        thumb_bitmasks::LDR => {
            dummy_cycle
        },
        thumb_bitmasks::LDRB => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LS_EBH_OP_MASK,
        thumb_bitmasks::STRH => {
            dummy_cycle
        },
        thumb_bitmasks::LDSB => {
            dummy_cycle
        },
        thumb_bitmasks::LDRH => {
            dummy_cycle
        },
        thumb_bitmasks::LDSH => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LS_NN_OFFSET_OP_MASK,
        thumb_bitmasks::STRI => {
            dummy_cycle
        },
        thumb_bitmasks::LDRI => {
            dummy_cycle
        },
        thumb_bitmasks::STRBI => {
            dummy_cycle
        },
        thumb_bitmasks::LDRBI => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LS_HW_OP_MASK,
        thumb_bitmasks::STRHW => {
            dummy_cycle
        },
        thumb_bitmasks::LDRHW => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::SP_LS_OP_MASK,
        thumb_bitmasks::SP_STR => {
            dummy_cycle
        },
        thumb_bitmasks::SP_LDR => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::RELATIVE_ADDR_OP_MASK,
        thumb_bitmasks::ADD_PC => {
            dummy_cycle
        },
        thumb_bitmasks::ADD_SP => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::SP_OFFSET_OP_MASK,
        thumb_bitmasks::ADD_SP_NN => {
            dummy_cycle
        },
        thumb_bitmasks::ADD_SP_MINUS_NN => {
            dummy_cycle
        }
    );

    // the immediate here is actually the PC/LR bit
    // and the index register is contains bits to each one of the general purpose registers
    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::STACK_OPS_OP_MASK,
        thumb_bitmasks::PUSH => {
            dummy_cycle
        },
        thumb_bitmasks::POP => {
            dummy_cycle
        }
    );

    // same thing here
    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LS_MIA_OP_MASK,
        thumb_bitmasks::STMIA => {
            dummy_cycle
        },
        thumb_bitmasks::LDMIA => {
            dummy_cycle
        }
    );

    // the conditional branch is an interesting case, I associated the operation bits with the opcode bitmasks
    // so I avoided writing too much code
    let extra_opcode_mask =
        thumb_bitmasks::COND_GENERAL_OP_MASK ^ thumb_bitmasks::COND_FULL_OP_MASK;

    let cond_branch_shift = 8;
    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::COND_FULL_OP_MASK,
        ((cond_arm::EQ as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::NE as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::CS as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::CC as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::MI as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::PL as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::VS as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::VC as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::HI as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::LS as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::GE as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::LT as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::GT as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        },
        ((cond_arm::LE as u16) << cond_branch_shift) & extra_opcode_mask => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::SWI_BK_OP_MASK,
        thumb_bitmasks::SWI => {
            dummy_cycle
        },
        thumb_bitmasks::BKPT => {
            dummy_cycle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::B_OP_MASK,
        thumb_bitmasks::B => {
            dummy_cycle
        }
    );

    // I should change this in the future, because this instruction is actually 32 bits with a long branch
    // but I should find a way to make this work as it is
    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LONG_BRANCH_OP_MASK,
        thumb_bitmasks::LONG_BRANCH_FIRST_OP => {
            dummy_cycle
        },
        thumb_bitmasks::BL => {
            dummy_cycle
        },
        thumb_bitmasks::BLLX => {
            dummy_cycle
        }
    );

    // operation not found error check
    if !operation {
        // to do: switch to ARM state, deal with exception
        eprintln!(
            "{:#x}: undefinded THUMB instruction exception.",
            cpu.arm.load_register(registers::PROGRAM_COUNTER as usize)
        );

        return VecDeque::new();
    } else {
        eprintln!(
            "{:#x}: unknown error in decode.",
            cpu.arm.load_register(registers::PROGRAM_COUNTER as usize)
        );
        return VecDeque::new();
    }
}

fn pass_operation_thumb(instruction: u16, operation: &mut bool, pack: ThumbOpPack) -> bool {
    if pack.op_bitmask ^ (instruction & pack.opcode_bitmask) == 0 {
        *operation = true;
        return true;
    }
    return false;
}
/*
fn put_temp_register_thumb(register: &mut i32, register_bitmask: u16, instruction: u16) {
    let mut bitmask_eval = register_bitmask;
    let mut shift_modifier = 0;

    while bitmask_eval % 2 == 0 {
        bitmask_eval >>= 1;
        shift_modifier += 1;
    }
    *register = ((register_bitmask & instruction) >> shift_modifier) as i32;
}
*/
// TESTS //

pub mod tests;
