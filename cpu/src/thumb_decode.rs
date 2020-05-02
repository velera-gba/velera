use crate::constants::{cond_arm, registers, thumb_bitmasks};
use crate::cpu::CPU;
use crate::instructionset::idle;
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
            idle
        },
        thumb_bitmasks::LSL => {
            idle
        },
        thumb_bitmasks::ASR => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::ADDSUB_OP_MASK,
        thumb_bitmasks::ADD => {
            idle
        },
        thumb_bitmasks::SUB => {
            idle
        },
        thumb_bitmasks::ADDI => {
            idle
        },
        thumb_bitmasks::SUBI => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::IMMEDIATE_OP_MASK,
        thumb_bitmasks::MOV => {
            idle
        },
        thumb_bitmasks::CMP => {
            idle
        },
        thumb_bitmasks::ADDRI => {
            idle
        },
        thumb_bitmasks::SUBRI => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::ALU_OP_MASK,
        thumb_bitmasks::ALU_AND => {
            idle
        },
        thumb_bitmasks::ALU_EOR => {
            idle
        },
        thumb_bitmasks::ALU_LSL => {
            idle
        },
        thumb_bitmasks::ALU_LSR => {
            idle
        },
        thumb_bitmasks::ALU_ASR => {
            idle
        },
        thumb_bitmasks::ALU_ADC => {
            idle
        },
        thumb_bitmasks::ALU_SBC => {
            idle
        },
        thumb_bitmasks::ALU_ROR => {
            idle
        },
        thumb_bitmasks::ALU_TST => {
            idle
        },
        thumb_bitmasks::ALU_NEG => {
            idle
        },
        thumb_bitmasks::ALU_CMP => {
            idle
        },
        thumb_bitmasks::ALU_CMN => {
            idle
        },
        thumb_bitmasks::ALU_ORR => {
            idle
        },
        thumb_bitmasks::ALU_MUL => {
            idle
        },
        thumb_bitmasks::ALU_BIC => {
            idle
        },
        thumb_bitmasks::ALU_MVN => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::HI_OP_MASK,
        thumb_bitmasks::HI_ADD => {
            idle
        },
        thumb_bitmasks::HI_CMP => {
            idle
        },
        thumb_bitmasks::HI_MOV => {
            idle
        },
        thumb_bitmasks::HI_NOP => {
            idle
        },
        thumb_bitmasks::BX => {
            idle
        },
        thumb_bitmasks::BLX => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LDPCR_MASK,
        thumb_bitmasks::LDPCR => {
            idle
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
            idle
        },
        thumb_bitmasks::STRB => {
            idle
        },
        thumb_bitmasks::LDR => {
            idle
        },
        thumb_bitmasks::LDRB => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LS_EBH_OP_MASK,
        thumb_bitmasks::STRH => {
            idle
        },
        thumb_bitmasks::LDSB => {
            idle
        },
        thumb_bitmasks::LDRH => {
            idle
        },
        thumb_bitmasks::LDSH => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LS_NN_OFFSET_OP_MASK,
        thumb_bitmasks::STRI => {
            idle
        },
        thumb_bitmasks::LDRI => {
            idle
        },
        thumb_bitmasks::STRBI => {
            idle
        },
        thumb_bitmasks::LDRBI => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::LS_HW_OP_MASK,
        thumb_bitmasks::STRHW => {
            idle
        },
        thumb_bitmasks::LDRHW => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::SP_LS_OP_MASK,
        thumb_bitmasks::SP_STR => {
            idle
        },
        thumb_bitmasks::SP_LDR => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::RELATIVE_ADDR_OP_MASK,
        thumb_bitmasks::ADD_PC => {
            idle
        },
        thumb_bitmasks::ADD_SP => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::SP_OFFSET_OP_MASK,
        thumb_bitmasks::ADD_SP_NN => {
            idle
        },
        thumb_bitmasks::ADD_SP_MINUS_NN => {
            idle
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
            idle
        },
        thumb_bitmasks::POP => {
            idle
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
            idle
        },
        thumb_bitmasks::LDMIA => {
            idle
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
            idle
        },
        ((cond_arm::NE as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::CS as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::CC as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::MI as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::PL as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::VS as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::VC as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::HI as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::LS as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::GE as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::LT as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::GT as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        },
        ((cond_arm::LE as u16) << cond_branch_shift) & extra_opcode_mask => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::SWI_BK_OP_MASK,
        thumb_bitmasks::SWI => {
            idle
        },
        thumb_bitmasks::BKPT => {
            idle
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        queue,
        thumb_bitmasks::B_OP_MASK,
        thumb_bitmasks::B => {
            idle
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
            idle
        },
        thumb_bitmasks::BL => {
            idle
        },
        thumb_bitmasks::BLLX => {
            idle
        }
    );

    // operation not found error check
    if !operation {
        // to do: switch to ARM state, deal with exception
        eprintln!(
            "{:#x}: undefinded THUMB instruction exception.",
            cpu.arm.load_register(registers::PROGRAM_COUNTER as usize)
        );

        VecDeque::new()
    } else {
        eprintln!(
            "{:#x}: unknown error in decode.",
            cpu.arm.load_register(registers::PROGRAM_COUNTER as usize)
        );
        VecDeque::new()
    }
}

fn pass_operation_thumb(instruction: u16, operation: &mut bool, pack: ThumbOpPack) -> bool {
    if pack.op_bitmask ^ (instruction & pack.opcode_bitmask) == 0 {
        *operation = true;
        return true;
    }
    false
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
