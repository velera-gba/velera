use crate::constants::cond_arm;
use crate::constants::{default_cpu, registers};
use crate::cpu::CPU;
use crate::enums::MnemonicARM;

/// Implementation of the ARM cpu.
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

#[inline]
fn u82bool(v: u8) -> bool {
    if v == 1 {
        true
    } else {
        false
    }
}

struct Instruction {
    cond: u8,
    instr: MnemonicARM,
    rn: Option<u8>,   // index register
    rm: Option<u8>,   // second index register
    rd: Option<u8>,   // destination register
    rs: Option<u8>,   // source register
    val1: Option<u8>, // multi-purpose value (can be a shift to apply, etc)
    val2: Option<u8>, // ^
    offset: Option<u8>,

    set_cond: Option<bool>, // choose if should set condition codes
    imm: Option<bool>,      // whether the values come from registers or not
    acc: Option<bool>,      // whether the values should accumulate
}

fn data_processing(instruction: u32) -> Instruction {
    use crate::constants::dp_opcodes::*;

    let cond: u8 = instruction >> 28;
    let imm: bool = u82bool(instruction >> 20 & 0b0000_0000_0001);
    let opcode: u8 = instruction >> 21 & 0b000_0000_1111;
    let set_cond: bool = u8tobool(instruction >> 20 & 0b0000_0000_0001);
    let rn: u8 = instruction >> 16 & 0b0000_0000_0000_1111;
    let rd: u8 = instruction >> 12 & 0b0000_0000_0000_0000_1111;

    // is it possible to change this to a stringify! statement?
    let opcode = match opcode {
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
    };

    if imm {
        let val1 = instruction >> 8 & 0b0000_0000_0000_0000_0000_1111; // shift applied to imm
        let rm = instruction << 32 & 0b0000_0000_0000_0000_0000_0000_1111_1111;
        Instruction {
            cond,
            opcode,
            rn,
            rm,
            rd,
            val1,
            val2: None,
            rs: None,
            offset: None,

            imm,
            set_cond,
        }
    }

    let val1 = instruction >> 4 & 0b0000_0000_0000_0000_0000_1111_1111;
    let rm = instruction & 0b0000_0000_0000_0000_0000_0000_0000_1111;

    return Instruction {
        cond,
        opcode,
        rn,
        rm,
        rd,
        val1,
        val2: None,
        rs: None,
        offset: None,

        imm,
        set_cond,
    };
}

/// Reads multiply/mul long/mul half statements.
fn multiply(instruction: u32) {
    let cond: u8 = instruction >> 28;
    let rd: u8 = instruction >> 16 & 0b0000_0000_0000_1111;
    let rn: u8 = instruction >> 12 & 0b0000_0000_0000_0000_1111;
}

/// Used for decoding ARM instructions.
pub fn decode_arm(cpu: &mut CPU, instruction: u32) {
    match instruction {
        _ => eprintln!(cpu.arm.registers[registers::PROGRAM_COUNTER as usize]),
    }
}

/// Executes the instructions.
pub fn execute_arm(_cpu: &mut CPU, _instruction: u32) {
    unimplemented!();
}
