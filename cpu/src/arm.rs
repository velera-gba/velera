use crate::{constants::default_cpu, cpu::CPU};

use crate::enums::{InstructionType, MnemonicARM, ProcessorMode, ShiftType};
use std::{collections::VecDeque, default::Default};

mod decode;

#[derive(Clone)]
pub struct ARM7TDMI {
    registers: [i32; 16],
    fiq_banked_registers: [i32; 7],
    irq_banked_registers: [i32; 2],
    supervisor_banked_registers: [i32; 2],
    abort_banked_registers: [i32; 2],
    undefined_banked_registers: [i32; 2],

    pub cpsr: PSR,
    pub spsr_fiq: PSR,
    pub spsr_irq: PSR,
    pub spsr_svc: PSR,
    pub spsr_abt: PSR,
    pub spsr_und: PSR,

    pub shifter_carry: u32, // last bit shifted out in execution
}

#[derive(Clone)]
pub struct PSR {
    pub negative: bool,
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
    pub thumb_mode: bool,
    pub disable_irq: bool,
    pub disable_fiq: bool,

    pub mode: ProcessorMode,
}

impl Default for PSR {
    fn default() -> Self {
        Self {
            negative: false,
            zero: false,
            carry: false,
            overflow: false,
            thumb_mode: false,
            disable_irq: true,
            disable_fiq: true,
            mode: ProcessorMode::User,
        }
    }
}

impl PSR {
    pub fn unpack(&self) -> i32 {
        let mode = match self.mode {
            ProcessorMode::User => 0b1_0000,
            ProcessorMode::System => 0b1_1111,
            ProcessorMode::FIQ => 0b1_0001,
            ProcessorMode::IRQ => 0b1_0010,
            ProcessorMode::Supervisor => 0b1_0011,
            ProcessorMode::Abort => 0b1_0111,
            ProcessorMode::Undefined => 0b1_1011,
        };

        let neg = (self.negative as i32) << 31;
        let zero = (self.zero as i32) << 30;
        let carry = (self.carry as i32) << 29;
        let overflow = (self.overflow as i32) << 28;
        let irq = (self.disable_irq as i32) << 7;
        let fiq = (self.disable_fiq as i32) << 6;
        let thumb = (self.thumb_mode as i32) << 5;

        mode | neg | zero | carry | overflow | irq | fiq | thumb
    }

    pub fn pack(&mut self, source: u32) {
        let negative = get_bit_at(source, 31);
        let zero = get_bit_at(source, 30);
        let carry = get_bit_at(source, 29);
        let overflow = get_bit_at(source, 28);
        let disable_irq = get_bit_at(source, 7);
        let disable_fiq = get_bit_at(source, 6);
        let thumb_mode = get_bit_at(source, 5);

        let mode = match source & 0b1111 {
            0b0000 => ProcessorMode::User,
            0b1111 => ProcessorMode::System,
            0b0001 => ProcessorMode::FIQ,
            0b0010 => ProcessorMode::IRQ,
            0b0011 => ProcessorMode::Supervisor,
            0b0111 => ProcessorMode::Abort,
            0b1011 => ProcessorMode::Undefined,
            _ => ProcessorMode::User, // shouldn't get here
        };

        *self = Self {
            negative,
            zero,
            carry,
            overflow,
            disable_irq,
            disable_fiq,
            thumb_mode,
            mode,
        }
    }
}

#[inline]
fn get_bit_at(v: u32, pos: u8) -> bool {
    ((v << pos) & 1) != 0
}

impl ARM7TDMI {
    /// Wraps the ARM registers to apply privilege modes when reading.
    pub fn load_register(&mut self, r: usize) -> i32 {
        match self.cpsr.mode {
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

    pub fn store_register(&mut self, r: usize, v: i32) {
        match self.cpsr.mode {
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
            cpsr: PSR::default(),
            spsr_fiq: PSR::default(),
            spsr_irq: PSR::default(),
            spsr_svc: PSR::default(),
            spsr_abt: PSR::default(),
            spsr_und: PSR::default(),
            shifter_carry: 0,
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
