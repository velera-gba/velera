use crate::cpu::{CPU};
use crate::constants;

// does nothing at all. used as a placeholder.
pub fn dummy_cycle(_cpu: &mut CPU) {

}

// for branches: stores the program counter value in the link register
pub fn store_pc_to_lr(cpu: &mut CPU) {
    cpu.arm.registers[constants::registers::LINK_REGISTER] = cpu.arm.registers[constants::registers::PROGRAM_COUNTER];
}