use memory::MMU;

use std::collections::VecDeque;
use std::default::Default;

use crate::arm::decode_arm;
use crate::thumb::decode_thumb;
use crate::{arm, gb};

use crate::constants;
use crate::enums::{InstructionType, ProcessorMode};

use crate::utils;

/// This struct will handle all the memory operations, fetching, decoding and execution of
/// instructions.
///
/// ```
/// let cpu = CPU::default();
/// cpu.fetch();
///
/// // finds out the mode of the function (either thumb or 32-bit arm) and decodes it
/// // through a bit mask or otherwise a DecodedInstruction struct.
/// cpu.decode();
///
/// cpu.execute();
/// ```
pub struct CPU {
    pub mmu: MMU,
    pub rom: Vec<u8>,
    pub arm: arm::ARM7TDMI,
    pub lr: gb::LR35902,
    pub should_exit: bool,
    pub fetched_instruction: InstructionType,
    pub decoded_instruction: InstructionType,
    pub execution_queue: VecDeque<fn(&mut CPU)>,
}

impl Default for CPU {
    /// Create new instance of CPU
    fn default() -> Self {
        Self {
            mmu: MMU::new(),
            rom: Vec::new(),
            arm: Default::default(),
            lr: Default::default(),
            should_exit: false,
            fetched_instruction: InstructionType::Thumb(0), // 0 is no-op
            decoded_instruction: InstructionType::Thumb(0),
            execution_queue: VecDeque::new(),
        }
    }
}

// MUST FIX FOR CYCLE ACCURACY!!!
/// Cycle through memory until it gets signalized to exit.
pub fn run_rom_max_cycle(cpu: &mut CPU, rom_path: &str) {
    cpu.rom = utils::read_rom_to_memory(rom_path).unwrap();
    while !cpu.should_exit {
        cycle(cpu);
    }
}

// MUST FIX FOR CYCLE ACCURACY!!!
/// Run F->D->E cycle.
pub fn cycle(cpu: &mut CPU) {
    execute(cpu);
    if cpu.execution_queue.is_empty() {
        let queue = decode(cpu);
        cpu.execution_queue = queue;
        cpu.fetched_instruction = fetch(cpu);
    }
}

/// Check if a function is in thumb mode
#[inline]
fn is_thumb_mode(cpu: &CPU) -> u32 {
    cpu.arm.cpsr & (1 << constants::cpsr_flags::STATE_BIT)
}

/// Get next instruction.
fn fetch(cpu: &mut CPU) -> InstructionType {
    let index = constants::registers::PROGRAM_COUNTER;
    let program_counter = cpu.arm.registers[index] as usize;
    if is_thumb_mode(cpu) != 0 {
        // fetches 16-bit half-word
        cpu.arm.registers[index] += 2;
        InstructionType::Thumb(
            ((cpu.rom[program_counter] as u16) << 8) | (cpu.rom[program_counter + 1] as u16),
        )
    } else {
        // fetches 32-bit word
        cpu.arm.registers[index] += 4;
        InstructionType::ARM(arm::ARMInstruction::new_fetched(
            ((cpu.rom[program_counter] as u32) << 24)
                | ((cpu.rom[program_counter + 1] as u32) << 16)
                | ((cpu.rom[program_counter + 2] as u32) << 8)
                | (cpu.rom[program_counter + 3] as u32),
        ))
    }
}

fn decode(cpu: &mut CPU) -> VecDeque<fn(&mut CPU)> {
    match cpu.fetched_instruction.clone() {
        InstructionType::ARM(instr) => {
            return decode_arm(cpu, instr.fetched_instruction.unwrap());
        }
        InstructionType::Thumb(instr) => {
            return decode_thumb(cpu, instr);
        }
    }
}

/// Execute the instruction according to its type
fn execute(cpu: &mut CPU) {
    if !cpu.execution_queue.is_empty() {
        pop_micro_operation(cpu)
    }
}

// Executes the next micro operation in the queue of execution
fn pop_micro_operation(cpu: &mut CPU) {
    let result = cpu.execution_queue.pop_front();
    match result {
        Some(function) => {
            function(cpu);
            let word_size = if is_thumb_mode(cpu) != 0 { 16 } else { 32 };
            cpu.arm.registers[constants::registers::PROGRAM_COUNTER as usize] += word_size;
        }

        None => eprintln!(
            "{:#x}: execution queue got to unexpected end, skipping cycle",
            cpu.arm.registers[constants::registers::PROGRAM_COUNTER]
        ),
    }
}
