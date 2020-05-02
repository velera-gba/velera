use crate::{
    constants,
    constants::registers,
    cpu::CPU,
    enums::{InstructionType, MnemonicARM, ProcessorMode, ShiftType},
    utils::{get_bit_at, count_set_bits},
};

// Start branch micro operations

/// Stores the program counter value in the link register
pub fn store_pc_to_lr(cpu: &mut CPU) {
    let pc = cpu.arm.load_register(registers::PROGRAM_COUNTER);
    cpu.arm.store_register(registers::LINK_REGISTER, pc);
}

/// Increases the program counter
pub fn increase_pc_by_offset(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm increase_pc_by_offset instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();
        match decoded.offset {
            Some(offset) => {
                let offset = cpu.arm.load_register(registers::PROGRAM_COUNTER) + offset;
                cpu.arm.store_register(registers::PROGRAM_COUNTER, offset);
            }
            _ => {
                eprintln!("Expected offset in branch instruction");
            }
        }
    }
}

/// Switches from arm mode to thumb or vice versa (Branch eXchange)
pub fn switch_mode(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm blx instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        match decoded.rn {
            Some(rn) => {
                let is_thumb = cpu.arm.load_register(rn as usize) & 1;
                cpu.arm.cpsr.thumb_mode = is_thumb != 0;
            }
            _ => {
                eprintln!("Expected to find rn");
            }
        }
    }
}

// End branch micro operations
// ------------------------------
// Start multiply micro operations

// TODO: (Alice Micheloni) Append dummy cycles to cpu directly from here.
// not the most elegant available solution, but works
//
// TODO: treat overflows
// rd = rm * rs
pub fn multiply(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm multiply instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        let rd = decoded.rd.unwrap() as usize;
        let rm = cpu.arm.load_register(decoded.rm.unwrap() as usize);
        let rs = cpu.arm.load_register(decoded.rs.unwrap() as usize);
        let set_cond = decoded.set_cond.unwrap();

        match rm.checked_mul(rs) {
            Some(res) => {
                cpu.arm.store_register(rd, res);
            }
            _ => {
                cpu.arm
                    .store_register(rd, ((rm as i64 * rs as i64) & 0xFFFF_FFFF) as i32);
            }
        }

        if set_cond {
            cpu.arm.cpsr.negative = cpu.arm.load_register(rd) >> 31 != 0;
            cpu.arm.cpsr.zero = cpu.arm.load_register(rd) == 0;
            cpu.arm.cpsr.carry = false;
        }
    }
}

// rd = rm * rs + rn
pub fn multiply_accumulate(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm multiply accumulate instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        let rd = decoded.rd.unwrap() as usize;
        let rm = cpu.arm.load_register(decoded.rm.unwrap() as usize);
        let rs = cpu.arm.load_register(decoded.rs.unwrap() as usize);
        let rn = cpu.arm.load_register(decoded.rn.unwrap() as usize);

        if let Some(res) = rm.checked_mul(rs) {
            let rn_val = cpu.arm.load_register(rn as usize);
            cpu.arm.store_register(rd, res + rn_val);
        } else {
            cpu.arm.store_register(
                rd,
                ((rm as i64 * rs as i64 + rn as i64) & 0xFFFF_FFFF) as i32,
            );
        }
        let rd = cpu.arm.load_register(rd) as i64;
        arm_set_flags_neutral(cpu, rd);
    }
}

pub fn signed_multiply(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm signed multiply instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        let rm = cpu.arm.load_register(decoded.rm.unwrap() as usize);
        let rs = cpu.arm.load_register(decoded.rs.unwrap() as usize);
        let rd_low = decoded.rn.unwrap() as usize;
        let rd_hi = decoded.rd.unwrap() as usize;

        let r = rm as i64 * rs as i64;
        arm_set_flags_neutral(cpu, r);

        cpu.arm.store_register(rd_low, (r & 0xFFFF_FFFF) as i32);
        cpu.arm.store_register(rd_hi, (r >> 32) as i32);
    }
}

