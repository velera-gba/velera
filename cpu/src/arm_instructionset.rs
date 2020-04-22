use crate::{
    constants,
    constants::registers,
    cpu::CPU,
    enums::{InstructionType, ProcessorMode, ShiftType},
};

#[inline]
fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        return input & (1 << n) != 0;
    }
    false
}

// Start branch micro operations

/// Stores the program counter value in the link register
#[inline]
pub fn store_pc_to_lr(cpu: &mut CPU) {
    let pc = cpu.arm.load_register(registers::PROGRAM_COUNTER);
    cpu.arm.store_register(registers::LINK_REGISTER, pc);
}

/// Increases the program counter
pub fn increase_pc_by_offset(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if let Some(decoded) = &instr.decoded_instruction {
            if let Some(offset) = decoded.offset {
                let offset = cpu.arm.load_register(registers::PROGRAM_COUNTER) + offset;
                cpu.arm.store_register(registers::PROGRAM_COUNTER, offset);
            } else {
                eprintln!("Expected offset in branch instruction");
            }
        } else {
            eprintln!("Expected decoded instruction at arm increase pc by offset");
        }
    }
}

/// Switches from arm mode to thumb or vice versa (Branch eXchange)
pub fn switch_mode(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        if let Some(decoded) = &instr.decoded_instruction {
            if let Some(rn) = decoded.rn {
                let is_thumb = cpu.arm.load_register(rn as usize) & 1;
                cpu.arm.cpsr.thumb_mode = is_thumb != 0;
            } else {
                eprintln!("Expected to find rn");
            }
        } else {
            eprintln!("Expected decoded instruction at arm blx");
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
        let (rd, rm, rs, set_cond);
        if let Some(decoded) = &instr.decoded_instruction {
            rd = decoded.rd.unwrap() as usize;
            rm = cpu.arm.load_register(decoded.rm.unwrap() as usize);
            rs = cpu.arm.load_register(decoded.rs.unwrap() as usize);
            set_cond = decoded.set_cond.unwrap();
        } else {
            eprintln!("Expected decoded instruction at arm multiply instruction");
            return;
        }

        if let Some(res) = rm.checked_mul(rs) {
            cpu.arm.store_register(rd, res);
        } else {
            cpu.arm
                .store_register(rd, ((rm as i64 * rs as i64) & 0xFFFF_FFFF) as i32);
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
        let (rd, rm, rs, rn);
        if let Some(decoded) = &instr.decoded_instruction {
            rd = decoded.rd.unwrap() as usize;
            rm = cpu.arm.load_register(decoded.rm.unwrap() as usize);
            rs = cpu.arm.load_register(decoded.rs.unwrap() as usize);
            rn = cpu.arm.load_register(decoded.rn.unwrap() as usize);
        } else {
            eprintln!("Expected decoded instruction at arm multiply accumulate instruction");
            return;
        }

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
        let (rm, rs, rd_low, rd_hi);
        if let Some(decoded) = &instr.decoded_instruction {
            rm = cpu.arm.load_register(decoded.rm.unwrap() as usize);
            rs = cpu.arm.load_register(decoded.rs.unwrap() as usize);
            rd_low = decoded.rn.unwrap() as usize;
            rd_hi = decoded.rd.unwrap() as usize;
        } else {
            eprintln!("Expected decoded instruction at arm signed multiply accumulate instruction");
            return;
        }

        let r = rm as i64 * rs as i64;
        arm_set_flags_neutral(cpu, r);

        cpu.arm.store_register(rd_low, (r & 0xFFFF_FFFF) as i32);
        cpu.arm.store_register(rd_hi, (r >> 32) as i32);
    }
}

pub fn unsigned_multiply(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        let (rm, rs, rd_low, rd_hi);
        if let Some(decoded) = &instr.decoded_instruction {
            rm = decoded.rm.unwrap() as usize;
            rs = decoded.rs.unwrap() as usize;
            rd_low = decoded.rn.unwrap() as usize;
            rd_hi = decoded.rd.unwrap() as usize;
        } else {
            eprintln!("Expected decoded instruction at arm unsigned multiply instruction");
            return;
        }

        let r = cpu.arm.load_register(rm) as u64 * cpu.arm.load_register(rs) as u64;

        arm_set_flags_neutral(cpu, r as i64);
        cpu.arm.store_register(rd_low, (r & 0xFFFF_FFFF) as i32);
        cpu.arm.store_register(rd_hi, (r >> 32) as i32);
    }
}

