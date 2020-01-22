use crate::constants::{cond_arm, registers, thumb_bitmasks};
use crate::cpu::CPU;
//use crate::micro_ops::*;

// WARNING!
// This file contains lots of code that could be in the CPU impl, but in order to decrease cacheing of
// CPU (and consequently increasing speed when executing a non-cached path), they are separate functions.

/// Thumb bitmasks for decoding.
struct ThumbOpPack {
    op_bitmask: u16,
    opcode_bitmask: u16,
    rd_bitmask: u16,
    rs_bitmask: u16,
    rn_bitmask: u16,
    immediate_bitmask: u16,
}

/// Decodes already-fetched thumb instruction.
pub fn decode_thumb(cpu: &mut CPU, instruction: u16) {
    let mut operation: bool = false;

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::MOVE_SHIFTED_REG_OP_MASK,
        thumb_bitmasks::MOVE_SHIFTED_REG_RD_MASK,
        thumb_bitmasks::MOVE_SHIFTED_REG_RS_MASK,
        0,
        thumb_bitmasks::MOVE_SHIFTED_REG_OFFSET_MASK,
        thumb_bitmasks::LSR => {
        },
        thumb_bitmasks::LSL => {
        },
        thumb_bitmasks::ASR => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::ADDSUB_OP_MASK,
        thumb_bitmasks::ADDSUB_RD_MASK,
        thumb_bitmasks::ADDSUB_RS_MASK,
        thumb_bitmasks::ADDSUB_RN_MASK,
        0,
        thumb_bitmasks::ADD => {
        },
        thumb_bitmasks::SUB => {
        },
        thumb_bitmasks::ADDI => {
        },
        thumb_bitmasks::SUBI => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::IMMEDIATE_OP_MASK,
        thumb_bitmasks::IMMEDIATE_RD_MASK,
        0,
        0,
        thumb_bitmasks::IMMEDIATE_NN_MASK,
        thumb_bitmasks::MOV => {
        },
        thumb_bitmasks::CMP => {
        },
        thumb_bitmasks::ADDRI => {
        },
        thumb_bitmasks::SUBRI => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::ALU_OP_MASK,
        thumb_bitmasks::ALU_RD_MASK,
        thumb_bitmasks::ALU_RS_MASK,
        0,
        0,
        thumb_bitmasks::ALU_AND => {
        },
        thumb_bitmasks::ALU_EOR => {
        },
        thumb_bitmasks::ALU_LSL => {
        },
        thumb_bitmasks::ALU_LSR => {
        },
        thumb_bitmasks::ALU_ASR => {
        },
        thumb_bitmasks::ALU_ADC => {
        },
        thumb_bitmasks::ALU_SBC => {
        },
        thumb_bitmasks::ALU_ROR => {
        },
        thumb_bitmasks::ALU_TST => {
        },
        thumb_bitmasks::ALU_NEG => {
        },
        thumb_bitmasks::ALU_CMP => {
        },
        thumb_bitmasks::ALU_CMN => {
        },
        thumb_bitmasks::ALU_ORR => {
        },
        thumb_bitmasks::ALU_MUL => {
        },
        thumb_bitmasks::ALU_BIC => {
        },
        thumb_bitmasks::ALU_MVN => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::HI_OP_MASK,
        thumb_bitmasks::HI_RD,
        thumb_bitmasks::HI_RS,
        thumb_bitmasks::HI_MSBD_MASK,
        thumb_bitmasks::HI_MSBS_MASK,
        thumb_bitmasks::HI_ADD => {
        },
        thumb_bitmasks::HI_CMP => {
        },
        thumb_bitmasks::HI_MOV => {
        },
        thumb_bitmasks::HI_NOP => {
        },
        thumb_bitmasks::BX => {
        },
        thumb_bitmasks::BLX => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::LDPCR_MASK,
        thumb_bitmasks::LDPCR_RD,
        0,
        0,
        thumb_bitmasks::LDPCR_OFFSET,
        thumb_bitmasks::LDPCR => {
        }
    );

    // here (and in other places) I use the source temporary register as the base register
    // and the index register as the offset register
    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::LS_REG_OFFSET_OPCODE_MASK,
        thumb_bitmasks::LS_REG_OFFSET_RD_MASK,
        thumb_bitmasks::LS_REG_OFFSET_RB_MASK,
        thumb_bitmasks::LS_REG_OFFSET_RO_MASK,
        0,
        thumb_bitmasks::STR => {
        },
        thumb_bitmasks::STRB => {
        },
        thumb_bitmasks::LDR => {
        },
        thumb_bitmasks::LDRB => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::LS_EBH_OP_MASK,
        thumb_bitmasks::LS_EBH_RD_MASK,
        thumb_bitmasks::LS_EBH_RB_MASK,
        thumb_bitmasks::LS_EBH_RO_MASK,
        0,
        thumb_bitmasks::STRH => {
        },
        thumb_bitmasks::LDSB => {
        },
        thumb_bitmasks::LDRH => {
        },
        thumb_bitmasks::LDSH => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::LS_NN_OFFSET_OP_MASK,
        thumb_bitmasks::LS_NN_OFFSET_RD_MASK,
        thumb_bitmasks::LS_NN_OFFSET_RB_MASK,
        0,
        thumb_bitmasks::LS_NN_OFFSET_NN_MASK,
        thumb_bitmasks::STRI => {
        },
        thumb_bitmasks::LDRI => {
        },
        thumb_bitmasks::STRBI => {
        },
        thumb_bitmasks::LDRBI => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::LS_HW_OP_MASK,
        thumb_bitmasks::LS_HW_RD_MASK,
        thumb_bitmasks::LS_HW_RB_MASK,
        0,
        thumb_bitmasks::LS_HW_NN_MASK,
        thumb_bitmasks::STRHW => {
        },
        thumb_bitmasks::LDRHW => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::SP_LS_OP_MASK,
        thumb_bitmasks::SP_LS_RD_MASK,
        0,
        0,
        thumb_bitmasks::SP_LS_NN_MASK,
        thumb_bitmasks::SP_STR => {
        },
        thumb_bitmasks::SP_LDR => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::RELATIVE_ADDR_OP_MASK,
        thumb_bitmasks::RELATIVE_ADDR_RD_MASK,
        0,
        0,
        thumb_bitmasks::RELATIVE_ADDR_NN_MASK,
        thumb_bitmasks::ADD_PC => {
        },
        thumb_bitmasks::ADD_SP => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::SP_OFFSET_OP_MASK,
        0,
        0,
        0,
        thumb_bitmasks::SP_OFFSET_NN_MASK,
        thumb_bitmasks::ADD_SP_NN => {
        },
        thumb_bitmasks::ADD_SP_MINUS_NN => {
        }
    );

    // the immediate here is actually the PC/LR bit
    // and the index register is contains bits to each one of the general purpose registers
    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::STACK_OPS_OP_MASK,
        0,
        0,
        thumb_bitmasks::STACK_OPS_RLIST_MASK,
        thumb_bitmasks::STACK_OPS_PC_LR_BIT_MASK,
        thumb_bitmasks::PUSH => {
        },
        thumb_bitmasks::POP => {
        }
    );

    // same thing here
    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::LS_MIA_OP_MASK,
        0,
        thumb_bitmasks::LS_MIA_RB_MASK,
        thumb_bitmasks::LS_MIA_RLIST_MASK,
        0,
        thumb_bitmasks::STMIA => {
        },
        thumb_bitmasks::LDMIA => {
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
        thumb_bitmasks::COND_FULL_OP_MASK,
        0,
        0,
        0,
        thumb_bitmasks::COND_OFFSET_MASK,
        ((cond_arm::EQ as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::NE as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::CS as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::CC as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::MI as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::PL as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::VS as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::VC as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::HI as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::LS as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::GE as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::LT as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::GT as u16) << cond_branch_shift) & extra_opcode_mask => {
        },
        ((cond_arm::LE as u16) << cond_branch_shift) & extra_opcode_mask => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::SWI_BK_OP_MASK,
        0,
        0,
        0,
        thumb_bitmasks::SWI_BK_NN_MASK,
        thumb_bitmasks::SWI => {
        },
        thumb_bitmasks::BKPT => {
        }
    );

    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::B_OP_MASK,
        0,
        0,
        0,
        thumb_bitmasks::B_OFFSET_MASK,
        thumb_bitmasks::B => {
        }
    );

    // I should change this in the future, because this instruction is actually 32 bits with a long branch
    // but I should find a way to make this work as it is
    temp_reg_wrap!(
        cpu,
        instruction,
        &mut operation,
        thumb_bitmasks::LONG_BRANCH_OP_MASK,
        0,
        0,
        0,
        thumb_bitmasks::LONG_BRANCH_ADDR_MASK,
        thumb_bitmasks::LONG_BRANCH_FIRST_OP => {
        },
        thumb_bitmasks::BL => {
        },
        thumb_bitmasks::BLLX => {
        }
    );

    // operation not found error check
    if !operation {
        // to do: switch to ARM state, deal with exception
        println!(
            "{:#x}: undefinded THUMB instruction exception.",
            cpu.arm.registers[registers::PROGRAM_COUNTER as usize]
        );
    }
}

