use crate::{constants::default_cpu, cpu::CPU};

use crate::enums::{InstructionType, MnemonicARM, ProcessorMode, ShiftType};
use std::{collections::VecDeque, default::Default};

mod decode;

pub struct ARM7TDMI {
    registers: [i32; 16],
    fiq_banked_registers: [i32; 7],
    irq_banked_registers: [i32; 2],
    supervisor_banked_registers: [i32; 2],
    abort_banked_registers: [i32; 2],
    undefined_banked_registers: [i32; 2],
    pub cpsr: u32,
    pub spsr: u32,
    pub shifter_carry: u32, // last bit shifted out in execution
    pub mode: ProcessorMode,
}

impl ARM7TDMI {
    /// Wraps the ARM registers to apply privilege modes when reading.
    pub fn load_register(&mut self, r: usize) -> i32 {
        match self.mode {
            ProcessorMode::User | ProcessorMode::System => self.registers[r],
            ProcessorMode::FIQ => {
                if r >= 8 && r < 15 {
                    self.fiq_banked_registers[r - 8]
                } else {
                    self.registers[r]
                }
            }
            ProcessorMode::Supervisor => {
                if r >= 13 && r < 15 {
                    self.supervisor_banked_registers[r - 13]
                } else {
                    self.registers[r]
                }
            }
            ProcessorMode::IRQ => {
                if r >= 13 && r < 15 {
                    self.irq_banked_registers[r - 13]
                } else {
                    self.registers[r]
                }
            }
            ProcessorMode::Abort => {
                if r >= 13 && r < 15 {
                    self.abort_banked_registers[r - 13]
                } else {
                    self.registers[r]
                }
            }
            ProcessorMode::Undefined => {
                if r >= 13 && r < 15 {
                    self.undefined_banked_registers[r - 13]
                } else {
                    self.registers[r]
                }
            }
        }
    }

    pub fn write_register(&mut self, r: usize, v: i32) {
        match self.mode {
            ProcessorMode::User | ProcessorMode::System => self.registers[r] = v,
            ProcessorMode::FIQ => {
                if r >= 8 && r < 15 {
                    self.fiq_banked_registers[r - 8] = v;
                } else {
                    self.registers[r] = v;
                }
            }
            ProcessorMode::Supervisor => {
                if r >= 13 && r < 15 {
                    self.supervisor_banked_registers[r - 13] = v;
                } else {
                    self.registers[r] = v;
                }
            }
            ProcessorMode::IRQ => {
                if r >= 13 && r < 15 {
                    self.irq_banked_registers[r - 13] = v;
                } else {
                    self.registers[r] = v;
                }
            }
            ProcessorMode::Abort => {
                if r >= 13 && r < 15 {
                    self.abort_banked_registers[r - 13] = v;
                } else {
                    self.registers[r] = v;
                }
            }
            ProcessorMode::Undefined => {
                if r >= 13 && r < 15 {
                    self.undefined_banked_registers[r - 13] = v;
                } else {
                    self.registers[r] = v;
                }
            }
        }
    }
}

impl Default for ARM7TDMI {
    fn default() -> Self {
        Self {
            registers: default_cpu::REGISTERS,
            fiq_banked_registers: default_cpu::FIQ_REGISTERS,
            irq_banked_registers: default_cpu::BANKED_REGISTERS,
            supervisor_banked_registers: default_cpu::BANKED_REGISTERS,
            abort_banked_registers: default_cpu::BANKED_REGISTERS,
            undefined_banked_registers: default_cpu::BANKED_REGISTERS,
            cpsr: default_cpu::CPSR,
            spsr: default_cpu::SPSR,
            shifter_carry: 0,
            mode: ProcessorMode::User,
        }
    }
}

/// Holds a temporary instruction to be executed
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DecodedInstruction {
    pub cond: u8,
    pub instr: MnemonicARM,

    pub rn: Option<u8>, // index register
    pub rm: Option<u8>, // second index register
    pub rd: Option<u8>, // destination register
    pub rs: Option<u8>, // source register

    pub val1: Option<u8>, // multi-purpose value (can be a shift to apply, etc)
    pub val2: Option<u8>, // ^
    pub val3: Option<u8>,

    pub offset: Option<i32>, // offset for branching

    pub shift_type: Option<ShiftType>, // 0=LSL, 1=LSR, 2=ASR, 3=ROR
    pub set_cond: Option<bool>,        // choose if should set condition codes
    pub imm: Option<bool>,             // whether the values come from registers or not
    pub acc: Option<bool>,             // whether the values should accumulate
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
/// Finds out which instruction the numbers represent and separates its values
pub fn decode_arm(cpu: &mut CPU, instruction: u32) -> VecDeque<fn(&mut CPU)> {
    let decoded = decode::BaseInstruction::base_to_decoded(instruction);
    cpu.decoded_instruction = InstructionType::ARM(ARMInstruction::new_decoded(decoded));
    // digest decoded into a series of single-cycle instructions...
    unimplemented!();
}

pub mod tests;