pub fn signed_multiply_accumulate(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        let (rd_hi, rd_low, rm, rs);
        if let Some(decoded) = &instr.decoded_instruction {
            rm = decoded.rm.unwrap() as usize;
            rs = decoded.rs.unwrap() as usize;
            rd_hi = decoded.rd.unwrap() as usize;
            rd_low = decoded.rn.unwrap() as usize;
        } else {
            eprintln!("Expected decoded instruction at arm signed multiply accumulate instruction");
            return;
        }

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
        let (rd_hi, rd_low, rm, rs);
        if let Some(decoded) = &instr.decoded_instruction {
            rm = decoded.rm.unwrap() as usize;
            rs = decoded.rs.unwrap() as usize;
            rd_hi = decoded.rd.unwrap() as usize;
            rd_low = decoded.rn.unwrap() as usize;
        } else {
            eprintln!(
                "Expected decoded instruction at arm unsigned multiply accumulate instruction"
            );
            return;
        }

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

pub fn load_register(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = cpu.decoded_instruction.clone() {
        let (rn, mut rn_val, rd, imm);
        let (pre, up, half, writeback);
        let offset: i32;
        if let Some(decoded) = &instr.decoded_instruction {
            rd = decoded.rd.unwrap() as usize;
            rn = decoded.rn.unwrap() as usize;
            rn_val = cpu.arm.load_register(rn as usize) as i32;
            imm = decoded.imm.unwrap();

            if imm {
                offset = decoded.offset.unwrap() as i32;
            } else {
                let shift_type = decoded.shift_type.clone().unwrap();
                let shift_amount = decoded.val2.unwrap() as i32;
                let rm = decoded.rm.unwrap();
                let rm_val = cpu.arm.load_register(rm as usize) as i32;
                offset = arm_barrelshifter(cpu, shift_type, false, rm_val, shift_amount, rm) as i32;
            }

            let flags = decoded.val1.unwrap() as u32;
            pre = get_bit_at(flags, 3);
            up = get_bit_at(flags, 2);
            half = get_bit_at(flags, 1);
            writeback = get_bit_at(flags, 0);
        } else {
            eprintln!("Expected decoded instruction at arm LDR instruction");
            return;
        }

        if pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        let val = if half {
            cpu.mmu.load16(rn_val as u32) as i32
        } else {
            cpu.mmu.load8(rn_val as u32) as i32
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

pub fn store_register(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = cpu.decoded_instruction.clone() {
        let (rn, mut rn_val, rd, imm);
        let (pre, up, half, writeback);
        let offset: i32;
        if let Some(decoded) = &instr.decoded_instruction {
            rd = cpu.arm.load_register(decoded.rd.unwrap() as usize);
            rn = decoded.rn.unwrap() as usize;
            rn_val = cpu.arm.load_register(rn);
            imm = decoded.imm.unwrap();

            if imm {
                offset = decoded.offset.unwrap() as i32;
            } else {
                let shift_type = decoded.shift_type.clone().unwrap();
                let shift_amount = decoded.val2.unwrap() as i32;
                let rm = decoded.rm.unwrap();
                let rm_val = cpu.arm.load_register(rm as usize) as i32;
                offset = arm_barrelshifter(cpu, shift_type, false, rm_val, shift_amount, rm) as i32;
            }

            let flags = decoded.val1.unwrap() as u32;
            pre = get_bit_at(flags, 3);
            up = get_bit_at(flags, 2);
            half = get_bit_at(flags, 1);
            writeback = get_bit_at(flags, 0);
        } else {
            eprintln!("Expected decoded instruction at arm LDR instruction");
            return;
        }

        if pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        if half {
            cpu.mmu.store16(rn_val as u32, rd as u16)
        } else {
            cpu.mmu.store8(rn_val as u32, rd as u8)
        };

        if !pre {
            rn_val = if up { rn_val + offset } else { rn_val - offset }
        }

        if !pre || writeback {
            cpu.arm.store_register(rn, rn_val);
        }
    }
}

// SWP R0,R1,[R2] ; Load R0 with the word addressed by R2, and store R1 at R2.
// R0 = [R2]
// [R2] = R1
fn swap(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        let (rn, rd, rm, is_byte);
        if let Some(decoded) = &instr.decoded_instruction {
            rn = cpu.arm.load_register(decoded.rn.unwrap() as usize) as u32;
            rd = decoded.rd.unwrap() as usize;
            rm = cpu.arm.load_register(decoded.rm.unwrap() as usize) as u32;
            is_byte = decoded.val1.unwrap() != 0;
        } else {
            eprintln!("Expected decoded instruction at arm swap instruction");
            return;
        }

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
// Start PSR transfer micro operations

/// Save PSR in rd
pub fn mrs(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        let (rd, psr);
        if let Some(decoded) = &instr.decoded_instruction {
            rd = decoded.rd.unwrap() as usize;
            psr = decoded.val1.unwrap();
        } else {
            eprintln!("Expected decoded instruction at arm mrs instruction");
            return;
        }

        let unpacked_psr = if psr == 0 {
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
        };

        cpu.arm.store_register(rd, unpacked_psr);
    }
}

/// Save rd in PSR
pub fn msr(cpu: &mut CPU) {
    if let InstructionType::ARM(instr) = &cpu.decoded_instruction {
        let (flags, rm, psr, imm);
        let mut shifted_imm = 0;

        if let Some(decoded) = &instr.decoded_instruction {
            rm = decoded.rd.unwrap() as usize;
            psr = decoded.val1.unwrap();
            imm = decoded.imm.unwrap();
            flags = decoded.offset.unwrap();
            if imm {
                let imm_val = decoded.val2.unwrap() as u32;
                // ror in steps of two (0-30)
                let imm_shift = decoded.val3.unwrap() as u32 * 2;

                shifted_imm = arm_ror(cpu, imm_val, imm_shift, false);
            }
        } else {
            eprintln!("Expected decoded instruction at arm msr instruction");
            return;
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

        let (rn, rd, set_cond);
        let op2: i32;
        let mnemonic: crate::enums::MnemonicARM;

        if let Some(decoded) = &instr.decoded_instruction {
            rn = cpu.arm.load_register(decoded.rn.unwrap() as usize) as i32;
            set_cond = decoded.set_cond.unwrap();
            rd = decoded.rn.unwrap() as usize;
            mnemonic = decoded.instr.clone();

            // implement the barrel shifter
            if decoded.imm.unwrap() {
                // the immediate shift can only represent even numbers, so we multiply it by 2.
                let shift = decoded.val1.unwrap() as u32 * 2;
                let imm_value = decoded.val2.unwrap() as u32;
                op2 = imm_value.rotate_right(shift) as i32;
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

                op2 = arm_barrelshifter(cpu, shift_type, set_cond, to_shift, shift_amount, rm);
            }
        } else {
            eprintln!("Expected decoded instruction at arm ALU");
            return;
        }

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