pub fn unsigned_multiply(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm unsigned multiply instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();
        let rm = decoded.rm.unwrap() as usize;
        let rs = decoded.rs.unwrap() as usize;

        let rd_low = decoded.rn.unwrap() as usize;
        let rd_hi = decoded.rd.unwrap() as usize;

        let r = cpu.arm.load_register(rm) as u64 * cpu.arm.load_register(rs) as u64;

        arm_set_flags_neutral(cpu, r as i64);
        cpu.arm.store_register(rd_low, (r & 0xFFFF_FFFF) as i32);
        cpu.arm.store_register(rd_hi, (r >> 32) as i32);
    }
}

pub fn signed_multiply_accumulate(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm signed multiply accumulate instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        let rm = decoded.rm.unwrap() as usize;
        let rs = decoded.rs.unwrap() as usize;

        let rd_hi = decoded.rd.unwrap() as usize;
        let rd_low = decoded.rn.unwrap() as usize;

        let r = cpu.arm.load_register(rm) as i64 * cpu.arm.load_register(rs) as i64;

        arm_set_flags_neutral(cpu, r);

        cpu.arm
            .store_register(rd_low, ((r + rd_low as i64) & 0xFFFF_FFFF) as i32);

        let rd_hi_val = cpu.arm.load_register(rd_hi);
        cpu.arm
            .store_register(rd_hi, (rd_hi_val + (r >> 32) as i32) as i32);
    }
}

pub fn unsigned_multiply_accumulate(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!(
                "Expected decoded instruction at arm unsigned multiply accumulate instruction"
            );
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();
        let rm = decoded.rm.unwrap() as usize;
        let rs = decoded.rs.unwrap() as usize;
        let rd_hi = decoded.rd.unwrap() as usize;
        let rd_low = decoded.rn.unwrap() as usize;

        let r = cpu.arm.load_register(rm) as u64 * cpu.arm.load_register(rs) as u64;

        arm_set_flags_neutral(cpu, r as i64);

        cpu.arm
            .store_register(rd_low, ((r + rd_low as u64) & 0xFFFF_FFFF) as i32);

        let rd_hi_val = cpu.arm.load_register(rd_hi) as u32;

        cpu.arm
            .store_register(rd_hi, (rd_hi_val + (r >> 32) as u32) as i32);
    }
}

// End multiply micro operations
// -----------------------------
// Start Load/Store micro operations

// TODO (Alice Micheloni): Should support user access when post-indexing
pub fn single_load_register(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = cpu.decoded_instruction.clone() {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm single load instruction");
            return;
        }

        let decoded = instr.decoded_instruction.unwrap();
        let rd = decoded.rd.unwrap() as usize;

        let rn = decoded.rn.unwrap() as usize;
        let mut rn_val = cpu.arm.load_register(rn as usize) as i32;

        let imm = decoded.imm.unwrap();

        let offset = if imm {
            decoded.offset.unwrap() as i32
        } else {
            let shift_type = decoded.shift_type.clone().unwrap();
            let shift_amount = decoded.val2.unwrap() as i32;
            let rm = decoded.rm.unwrap();
            let rm_val = cpu.arm.load_register(rm as usize) as i32;
            arm_barrelshifter(cpu, shift_type, false, rm_val, shift_amount, rm) as i32
        };

        let flags = decoded.val1.unwrap() as u32;
        let pre = get_bit_at(flags, 3);
        let up = get_bit_at(flags, 2);
        let byte = get_bit_at(flags, 1);
        let writeback = get_bit_at(flags, 0);

        if pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        let val = if byte {
            cpu.mmu.load8(rn_val as u32) as i32
        } else {
            cpu.mmu.load32_rotated(rn_val as u32) as i32
        };

        if !pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        if !pre || writeback {
            cpu.arm.store_register(rn as usize, rn_val);
        }

        cpu.arm.store_register(rd, val);
    }
}

