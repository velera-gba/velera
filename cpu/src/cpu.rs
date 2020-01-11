use memory::memory::MMU;
use std::fs::File;
use std::io::{Read, Result};
use std::default::Default;
use std::mem::discriminant;
mod enums;
mod constants;

// the GBA has a coprocessor for backwards compatibility with the GameBoy, based off the Sharp LR35902 (original GameBoy CPU)
// a regular GBA should never switch into this mode, so I'll implement this in case we want backward compatibility
struct LR35902 {

}

impl Default for LR35902 {
    fn default() -> Self {
        Self {

        }
    }
}

struct ARM7HDTI {
    registers: [i32; 16],
    cpsr: u32,
    spsr: u32,
    temp_reg0: i32,
    temp_reg1: i32,
    temp_reg2: i32
}

impl Default for ARM7HDTI {
    fn default() -> Self {
        Self {
            registers: constants::default_cpu::RS,
            cpsr: constants::default_cpu::CPSR,
            spsr: constants::default_cpu::SPSR,
            temp_reg0: 0,
            temp_reg1: 0,
            temp_reg2: 0
        }
    }
}

pub struct CPU {
    mmu: MMU,
    rom: Vec<u8>,
    arm: ARM7HDTI,
    lr: LR35902,
    should_exit: bool
}

impl Default for CPU {
    fn default() -> Self {
        Self{
            mmu: MMU::new(constants::default_cpu::MMU_DISPLAY).unwrap(),
            rom: Vec::new(),
            arm: Default::default(),
            lr: Default::default(),
            should_exit: false
        }
    }
}

pub fn run_rom_max_cycle(cpu: &mut CPU, rom_path: &str) {
    cpu.rom = read_rom_to_memory(rom_path).unwrap();
    while !cpu.should_exit {
        let instruction = fetch(cpu);
        decode(cpu, &instruction);
        execute(cpu, &instruction);
    }
}

pub fn cycle(cpu: &mut CPU) {
    let instruction = fetch(cpu);
    decode(cpu, &instruction);
    execute(cpu, &instruction);
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
    let index = constants::registers::PROGRAM_COUNTER as usize;
    let program_counter = cpu.arm.registers[index] as usize;
    if is_thumb_mode(cpu) != 0 {
        // fetches 16-bit half-word
        cpu.arm.registers[index] += 2;
        return InstructionType::Thumb(
            ((cpu.rom[program_counter] as u16) << 8) |
            (cpu.rom[program_counter + 1] as u16)
        );
    }
    else {
        // fetches 32-bit word
        cpu.arm.registers[index] += 4;
        return InstructionType::ARM(
            ((cpu.rom[program_counter] as u32) << 24) |
            ((cpu.rom[program_counter + 1] as u32) << 16) |
            ((cpu.rom[program_counter + 2] as u32) << 8) |
            (cpu.rom[program_counter + 3] as u32)
        );
    }
}

fn is_thumb_mode(cpu: &CPU) -> u32 {
    (cpu.arm.cpsr & (1 << constants::cpsr_flags::STATE_BIT))
}

fn decode(cpu: &mut CPU, instruction: &InstructionType) {
    match instruction {
        InstructionType::ARM(x) => {
            decode_arm(cpu, x);
        }
        InstructionType::Thumb(x) => {
            decode_thumb(cpu, x);
        }
        _ => {
            println!("Unexpected error in instruction decode at {:#x}, aborting.",
                cpu.arm.registers[constants::registers::PROGRAM_COUNTER as usize]);
        }
    }
}

// DECODE PRODECURES //

fn decode_thumb(cpu: &mut CPU, instruction: &u16) {
    let mut operation: u16 = 0;

    if operation == 0 {
        println!("{:#x}: undefinded THUMB instruction exception.",
            cpu.arm.registers[constants::registers::PROGRAM_COUNTER as usize]);
    }
}

fn decode_arm(cpu: &mut CPU, instruction: &u32) {
    let mut operation: u16 = 0;

    if operation == 0 {
        println!("{:#x}: undefinded ARM instruction exception.",
            cpu.arm.registers[constants::registers::PROGRAM_COUNTER as usize]);
    }
}

// EXECUTION PROCEDURES //

fn execute(cpu: &mut CPU, instruction: &InstructionType) {
    match instruction {
        InstructionType::ARM(x) => {
            execute_arm(cpu, x);
        }
        InstructionType::Thumb(x) => {
            execute_thumb(cpu, x);
        }
        _ => {
            println!("Unexpected error in instruction execution at {:#x}, aborting.",
                cpu.arm.registers[constants::registers::PROGRAM_COUNTER as usize]);
        }
    }
}

fn execute_thumb(cpu: &mut CPU, instruction: &u16) {

}

fn execute_arm(cpu: &mut CPU, instruction: &u32) {

}
