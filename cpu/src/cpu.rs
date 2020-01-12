use memory::memory::MMU;
use std::fs::File;
use std::io::{Read, Result};
use std::default::Default;
mod enums;
mod constants;

// this macro takes a lot of parameters, packages them for the decoding procedures and does this to an arbitrary
// number of operations. A method with over 4 parameters might kill performance so I used a struct.
// NOT TESTED!
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
    temp_rd: i32, // temporary destination register
    temp_rs: i32, // temporary source register
    temp_rn: i32, // temporary index register
    immediate: i32 // temporary immediate
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

// NOT TESTED!
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

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::ALU_OP_MASK,
        constants::thumb_bitmasks::ALU_RD_MASK,
        constants::thumb_bitmasks::ALU_RS_MASK,
        0,
        0,
        constants::thumb_bitmasks::ALU_AND,
        constants::thumb_bitmasks::ALU_EOR,
        constants::thumb_bitmasks::ALU_LSL,
        constants::thumb_bitmasks::ALU_LSR,
        constants::thumb_bitmasks::ALU_ASR,
        constants::thumb_bitmasks::ALU_ADC,
        constants::thumb_bitmasks::ALU_SBC,
        constants::thumb_bitmasks::ALU_ROR,
        constants::thumb_bitmasks::ALU_TST,    
        constants::thumb_bitmasks::ALU_NEG,    
        constants::thumb_bitmasks::ALU_CMP,    
        constants::thumb_bitmasks::ALU_CMN,    
        constants::thumb_bitmasks::ALU_ORR,    
        constants::thumb_bitmasks::ALU_MUL,    
        constants::thumb_bitmasks::ALU_BIC,    
        constants::thumb_bitmasks::ALU_MVN
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::HI_OP_MASK,
        constants::thumb_bitmasks::HI_RD,
        constants::thumb_bitmasks::HI_RS,
        constants::thumb_bitmasks::HI_MSBD_MASK,
        constants::thumb_bitmasks::HI_MSBS_MASK,
        constants::thumb_bitmasks::HI_ADD,
        constants::thumb_bitmasks::HI_CMP,
        constants::thumb_bitmasks::HI_MOV,
        constants::thumb_bitmasks::HI_NOP,
        constants::thumb_bitmasks::BX,
        constants::thumb_bitmasks::BLX
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::HI_OP_MASK,
        constants::thumb_bitmasks::HI_RD,
        constants::thumb_bitmasks::HI_RS,
        constants::thumb_bitmasks::HI_MSBD_MASK,
        constants::thumb_bitmasks::HI_MSBS_MASK,
        constants::thumb_bitmasks::HI_ADD,
        constants::thumb_bitmasks::HI_CMP,
        constants::thumb_bitmasks::HI_MOV,
        constants::thumb_bitmasks::HI_NOP,
        constants::thumb_bitmasks::BX,
        constants::thumb_bitmasks::BLX
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::LDPCR_MASK,
        constants::thumb_bitmasks::LDPCR_RD,
        0,
        0,
        constants::thumb_bitmasks::LDPCR_OFFSET,
        constants::thumb_bitmasks::LDPCR
    );

    // here (and in other places) I use the source temporary register as the base register
    // and the index register as the offset register
    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::LS_REG_OFFSET_OPCODE_MASK,
        constants::thumb_bitmasks::LS_REG_OFFSET_RD_MASK,
        constants::thumb_bitmasks::LS_REG_OFFSET_RB_MASK,
        constants::thumb_bitmasks::LS_REG_OFFSET_RO_MASK,
        0,
        constants::thumb_bitmasks::STR,
        constants::thumb_bitmasks::STRB,
        constants::thumb_bitmasks::LDR,
        constants::thumb_bitmasks::LDRB
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::LS_EBH_OP_MASK,
        constants::thumb_bitmasks::LS_EBH_RD_MASK,
        constants::thumb_bitmasks::LS_EBH_RB_MASK,
        constants::thumb_bitmasks::LS_EBH_RO_MASK,
        0,
        constants::thumb_bitmasks::STRH,
        constants::thumb_bitmasks::LDSB,
        constants::thumb_bitmasks::LDRH,
        constants::thumb_bitmasks::LDSH
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::LS_NN_OFFSET_OP_MASK,
        constants::thumb_bitmasks::LS_NN_OFFSET_RD_MASK,
        constants::thumb_bitmasks::LS_NN_OFFSET_RB_MASK,
        0,
        constants::thumb_bitmasks::LS_NN_OFFSET_NN_MASK,
        constants::thumb_bitmasks::STRI,
        constants::thumb_bitmasks::LDRI,
        constants::thumb_bitmasks::STRBI,
        constants::thumb_bitmasks::LDRBI
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::LS_HW_OP_MASK,
        constants::thumb_bitmasks::LS_HW_RD_MASK,
        constants::thumb_bitmasks::LS_HW_RB_MASK,
        0,
        constants::thumb_bitmasks::LS_HW_NN_MASK,
        constants::thumb_bitmasks::STRHW,
        constants::thumb_bitmasks::LDRHW
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::SP_LS_OP_MASK,
        constants::thumb_bitmasks::SP_LS_RD_MASK,
        0,
        0,
        constants::thumb_bitmasks::SP_LS_NN_MASK,
        constants::thumb_bitmasks::SP_STR,
        constants::thumb_bitmasks::SP_LDR
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::RELATIVE_ADDR_OP_MASK,
        constants::thumb_bitmasks::RELATIVE_ADDR_RD_MASK,
        0,
        0,
        constants::thumb_bitmasks::RELATIVE_ADDR_NN_MASK,
        constants::thumb_bitmasks::ADD_PC,
        constants::thumb_bitmasks::ADD_SP
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::SP_OFFSET_OP_MASK,
        0,
        0,
        0,
        constants::thumb_bitmasks::SP_OFFSET_NN_MASK,
        constants::thumb_bitmasks::ADD_SP_NN,
        constants::thumb_bitmasks::ADD_SP_MINUS_NN
    );

    // the immediate here is actually the PC/LR bit
    // and the index register is contains bits to each one of the general purpose registers
    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::STACK_OPS_OP_MASK,
        0,
        0,
        constants::thumb_bitmasks::STACK_OPS_RLIST_MASK,
        constants::thumb_bitmasks::STACK_OPS_PC_LR_BIT_MASK,
        constants::thumb_bitmasks::PUSH,
        constants::thumb_bitmasks::POP
    );

    // same thing here
    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::LS_MIA_OP_MASK,
        0,
        constants::thumb_bitmasks::LS_MIA_RB_MASK,
        constants::thumb_bitmasks::LS_MIA_RLIST_MASK,
        0,
        constants::thumb_bitmasks::STMIA,
        constants::thumb_bitmasks::LDMIA
    );

    // the conditional branch is an interesting case, I associated the operation bits with the opcode bitmasks
    // so I avoided writing too much code
    let extra_opcode_mask = constants::thumb_bitmasks::COND_GENERAL_OP_MASK ^ constants::thumb_bitmasks::COND_FULL_OP_MASK;
    let cond_branch_shift = 8;
    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::COND_FULL_OP_MASK,
        0,
        0,
        0,
        constants::thumb_bitmasks::COND_OFFSET_MASK,
        ((constants::cond_arm::ARM_CONDITION_EQ as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_NE as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_CS as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_CC as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_MI as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_PL as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_VS as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_VC as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_HI as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_LS as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_GE as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_LT as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_GT as u16) << cond_branch_shift) & extra_opcode_mask,
        ((constants::cond_arm::ARM_CONDITION_LE as u16) << cond_branch_shift) & extra_opcode_mask
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::SWI_BK_OP_MASK,
        0,
        0,
        0,
        constants::thumb_bitmasks::SWI_BK_NN_MASK,
        constants::thumb_bitmasks::SWI,
        constants::thumb_bitmasks::BKPT
    );

    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::B_OP_MASK,
        0,
        0,
        0,
        constants::thumb_bitmasks::B_OFFSET_MASK,
        constants::thumb_bitmasks::B
    );

    // I should change this in the future, because this instruction is actually 32 bits with a long branch
    // but I should find a way to make this work as it is
    temp_reg_wrap!(cpu, instruction, &mut operation,
        constants::thumb_bitmasks::LONG_BRANCH_OP_MASK,
        0,
        0,
        0,
        constants::thumb_bitmasks::LONG_BRANCH_ADDR_MASK,
        constants::thumb_bitmasks::LONG_BRANCH_FIRST_OP,
        constants::thumb_bitmasks::BL,
        constants::thumb_bitmasks::BLLX
    );

    // operation not found error check
    if operation == false {
        println!("{:#x}: undefinded THUMB instruction exception.",
            cpu.arm.registers[constants::registers::PROGRAM_COUNTER as usize]);
    }
}

// NOT TESTED!
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

// NOT TESTED!
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