pub fn single_store_register(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = cpu.decoded_instruction.clone() {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm single store instruction");
            return;
        }

        let decoded = instr.decoded_instruction.unwrap();
        let rd = cpu.arm.load_register(decoded.rd.unwrap() as usize);

        let rn = decoded.rn.unwrap() as usize;
        let mut rn_val = cpu.arm.load_register(rn);

        let imm = decoded.imm.unwrap();

        let offset = if imm {
            decoded.offset.unwrap() as i32
        } else {
            let shift_type = decoded.shift_type.clone().unwrap();
            let shift_amount = decoded.val2.unwrap() as i32;
            let rm = decoded.rm.unwrap();
            let rm_val = cpu.arm.load_register(rm as usize) as i32;

            arm_barrelshifter(cpu, shift_type, false, rm_val, shift_amount, rm) as i32
        };

        let flags = decoded.val1.unwrap() as u32;
        let pre = get_bit_at(flags, 3);
        let up = get_bit_at(flags, 2);
        let byte = get_bit_at(flags, 1);
        let writeback = get_bit_at(flags, 0);

        if pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        if byte {
            cpu.mmu.store8(rn_val as u32, rd as u8)
        } else {
            cpu.mmu.store32(rn_val as u32, rd as u32)
        };

        if !pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        if !pre || writeback {
            cpu.arm.store_register(rn, rn_val);
        }
    }
}

/// Executes half-word or signed load
pub fn half_signed_load_register(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = cpu.decoded_instruction.clone() {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm half signed load instruction");
            return;
        }

        let decoded = instr.decoded_instruction.unwrap();

        let dec = &decoded.instr;
        let rd = cpu.arm.load_register(decoded.rd.unwrap() as usize);

        let rn = decoded.rn.unwrap() as usize;
        let mut rn_val = cpu.arm.load_register(rn);

        let imm = decoded.imm.unwrap();

        let offset = if imm {
            decoded.offset.unwrap() as i32
        } else {
            let rm = decoded.rm.unwrap();
            cpu.arm.load_register(rm as usize) as i32
        };

        let flags = decoded.val1.unwrap() as u32;
        let pre = get_bit_at(flags, 3);
        let up = get_bit_at(flags, 2);
        let writeback = get_bit_at(flags, 0);

        if pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        let val = match dec {
            MnemonicARM::LDRH => cpu.mmu.load16_rotated(rn_val as u32) as i32,
            MnemonicARM::LDRSB => cpu.mmu.load8_signed(rn_val as u32) as i32,
            MnemonicARM::LDRSH => cpu.mmu.load16_signed(rn_val as u32) as i32,
            _ => unreachable!(),
        };

        if !pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        if !pre || writeback {
            cpu.arm.store_register(rn as usize, rn_val);
        }

        cpu.arm.store_register(rd as usize, val);
    }
}

/// Stores half-word
pub fn store_half_word(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = cpu.decoded_instruction.clone() {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm store half word instruction");
            return;
        }

        let decoded = instr.decoded_instruction.unwrap();
        let rd = cpu.arm.load_register(decoded.rd.unwrap() as usize);

        let rn = decoded.rn.unwrap() as usize;
        let mut rn_val = cpu.arm.load_register(rn);

        let imm = decoded.imm.unwrap();

        let offset = if imm {
            decoded.offset.unwrap() as i32
        } else {
            let rm = decoded.rm.unwrap();
            cpu.arm.load_register(rm as usize) as i32
        };

        let flags = decoded.val1.unwrap() as u32;
        let pre = get_bit_at(flags, 3);
        let up = get_bit_at(flags, 2);
        let writeback = get_bit_at(flags, 0);

        if pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        cpu.mmu.store16(rn_val as u32, rd as u16);

        if !pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        if !pre || writeback {
            cpu.arm.store_register(rn as usize, rn_val);
        }
    }
}

