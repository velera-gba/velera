use memory::memory::MMU;
use std::fs::File;
use std::io::{Read, Result};
use std::default::Default;
mod enums;
mod constants;

macro_rules! temp_reg_wrap {
    ($cpu: expr,
    $instruction: expr,
    $operation: expr,
    $opcode: expr,
    $rd: expr,
    $rs: expr,
    $rn: expr,
    $immediate: expr,
    $x: expr) => {
        pass_operation_thumb($cpu, $instruction, $operation, ThumbOpPack {
            op_bitmask: $x,
            opcode_bitmask: $opcode,
            rd_bitmask: $rd,
            rs_bitmask: $rs,
            rn_bitmask: $rn,
            immediate_bitmask: $immediate
        });
    };

    ($cpu: expr,
    $instruction: expr,
    $operation: expr,
    $opcode: expr,
    $rd: expr,
    $rs: expr,
    $rn: expr,
    $immediate: expr,
    $x: expr,
    $($xs: expr),*) => {
        temp_reg_wrap!($cpu, $instruction, $operation, $opcode, $rd, $rs, $rn, $immediate, $x);
        temp_reg_wrap!($cpu, $instruction, $operation, $opcode, $rd, $rs, $rn, $immediate, $($xs),*)
    };  
}

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
    temp_rd: i32,
    temp_rs: i32,
    temp_rn: i32,
    immediate: i32
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
            immediate: 0
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
    }
}

// DECODE PRODECURES //

// THUMB //

struct ThumbOpPack {
    op_bitmask: u16,
    opcode_bitmask: u16,
    rd_bitmask: u16,
    rs_bitmask: u16,
    rn_bitmask: u16,
    immediate_bitmask: u16,
}

fn decode_thumb(cpu: &mut CPU, instruction: &u16) {
    let mut operation: bool = false;

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::MOVE_SHIFTED_REG_OP_MASK,
        constants::thumb_bitmasks::MOVE_SHIFTED_REG_RD_MASK,
        constants::thumb_bitmasks::MOVE_SHIFTED_REG_RS_MASK,
        0,
        constants::thumb_bitmasks::MOVE_SHIFTED_REG_OFFSET_MASK,
        constants::thumb_bitmasks::LSR,
        constants::thumb_bitmasks::LSL,
        constants::thumb_bitmasks::ASR
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::ADDSUB_OP_MASK,
        constants::thumb_bitmasks::ADDSUB_RD_MASK,
        constants::thumb_bitmasks::ADDSUB_RS_MASK,
        constants::thumb_bitmasks::ADDSUB_RN_MASK,
        0,
        constants::thumb_bitmasks::ADD,
        constants::thumb_bitmasks::SUB,
        constants::thumb_bitmasks::ADDI,
        constants::thumb_bitmasks::SUBI
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::IMMEDIATE_OP_MASK,
        constants::thumb_bitmasks::IMMEDIATE_RD_MASK,
        0,
        0,
        constants::thumb_bitmasks::IMMEDIATE_NN_MASK,
        constants::thumb_bitmasks::MOV,
        constants::thumb_bitmasks::CMP,
        constants::thumb_bitmasks::ADDRI,
        constants::thumb_bitmasks::SUBRI
    );

    // operation not found error check
    if operation == false {
        println!("{:#x}: undefinded THUMB instruction exception.",
            cpu.arm.registers[constants::registers::PROGRAM_COUNTER as usize]);
    }
}

fn pass_operation_thumb(cpu: &mut CPU, instruction: &u16, operation: &mut bool, pack: ThumbOpPack) {
    if (!(pack.op_bitmask ^ instruction) & pack.opcode_bitmask) == pack.opcode_bitmask  {
        *operation = true;
        if pack.rd_bitmask != 0 {
            put_temp_register_thumb(&mut cpu.arm.temp_rd, &pack.rd_bitmask, instruction);
        }
        if pack.rs_bitmask != 0 {
            put_temp_register_thumb(&mut cpu.arm.temp_rs, &pack.rs_bitmask, instruction);
        }
        if pack.rn_bitmask != 0 {
            put_temp_register_thumb(&mut cpu.arm.temp_rn, &pack.rn_bitmask, instruction)
        }
        if pack.immediate_bitmask != 0 {
            put_temp_register_thumb(&mut cpu.arm.immediate, &pack.immediate_bitmask, instruction);
        }
    }
}

fn put_temp_register_thumb(register: &mut i32, register_bitmask: &u16, instruction: &u16) {
    let mut bitmask_eval = *register_bitmask;
    let mut shift_modifier = 0;
    while bitmask_eval % 2 == 0 {
        bitmask_eval >>= 1;
        shift_modifier += 1;
    }
    *register = ((register_bitmask & instruction) >> shift_modifier) as i32;
}

// ARM //

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

// THUMB //

fn execute_thumb(cpu: &mut CPU, instruction: &u16) {

}

// ARM //

fn execute_arm(cpu: &mut CPU, instruction: &u32) {

}
