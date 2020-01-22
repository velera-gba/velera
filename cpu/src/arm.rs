use crate::{constants::default_cpu, cpu::CPU};

use std::default::Default;

mod decode;

/// Handles ARM decoding and execution.
pub struct ARM7HDTI {
    pub registers: [i32; 16],
    pub cpsr: u32,
    pub spsr: u32,
    pub temp_rd: i32,   // temporary destination register
    pub temp_rs: i32,   // temporary source register
    pub temp_rn: i32,   // temporary index register
    pub immediate: i32, // temporary immediate
}

impl Default for ARM7HDTI {
    fn default() -> Self {
        Self {
            registers: default_cpu::RS,
            cpsr: default_cpu::CPSR,
            spsr: default_cpu::SPSR,
            temp_rd: 0,
            temp_rs: 0,
            temp_rn: 0,
            immediate: 0,
        }
    }
}

/// Used for decoding ARM instructions.
pub fn decode_arm(_: &mut CPU, instruction: u32) -> decode::DecodedInstruction {
    use crate::arm::decode::BaseInstruction;

    let cond = (instruction >> 28) as u8;
    let instruction = decode::get_last_bits(instruction, 28);

    let instr = BaseInstruction::base_to_decoded(
        BaseInstruction::get_instr(instruction),
        instruction,
        cond,
    );

    instr
}

/// Executes the instructions.
pub fn execute_arm(_cpu: &mut CPU, instruction: u32) {
    let _cond = (instruction >> 28) as u8;
    unimplemented!();
}

pub mod tests;