// SWP R0,R1,[R2] ; Load R0 with the word addressed by R2, and store R1 at R2.
// R0 = [R2]
// [R2] = R1
fn swap(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm swap instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();
        let rn = cpu.arm.load_register(decoded.rn.unwrap() as usize) as u32;
        let rd = decoded.rd.unwrap() as usize;
        let rm = cpu.arm.load_register(decoded.rm.unwrap() as usize) as u32;
        let is_byte = decoded.val1.unwrap() != 0;

        if is_byte {
            let byte = cpu.mmu.load8(rn) as i32;
            cpu.arm.store_register(rd, byte);
            cpu.mmu.store8(rn, rm as u8);
        } else {
            let word = cpu.mmu.load16(rn) as i32;
            cpu.arm.store_register(rd, word);
            cpu.mmu.store16(rn, rm as u16);
        }
    }
}
// End Load/Store micro operations
// -----------------------------
// Start Block Data Transfer micro operations

pub fn load_multiple(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm LDM instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        let reglist = decoded.offset.unwrap() as u32;
        let rn = decoded.rn.unwrap() as usize;
        let flags = decoded.val1.unwrap() as u32;

        let mut writeback = get_bit_at(flags, 0);
        let force_mode = get_bit_at(flags, 1);
        let increment = get_bit_at(flags, 2);
        let mut before = get_bit_at(flags, 3);

        let mode = cpu.arm.cpsr.mode.clone();
        if force_mode {
            cpu.arm.cpsr.mode = ProcessorMode::User;
        }

        let mut addr = cpu.arm.load_register(rn) as u32;

        if reglist != 0 {
            if !increment {
                addr -= 4 * count_set_bits(reglist);
                before ^= true;

                if writeback {
                    cpu.arm.store_register(rn, addr as i32);
                    writeback = false;
                }
            }

            if reglist & (1 << rn) != 0 {
                writeback = false;
            }

            let mut count = 0;
            while count < 16 {
                if ((reglist >> count) & 1) != 0 {
                    if before {
                        addr += 4;
                    }

                    let val = cpu.mmu.load32(addr) as i32;
                    cpu.arm.store_register(count, val);

                    if !before {
                        addr += 4;
                    }
                }
                count += 1;
            }
        } else {
            let val = cpu.mmu.load32(addr) as i32;
            cpu.arm
                .store_register(constants::registers::PROGRAM_COUNTER, val);
        }
        if writeback {
            cpu.arm.store_register(rn, addr as i32);
        }

        if force_mode {
            cpu.arm.cpsr.mode = mode;
        }
    }
}

pub fn store_multiple(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm STM instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        let reglist = decoded.offset.unwrap() as u32;
        let rn = decoded.rn.unwrap() as usize;
        let flags = decoded.val1.unwrap() as u32;

        let mut writeback = get_bit_at(flags, 0);
        let force_mode = get_bit_at(flags, 1);
        let increment = get_bit_at(flags, 2);
        let mut before = get_bit_at(flags, 3);

        let mode = cpu.arm.cpsr.mode.clone();
        if force_mode {
            cpu.arm.cpsr.mode = ProcessorMode::User;
        }

        let mut addr = cpu.arm.load_register(rn) as u32;
        let base = addr;

        if reglist != 0 {
            if !increment {
                addr -= 4 * count_set_bits(reglist);
                before ^= true;

                if writeback {
                    cpu.arm.store_register(rn, addr as i32);
                    writeback = false;
                }
            }

            let mut count = 0;

            let mut begin: bool = true;
            while count < 16 {
                if ((reglist >> count) & 1) != 0 {
                    if before {
                        addr += 4;
                    }

                    cpu.mmu.store32(
                        addr,
                        if count != rn {
                            if count != 15 {
                                (cpu.arm.load_register(count) + 0) as u32
                            } else {
                                (cpu.arm.load_register(count) + 4) as u32
                            }
                        } else {
                            if begin {
                                base
                            } else {
                                (base as i32
                                    + if increment { 4 } else { -4 }
                                        * count_set_bits(reglist) as i32)
                                    as u32
                            }
                        },
                    );

                    if !before {
                        addr += 4;
                    }

                    begin = false;
                }

                count += 1;
            }
        } else {
            let pc = (cpu.arm.load_register(constants::registers::PROGRAM_COUNTER) + 4) as u32;
            match (increment, before) {
                (false, false) => cpu.mmu.store32(addr - 0x3C, pc),
                (true, false) => cpu.mmu.store32(addr - 0x40, pc),
                (false, true) => cpu.mmu.store32(addr + 0x00, pc),
                (true, true) => cpu.mmu.store32(addr + 0x04, pc),
            }

            if increment {
                addr += 0x40;
            } else {
                addr -= 0x40;
            }
        }

        if writeback {
            cpu.arm.store_register(rn, addr as i32);
        }

        if force_mode {
            cpu.arm.cpsr.mode = mode;
        }
    }
}

