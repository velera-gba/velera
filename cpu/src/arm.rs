use crate::{constants::default_cpu, cpu::CPU, enums::MnemonicARM};

use std::{
    collections::VecDeque,
    default::Default,
};

mod decode;

/// Holds a temporary instruction to be executed
#[derive(Debug, Default, PartialEq, Clone)]
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

#[derive(Clone)]
pub struct ARMInstruction {
    pub fetched_instruction: Option<u32>,
    pub decoded_instruction: Option<DecodedInstruction>,
}

impl ARMInstruction {
    pub fn new_decoded(decoded_instr: DecodedInstruction) -> Self {
        Self {
            fetched_instruction: None,
            decoded_instruction: Some(decoded_instr),
        }
    }

    pub fn new_fetched(fetched_instr: u32) -> Self {
        Self {
            fetched_instruction: Some(fetched_instr),
            decoded_instruction: None,
        }
    }
}

/// Handles ARM decoding and execution.
pub struct ARM7TDMI {
    pub registers: [i32; 16],
    pub cpsr: u32,
    pub spsr: u32,
    pub temp_rd: i32,   // temporary destination register
    pub temp_rs: i32,   // temporary source register
    pub temp_rn: i32,   // temporary index register
    pub immediate: i32, // temporary immediate
}

impl Default for ARM7TDMI {
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
pub fn decode_arm(_: &mut CPU, instruction: u32) -> VecDeque<fn(&mut CPU)> {
    let _decoded = decode::BaseInstruction::base_to_decoded(instruction);
    // digest decoded into a series of single-cycle instructions...
    unimplemented!();
}

pub mod tests;