fn pass_operation_thumb(cpu: &mut CPU, instruction: u16, operation: &mut bool, pack: ThumbOpPack) {
    if pack.op_bitmask ^ (instruction & pack.opcode_bitmask) == 0 {
        *operation = true;
        if pack.rd_bitmask != 0 {
            put_temp_register_thumb(&mut cpu.arm.temp_rd, pack.rd_bitmask, instruction);
        }
        if pack.rs_bitmask != 0 {
            put_temp_register_thumb(&mut cpu.arm.temp_rs, pack.rs_bitmask, instruction);
        }
        if pack.rn_bitmask != 0 {
            put_temp_register_thumb(&mut cpu.arm.temp_rn, pack.rn_bitmask, instruction);
        }
        if pack.immediate_bitmask != 0 {
            put_temp_register_thumb(&mut cpu.arm.immediate, pack.immediate_bitmask, instruction);
        }
    }
}

fn put_temp_register_thumb(register: &mut i32, register_bitmask: u16, instruction: u16) {
    let mut bitmask_eval = register_bitmask;
    let mut shift_modifier = 0;

    while bitmask_eval % 2 == 0 {
        bitmask_eval >>= 1;
        shift_modifier += 1;
    }
    *register = ((register_bitmask & instruction) >> shift_modifier) as i32;
}

/// Execute thumb code.
pub fn execute_thumb(_cpu: &mut CPU) {

}

// TESTS //

pub mod tests;
