use memory::memory::MMU;
mod enums;

pub struct CPU {
    mmu: MMU,
    registers: [u32; 16],
    cpsr: u32,
    spsr: u32
}

fn run_rom(cpu: &mut CPU, rom_path: &String) {

}

fn read_rom_to_memory(cpu: &mut CPU, rom_path: &String) {

}

fn fetch(cpu: &mut CPU, rom: &Vec<u32>) {

}

fn decode_execute(cpu: &mut CPU, instruction: u32) {

}
