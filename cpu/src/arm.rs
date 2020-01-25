use crate::constants::{default_cpu, registers};
use crate::cpu::CPU;
use crate::enums::MnemonicARM;
use std::collections::VecDeque;

use std::default::Default;

/// Implementation of the ARM cpu.
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
fn get_last_bits(input: u32, n: u8) -> u32 {
    if n < 32 {
        return input & ((1 << n) - 1);
    }
    panic!("tried to get >32 last bits");
}

#[derive(Debug, Default)]
struct Instruction {
    cond: u8,
    instr: MnemonicARM,
    rn: Option<u8>,      // index register
    rm: Option<u8>,      // second index register
    rd: Option<u8>,      // destination register
    rs: Option<u8>,      // source register
    val1: Option<u8>,    // multi-purpose value (can be a shift to apply, etc)
    val2: Option<u8>,    // ^
    offset: Option<u32>, // offset for branching

    set_cond: Option<bool>, // choose if should set condition codes
    imm: Option<bool>,      // whether the values come from registers or not
    acc: Option<bool>,      // whether the values should accumulate
}

fn data_processing(instruction: u32) -> Instruction {
    use crate::constants::dp_opcodes::*;

    let cond = (instruction >> 28) as u8;
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
        return Instruction {
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
    Instruction {
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
fn branch_exchange(instruction: u32) -> Instruction {
    let cond = (instruction >> 28) as u8;
    let rn = Some(get_last_bits(instruction, 4) as u8);

    Instruction {
        cond,
        rn,
        instr: MnemonicARM::BX,
        ..Default::default()
    }
}

/// decodes B, BL instructions.
fn branch(instruction: u32) -> Instruction {
    let cond = (instruction >> 28) as u8;
    let val1 = Some((instruction >> 24 & 1) as u8); // to link or not to link, that is the question...
    let offset = Some(get_last_bits(instruction, 24));

    Instruction {
        cond,
        val1,
        offset,
        ..Default::default()
    }
}

/// Reads multiply/mul long/mul half statements.
fn multiply(instruction: u32) -> Instruction {
    let cond = (instruction >> 28) as u8;
    let rd = Some(get_last_bits(instruction >> 16, 4) as u8);
    let rn = Some(get_last_bits(instruction >> 12, 4) as u8);
    let rs = Some(get_last_bits(instruction >> 8, 4) as u8);
    let rm = Some(get_last_bits(instruction, 4) as u8);

    let acc = get_bit_at(instruction, 21);
    let set_cond = Some(get_bit_at(instruction, 20));

    let instr = if acc {
        MnemonicARM::MLA
    } else {
        MnemonicARM::MUL
    };

    Instruction {
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

/// Used for decoding ARM instructions.
pub fn decode_arm(cpu: &mut CPU, instruction: u32) -> VecDeque<fn(&mut CPU)> {
    match instruction {
        _ => {
            eprintln!(
            "unknown instruction: {}",
            cpu.arm.registers[registers::PROGRAM_COUNTER as usize]);
            return VecDeque::new();
        }
    }
}

/// Executes the instructions.
pub fn execute_arm(_cpu: &mut CPU, _instruction: u32) {
    unimplemented!();
}
