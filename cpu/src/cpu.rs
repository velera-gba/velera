use memory::memory::MMU;
use std::default::Default;
use std::collections::VecDeque;

use crate::arm::{decode_arm};
use crate::thumb::{decode_thumb};
use crate::{arm, gb};

use crate::constants;
use crate::enums::InstructionType;

use crate::utils;

pub struct CPU {
    pub mmu: MMU,
    pub rom: Vec<u8>,
    pub arm: arm::ARM7TDMI,
    pub lr: gb::LR35902,
    pub should_exit: bool,
    pub fetched_instruction: InstructionType,
    pub execution_queue: VecDeque<fn(&mut CPU)>
}

impl Default for CPU {
    /// Create new instance of CPU
    fn default() -> Self {
        Self {
            mmu: MMU::new(constants::default_cpu::MMU_DISPLAY).unwrap(),
            rom: Vec::new(),
            arm: Default::default(),
            lr: Default::default(),
            should_exit: false,
            fetched_instruction: InstructionType::ARM(0), // change the initial value to a NOP! very important
            execution_queue: VecDeque::new()
        }
    }
}

/// Cycle through memory until it gets signalized to exit.
pub fn run_rom_max_cycle(cpu: &mut CPU, rom_path: &str) {
    cpu.rom = utils::read_rom_to_memory(rom_path).unwrap();
    while !cpu.should_exit {
        cycle(cpu);
    }
}

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
    (cpu.arm.cpsr & (1 << constants::cpsr_flags::STATE_BIT))
}

/// Get next instruction.
fn fetch(cpu: &mut CPU) -> InstructionType {
    let index = constants::registers::PROGRAM_COUNTER as usize;
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
        InstructionType::ARM(
            ((cpu.rom[program_counter] as u32) << 24)
                | ((cpu.rom[program_counter + 1] as u32) << 16)
                | ((cpu.rom[program_counter + 2] as u32) << 8)
                | (cpu.rom[program_counter + 3] as u32),
        )
    }
}

fn decode(cpu: &mut CPU) -> VecDeque<fn(&mut CPU)> {
    match cpu.fetched_instruction {
        InstructionType::ARM(x) => {
            return decode_arm(cpu, x);
        }
        InstructionType::Thumb(x) => {
            return decode_thumb(cpu, x);
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
        },
        None => {
            println!("{:#x}: execution queue got to unexpected end, skipping cycle",
                cpu.arm.registers[constants::registers::PROGRAM_COUNTER as usize])
        }
    }
}