// End Block Data Transfer micro operations
// -----------------------------
// Start PSR transfer micro operations

/// Save PSR in rd
pub fn mrs(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm MRS instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        let rd = decoded.rd.unwrap() as usize;
        let psr = decoded.val1.unwrap();

        cpu.arm.store_register(
            rd,
            if psr == 0 {
                cpu.arm.cpsr.unpack()
            } else {
                match cpu.arm.cpsr.mode {
                    // no spsr in user/system mode
                    ProcessorMode::User | ProcessorMode::System => return,
                    ProcessorMode::FIQ => cpu.arm.spsr_fiq.unpack(),
                    ProcessorMode::IRQ => cpu.arm.spsr_irq.unpack(),
                    ProcessorMode::Supervisor => cpu.arm.spsr_svc.unpack(),
                    ProcessorMode::Abort => cpu.arm.spsr_abt.unpack(),
                    ProcessorMode::Undefined => cpu.arm.spsr_und.unpack(),
                }
            },
        );
    }
}

/// Save rd in PSR
pub fn msr(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm MSR instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        let mut shifted_imm = 0;

        let rm = decoded.rd.unwrap() as usize;
        let psr = decoded.val1.unwrap();
        let imm = decoded.imm.unwrap();
        let flags = decoded.offset.unwrap();

        if imm {
            let imm_val = decoded.val2.unwrap() as u32;
            // ror in steps of two (0-30)
            let imm_shift = decoded.val3.unwrap() as u32 * 2;

            shifted_imm = arm_ror(cpu, imm_val, imm_shift, false);
        }

        let mut flag_mask = 0;

        if get_bit_at(flags as u32, 3) {
            flag_mask |= constants::psr_mode_flag_masks::PSR_FLAGS_MASK;
        }

        if get_bit_at(flags as u32, 0) && cpu.arm.cpsr.mode != ProcessorMode::User {
            flag_mask |= constants::psr_mode_flag_masks::PSR_CONTROL_MASK;
        }

        let val = if imm {
            cpu.arm.load_register(rm) as u32
        } else {
            shifted_imm
        };

        if psr == 0 {
            cpu.arm.cpsr.pack(val & flag_mask);
        } else {
            match cpu.arm.cpsr.mode {
                // no spsr in user/system mode
                ProcessorMode::User | ProcessorMode::System => cpu.arm.cpsr.pack(val & flag_mask),
                ProcessorMode::FIQ => cpu.arm.spsr_fiq.pack(val & flag_mask),
                ProcessorMode::IRQ => cpu.arm.spsr_irq.pack(val & flag_mask),
                ProcessorMode::Supervisor => cpu.arm.spsr_svc.pack(val & flag_mask),
                ProcessorMode::Abort => cpu.arm.spsr_abt.pack(val & flag_mask),
                ProcessorMode::Undefined => cpu.arm.spsr_und.pack(val & flag_mask),
            }
        }
    }
}

// End MRS/MSR micro operations
// -----------------------------
// Start ALU micro operations

