use crate::cpu::{CPU};
use crate::constants;

pub fn store_pc_to_lr(cpu: &mut CPU) {
    cpu.arm.registers[constants::registers::LINK_REGISTER] = cpu.arm.registers[constants::registers::PROGRAM_COUNTER];
}