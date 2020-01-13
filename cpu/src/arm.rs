use crate::constants;
use crate::cpu::CPU;

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
            registers: constants::default_cpu::RS,
            cpsr: constants::default_cpu::CPSR,
            spsr: constants::default_cpu::SPSR,
            temp_rd: 0,
            temp_rs: 0,
            temp_rn: 0,
            immediate: 0,
        }
    }
}

pub fn decode_arm(cpu: &mut CPU, _instruction: u32) {
    let operation: u16 = 0;

    if operation == 0 {
        println!(
            "{:#x}: undefinded ARM instruction exception.",
            cpu.arm.registers[constants::registers::PROGRAM_COUNTER as usize]
        );
    }
}

pub fn execute_arm(_cpu: &mut CPU, _instruction: u32) {
    unimplemented!();
}