// TODO (Alice Micheloni): Handle PC ops.
// TODO (Alice Micheloni): Correct execution time.
pub fn alu_master(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        use crate::enums::MnemonicARM::*;

        if instr.decoded_instruction.is_none() {
            eprintln!("Expected decoded instruction at arm ALU instruction");
            return;
        }

        let decoded = instr.decoded_instruction.as_ref().unwrap();

        let rn = cpu.arm.load_register(decoded.rn.unwrap() as usize) as i32;
        let set_cond = decoded.set_cond.unwrap();
        let rd = decoded.rn.unwrap() as usize;
        let mnemonic = decoded.instr.clone();

        // implement the barrel shifter
        let op2 = if decoded.imm.unwrap() {
            // the immediate shift can only represent even numbers, so we multiply it by 2.
            let shift = decoded.val1.unwrap() as u32 * 2;
            let imm_value = decoded.val2.unwrap() as u32;
            imm_value.rotate_right(shift) as i32
        } else {
            let rm = decoded.rm.unwrap();
            let to_shift = cpu.arm.load_register(rm as usize);

            let shift_type = decoded.shift_type.clone().unwrap();

            let shift_amount;

            // if rs is defined, we have to shift by register
            if let Some(rs) = decoded.rs {
                shift_amount = cpu.arm.load_register(rs as usize);
            } else {
                shift_amount = decoded.val1.unwrap() as i32;
            }

            arm_barrelshifter(cpu, shift_type, set_cond, to_shift, shift_amount, rm)
        };

        match mnemonic {
            // logical ops
            AND => {
                cpu.arm.store_register(rd, rn & op2);
                if set_cond {
                    arm_set_flags_neutral(cpu, (rn & op2) as i64);
                }
            }
            EOR => {
                cpu.arm.store_register(rd, rn ^ op2);
                if set_cond {
                    let rd = cpu.arm.load_register(rd) as i64;
                    arm_set_flags_neutral(cpu, rd);
                }
            }
            ORR => {
                cpu.arm.store_register(rd, rn | op2);
                if set_cond {
                    let rd = cpu.arm.load_register(rd) as i64;
                    arm_set_flags_neutral(cpu, rd);
                }
            }
            BIC => {
                cpu.arm.store_register(rd, rn & !op2);
                if set_cond {
                    let rd = cpu.arm.load_register(rd) as i64;
                    arm_set_flags_neutral(cpu, rd);
                }
            }

            // addition ops
            ADD => {
                let val = arm_addition(cpu, rn as u32, op2 as u32, set_cond) as i32;
                cpu.arm.store_register(rd, val);
            }
            ADC => {
                let val = arm_addition(cpu, rn as u32, op2 as u32, set_cond) as i32
                    + cpu.arm.cpsr.carry as i32;
                cpu.arm.store_register(rd, val);
            }

            // subtraction ops
            SUB => {
                let val = arm_subtract(cpu, rn as u32, op2 as u32, set_cond) as i32;
                cpu.arm.store_register(rd, val);
            }
            RSB => {
                let val = arm_subtract(cpu, op2 as u32, rn as u32, set_cond) as i32;
                cpu.arm.store_register(rd, val);
            }
            SBC => {
                let val = arm_subtract(cpu, rn as u32, op2 as u32, set_cond) as i32
                    + cpu.arm.cpsr.carry as i32
                    - 1;
                cpu.arm.store_register(rd, val);
            }
            RSC => {
                let val = arm_subtract(cpu, op2 as u32, rn as u32, set_cond) as i32
                    + cpu.arm.cpsr.carry as i32
                    - 1;
                cpu.arm.store_register(rd, val);
            }

            // move ops
            MOV => cpu.arm.store_register(rd, op2),
            MVN => cpu.arm.store_register(rd, !op2),

            // testing ops
            TST => arm_set_flags_neutral(cpu, (rn & op2) as i64),
            TEQ => arm_set_flags_neutral(cpu, (rn ^ op2) as i64),

            CMP => {
                arm_subtract(cpu, rn as u32, op2 as u32, true);
            }

            CMN => {
                arm_addition(cpu, rn as u32, op2 as u32, true);
            }

            x => panic!("Unexpected instruction in ALU, {:?}", x),
        }
    }
}

#[inline]
fn set_zero_flag(cpu: &mut CPU, z: i32) {
    cpu.arm.cpsr.zero = z == 0;
}

fn arm_set_flags_neutral(cpu: &mut CPU, d: i64) {
    cpu.arm.cpsr.negative = d < 0;
    cpu.arm.cpsr.zero = d == 0;

    cpu.arm.cpsr.carry = (cpu.arm.shifter_carry << 29) & 1 != 0;
}

