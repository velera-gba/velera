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
    spsr: u32
}

impl Default for CPU {
    fn default() -> CPU {
        CPU{
            mmu: MMU::new(constants::default_cpu::MMU_DISPLAY).unwrap(),
            registers: constants::default_cpu::RS,
            cpsr: constants::default_cpu::CPSR,
            spsr: constants::default_cpu::SPSR
        }
    }
}

pub fn run_rom(cpu: &mut CPU, rom_path: &str) {
    let rom: Vec<u8> = read_rom_to_memory(rom_path).unwrap();
    println!("{:?}", rom);
}

fn read_rom_to_memory(rom_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(rom_path)?;
    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;
    return Ok(rom);
}

fn fetch(cpu: &mut CPU, rom: &Vec<u8>) {

}

fn decode_execute(cpu: &mut CPU, instruction: u32) {

}
