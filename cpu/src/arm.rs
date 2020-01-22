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

/// Finds out which instruction the numbers represent and separates its values
#[inline]
pub fn decode_arm(_: &mut CPU, instruction: u32) -> decode::DecodedInstruction {
    decode::BaseInstruction::base_to_decoded(instruction)
}

pub fn execute_arm(_cpu: &mut CPU, instruction: u32) {
    let _cond = (instruction >> 28) as u8;
    unimplemented!();
}

pub mod tests;