fn arm_addition(cpu: &mut CPU, x: u32, y: u32, set_cond: bool) -> u32 {
    if let Some(z) = x.checked_add(y) {
        if set_cond {
            cpu.arm.cpsr.overflow = z > std::u32::MAX >> 1;
            set_zero_flag(cpu, z as i32);
        }
        z
    } else {
        let z = ((x as u64 + y as u64) & 0xFFFF_FFFF) as u32;
        if set_cond {
            set_zero_flag(cpu, z as i32);
            cpu.arm.cpsr.overflow = true;
            cpu.arm.cpsr.carry = true;
        }

        z
    }
}

fn arm_subtract(cpu: &mut CPU, x: u32, y: u32, set_cond: bool) -> u32 {
    if let Some(z) = x.checked_sub(y) {
        if set_cond {
            set_zero_flag(cpu, z as i32);
        }
        z
    } else {
        let z = x as i32 - y as i32;

        if set_cond {
            cpu.arm.cpsr.carry = true;
            set_zero_flag(cpu, z);
            cpu.arm.cpsr.negative = z < 0;
        }

        0
    }
}

fn arm_ror(cpu: &mut CPU, val: u32, to_shift: u32, set_cond: bool) -> u32 {
    if to_shift != 0 {
        let val = val.rotate_right(to_shift);

        if set_cond {
            cpu.arm.cpsr.negative = val >> 31 != 0;
        }
        val
    } else {
        let val = val >> 1;
        let carry = cpu.arm.cpsr.carry as u32;
        cpu.arm.shifter_carry = carry;
        val | (carry << 31)
    }
}

fn arm_barrelshifter(
    cpu: &mut CPU,
    shift_type: ShiftType,
    set_cond: bool,
    to_shift: i32,
    shift_amount: i32,
    rm: u8,
) -> i32 {
    match shift_type {
        ShiftType::LSL => {
            if set_cond && shift_amount > 0 {
                cpu.arm.shifter_carry |= ((to_shift << (shift_amount - 1)) & 1) as u32;
            }
            // force logical shift
            let x = (to_shift >> shift_amount) as u32;
            x as i32
        }

        ShiftType::LSR => {
            if shift_amount == 0 {
                let carry = cpu.arm.cpsr.carry as i32;
                let rm_val = cpu.arm.load_register(rm as usize);
                cpu.arm
                    .store_register(rm as usize, rm_val | (carry << 31) as i32);
                0
            } else {
                if set_cond {
                    cpu.arm.shifter_carry |= ((to_shift << (shift_amount - 1)) & 1) as u32;
                }

                // force logical shift
                let x = (to_shift >> shift_amount) as u32;
                x as i32
            }
        }

        ShiftType::ASR => {
            if shift_amount == 0 {
                let bit = cpu.arm.load_register(rm as usize) >> 31;
                if bit == 0 {
                    if set_cond {
                        cpu.arm.cpsr.carry = false;
                    }
                    0
                } else {
                    if set_cond {
                        // set carry bit
                        cpu.arm.cpsr.carry = true;
                    }

                    std::i32::MAX
                }
            } else {
                to_shift >> shift_amount
            }
        }

        ShiftType::ROR => arm_ror(cpu, to_shift as u32, shift_amount as u32, set_cond) as i32,
    }
}

// End ALU micro operations
// -------------------------
// Start misc operations

pub fn switch_to_svc(cpu: &mut CPU) {
    cpu.arm.cpsr.mode = ProcessorMode::Supervisor;
    cpu.arm.cpsr.disable_irq = true;
    cpu.arm.cpsr.thumb_mode = false;

    let pc = cpu.arm.load_register(registers::PROGRAM_COUNTER);
    cpu.arm.store_register(registers::LINK_REGISTER, pc + 4);

    // jump to SWI/PrefetchAbort vector address
    cpu.arm
        .store_register(registers::PROGRAM_COUNTER, 0x0000_0008);
}
