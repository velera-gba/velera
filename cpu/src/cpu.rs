use memory::memory::MMU;
use std::fs::File;
use std::io::{Read, Result};
use std::default::Default;
mod enums;
mod constants;

pub struct CPU {
    mmu: MMU,
    registers: [u32; 16],
    cpsr: u32,
    spsr: u32,
    rom: Vec<u8>
}

impl Default for CPU {
    fn default() -> CPU {
        CPU{
            mmu: MMU::new(constants::default_cpu::MMU_DISPLAY).unwrap(),
            registers: constants::default_cpu::RS,
            cpsr: constants::default_cpu::CPSR,
            spsr: constants::default_cpu::SPSR,
            rom: Vec::new()
        }
    }
}

pub fn run_rom_max_cycle(cpu: &mut CPU, rom_path: &str) {
    cpu.rom = read_rom_to_memory(rom_path).unwrap();
}

pub fn cycle(cpu: &mut CPU) {
    let instruction = fetch(cpu);
    decode_execute(cpu, instruction);
}

fn read_rom_to_memory(rom_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(rom_path)?;
    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;
    return Ok(rom);
}

enum InstructionType {
    Thumb(u16),
    ARM(u32)
}

fn fetch(cpu: &mut CPU) -> InstructionType {
    let program_counter = cpu.registers[constants::registers::PROGRAM_COUNTER] as usize;
    if is_thumb_mode(cpu) != 0 {
        // fetches 16-bit half-word
        cpu.registers[constants::registers::PROGRAM_COUNTER] += 2;
        return InstructionType::Thumb(
            ((cpu.rom[program_counter] as u16) << 8) |
            (cpu.rom[program_counter + 1] as u16)
        );
    }
    else {
        // fetches 32-bit word
        cpu.registers[constants::registers::PROGRAM_COUNTER] += 4;
        return InstructionType::ARM(
            ((cpu.rom[program_counter] as u32) << 24) |
            ((cpu.rom[program_counter + 1] as u32) << 16) |
            ((cpu.rom[program_counter + 2] as u32) << 8) |
            (cpu.rom[program_counter + 3] as u32)
        )
    }
}

fn is_thumb_mode(cpu: &CPU) -> u32 {
    (cpu.cpsr & (1 << constants::cpsr_flags::STATE_BIT))
}

fn decode_execute(cpu: &mut CPU, instruction: InstructionType) {

}
