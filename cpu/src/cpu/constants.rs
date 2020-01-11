pub mod cond_arm {
    pub const ARM_CONDITION_EQ: u8 = 0x0;
    pub const ARM_CONDITION_NE: u8 = 0x1;
    pub const ARM_CONDITION_CS: u8 = 0x2;
    pub const ARM_CONDITION_CC: u8 = 0x3;
    pub const ARM_CONDITION_MI: u8 = 0x4;
    pub const ARM_CONDITION_PL: u8 = 0x5;
    pub const ARM_CONDITION_VS: u8 = 0x6;
    pub const ARM_CONDITION_VC: u8 = 0x7;
    pub const ARM_CONDITION_HI: u8 = 0x8;
    pub const ARM_CONDITION_LS: u8 = 0x9;
    pub const ARM_CONDITION_GE: u8 = 0xA;
    pub const ARM_CONDITION_LT: u8 = 0xB;
    pub const ARM_CONDITION_GT: u8 = 0xC;
    pub const ARM_CONDITION_LE: u8 = 0xD;
    pub const ARM_CONDITION_AL: u8 = 0xE;
    pub const ARM_CONDITION_NV: u8 = 0xF;
}

pub mod registers {
    pub const STACK_POINTER: usize = 13;
    pub const LINK_REGISTER: usize = 14;
    pub const PROGRAM_COUNTER: usize = 15;
}

pub mod cpsr_flags {
    pub const SIGNED: u8 = 31;
    pub const ZERO: u8 = 30;
    pub const CARRY: u8 = 29;
    pub const OVERFLOW: u8 = 28;
    pub const STICKY_OVERFLOW: u8 = 27;
    pub const IRQ_DISABLE: u8 = 7;
    pub const FIQ_DISABLE: u8 = 6;
    pub const STATE_BIT: u8 = 5;
    pub const MODE4: u8 = 4;
    pub const MODE3: u8 = 3;
    pub const MODE2: u8 = 2;
    pub const MODE1: u8 = 1;
    pub const MODE0: u8 = 0;
}

pub mod cpu_modes {
    pub const USER: u8 = 0b10000;
    pub const FIQ: u8 = 0b10001;
    pub const IRQ: u8 = 0b10010;
    pub const SUPERVISOR: u8 = 0b10011;
    pub const ABORT: u8 = 0b10111;
    pub const UNDEFINED: u8 = 0b11011;
    pub const SYSTEM: u8 = 0b11111;
}

pub mod default_cpu {
    pub const MMU_DISPLAY: u32 = 1;
    pub const RS: [i32; 16] = [0; 16];
    // CPU starts at user mode, with FIQ and IRQ disabled by default
    pub const CPSR: u32 = 0b11000000;
    pub const SPSR: u32 = 0x0;
}

////////////// THUMB INSTRUCTION BITMASK CONSTANTS //////////////

pub mod thumb_bitmasks {
    // thumb 1: move shifted register
    pub const LSL: u16 = 0b0000000000000000;
    pub const LSR: u16 = 0b0000100000000000;
    pub const ASR: u16 = 0b0001000000000000;
    pub const MOVE_SHIFTED_REG_OP_MASK: u16 = 0b1111100000000000;
    pub const MOVE_SHIFTED_REG_RS_MASK: u16 = 0b0000000000111000;
    pub const MOVE_SHIFTED_REG_RD_MASK: u16 = 0b0000000000000111;
    pub const MOVE_SHIFTED_REG_OFFSET_MASK: u16 = 0b000011111000000;

    // thumb 2: add/subtract
    pub const ADD: u16 = 0b0001100000000000;
    pub const SUB: u16 = 0b0001101000000000;
    pub const ADDI: u16 = 0b0001110000000000;
    pub const SUBI: u16 = 0b0001111000000000;
    pub const ADDSUB_OP_MASK: u16 = 0b1111111000000000;
    pub const ADDSUB_RN_MASK: u16 = 0b0000000111000000;
    pub const ADDSUB_RS_MASK: u16 = 0b0000000000111000;
    pub const ADDSUB_RD_MASK: u16 = 0b0000000000000111;

    // thumb 3: move/compare/add/subtract immediate
    pub const MOV: u16 = 0b001000000000000;
    pub const CMP: u16 = 0b001010000000000;
    pub const ADDRI: u16 = 0b001100000000000;
    pub const SUBRI: u16 = 0b001110000000000;
    pub const IMMEDIATE_OP_MASK: u16 = 0b1111100000000000;
    pub const IMMEDIATE_RD_MASK: u16 = 0b0000011100000000;
    pub const IMMEDIATE_NN_MASK: u16 = 0b0000000011111111;
    
    // thumb 4: ALU ops
    pub const ALU_AND: u16 = 0b0100000000000000;
    pub const ALU_EOR: u16 = 0b0100000001000000;
    pub const ALU_LSL: u16 = 0b0100000010000000;
    pub const ALU_LSR: u16 = 0b0100000011000000;
    pub const ALU_ASR: u16 = 0b0100000100000000;
    pub const ALU_ADC: u16 = 0b0100000101000000;
    pub const ALU_SBC: u16 = 0b0100000110000000;
    pub const ALU_ROR: u16 = 0b0100000111000000;
    pub const ALU_TST: u16 = 0b0100001000000000;
    pub const ALU_NEG: u16 = 0b0100001001000000;
    pub const ALU_CMP: u16 = 0b0100001010000000;
    pub const ALU_CMN: u16 = 0b0100001011000000;
    pub const ALU_ORR: u16 = 0b0100001100000000;
    pub const ALU_MUL: u16 = 0b0100001101000000;
    pub const ALU_BIC: u16 = 0b0100001110000000;
    pub const ALU_MVN: u16 = 0b0100001111000000;
    pub const ALU_OP_MASK: u16 = 0b1111111111000000;
    pub const ALU_RS_MASK: u16 = 0b0000000000111000;
    pub const ALU_RD_MASK: u16 = 0b0000000000000111;
    
    // thumb 5: Hi register operations/branch exchange
    pub const HI_ADD: u16 = 0b0100010000000000;
    pub const HI_CMP: u16 = 0b0100010100000000;
    pub const HI_MOV: u16 = 0b0100011000000000;
    pub const HI_NOP: u16 = 0b0100011011000000;
    pub const BX: u16 = 0b0100011100000000;
    pub const BLX: u16 = 0b0100011110000000;
    pub const HI_OP_MASK: u16 = 0b1111111100000000;
    pub const HI_MSBD_MASK: u16 = 0b0000000010000000;
    pub const HI_MSBS_MASK: u16 = 0b0000000001000000;
    pub const HI_RS: u16 = 0b0000000000111000;
    pub const HI_RD: u16 = 0b0000000000000111;

    // thumb 6: load PC-relative
    pub const LDPCR: u16 = 0b0100100000000000;
    pub const LDPCR_MASK: u16 = 0b1111100000000000;
    pub const LDPCR_RD: u16 = 0b0000011100000000;
    pub const LDPCR_OFFSET: u16 = 0b0000000011111111;

    // thumb 7: load/store with register offset
    pub const STR: u16 = 0b0101000000000000;
    pub const STRB: u16 = 0b0101010000000000;
    pub const LDR: u16 = 0b0101100000000000;
    pub const LDRB: u16 = 0b0101110000000000;
    pub const LS_REG_OFFSET_OPCODE_MASK: u16 = 0b1111111000000000;
    pub const LS_REG_OFFSET_RO_MASK: u16 = 0b0000000111000000;
    pub const LS_REG_OFFSET_RB_MASK: u16 = 0b0000000000111000;
    pub const LS_REG_OFFSET_RD_MASK: u16 = 0b0000000000000111;

    // thumb 8: load/store sign-extended byte/halfword
    pub const STRH: u16 = 0b0101001000000000;
    pub const LDSB: u16 = 0b0101011000000000;
    pub const LDRH: u16 = 0b0101101000000000;
    pub const LDSH: u16 = 0b0101111000000000;
    pub const LS_EBH_OP_MASK: u16 = 0b1111111000000000;
    pub const LS_EBH_RO_MASK: u16 = 0b0000000111000000;
    pub const LS_EBH_RB_MASK: u16 = 0b0000000000111000;
    pub const LS_EBH_RD_MASK: u16 = 0b0000000000000111;

    // thumb 9: load/store with immediate offset
    pub const STRI: u16 = 0b0110000000000000;
    pub const LDRI: u16 = 0b0110100000000000;
    pub const STRBI: u16 = 0b0111000000000000;
    pub const LDRBI: u16 = 0b0111100000000000;
    pub const LS_NN_OFFSET_OP_MASK: u16 = 0b1111100000000000;
    pub const LS_NN_OFFSET_NN_MASK: u16 = 0b0000011111000000;
    pub const LS_NN_OFFSET_RB_MASK: u16 = 0b0000000000111000;
    pub const LS_NN_OFFSET_RD_MASK: u16 = 0b0000000000000111;

    // thumb 10: load/store halfword
    pub const STRHW: u16 = 0b1000000000000000;
    pub const LDRHW: u16 = 0b1000100000000000;
    pub const LS_HW_OP_MASK: u16 = 0b1111100000000000;
    pub const LS_HW_NN_MASK: u16 = 0b0000011111000000;
    pub const LS_HW_RB_MASK: u16 = 0b0000000000111000;
    pub const LS_HW_RD_MASK: u16 = 0b0000000000000111;

    // thumb 11: load/store SP-relative
    pub const SP_STR: u16 = 0b1001000000000000;
    pub const SP_LDR: u16 = 0b1001100000000000;
    pub const SP_LS_OP_MASK: u16 = 0b1111100000000000;
    pub const SP_LS_RD_MASK: u16 = 0b0000011100000000;
    pub const SP_LS_NN_MASK: u16 = 0b0000000011111111;

    // thumb 12: get relative address
    pub const ADD_PC: u16 = 0b1010000000000000;
    pub const ADD_SP: u16 = 0b1010100000000000;
    pub const RELATIVE_ADDR_OP_MASK: u16 = 0b1111100000000000;
    pub const RELATIVE_ADDR_RD_MASK: u16 = 0b0000011100000000;
    pub const RELATIVE_ADDR_NN_MASK: u16 = 0b0000000011111111;

    // thumb 13: add offset to stack pointer
    pub const ADD_SP_NN: u16 = 0b1011000000000000;
    pub const ADD_SP_MINUS_NN: u16 = 0b1011000010000000;
    pub const SP_OFFSET_OP_MASK: u16 = 0b1111111110000000;
    pub const SP_OFFSET_NN_MASK: u16 = 0b0000000001111111;

    // thumb 14: push/pop registers
    pub const PUSH: u16 = 0b1011010000000000;
    pub const POP: u16 = 0b1011100000000000;
    pub const STACK_OPS_OP_MASK: u16 = 0b1111111000000000;
    pub const STACK_OPS_PC_LR_BIT_MASK: u16 = 0b0000000100000000;
    pub const STACK_OPS_RLIST_MASK: u16 = 0b0000000011111111;

    // thumb 15: multiple load/store
    pub const STMIA: u16 = 0b1100000000000000;
    pub const LDMIA: u16 = 0b1100100000000000;
    pub const LS_MIA_OP_MASK: u16 = 0b1111100000000000;
    pub const LS_MIA_RB_MASK: u16 = 0b0000011100000000;
    pub const LS_MIA_RLIST_MASK: u16 = 0b0000000011111111;

    // thumb 16: conditional branch
    pub const COND_BRANCH_OP: u16 = 0b1101000000000000;
    pub const COND_GENERAL_OP_MASK: u16 = 0b1111000000000000;
    pub const COND_FULL_OP_MASK: u16 = 0b1111111100000000;
    pub const COND_OFFSET_MASK: u16 = 0b0000000011111111;

    // thumb 17: software interrupt and breakpoint
    pub const SWI: u16 = 0b1101111100000000;
    pub const BKPT: u16 = 0b1101111000000000;
    pub const SWI_BK_OP_MASK: u16 = 0b1111111100000000;
    pub const SWI_BK_NN_MASK: u16 = 0b0000000011111111;

    // thumb 18: unconditional branch
    pub const B: u16 = 0b1110000000000000;
    pub const B_OP_MASK: u16 = 0b1111100000000000;
    pub const B_OFFSET_MASK: u16 = 0b0000011111111111;

    // thumb 19: long branch with link
    pub const LONG_BRANCH_FIRST_OP: u16 = 0b1111000000000000;
    pub const BL: u16 = 0b1111100000000000;
    pub const BLLX: u16 = 0b1111000000000000;
    pub const LONG_BRANCH_OP_MASK: u16 = 0b1111100000000000;
    pub const LONG_BRANCH_ADDR_MASK: u16 = 0b0000011111111111;
}

pub mod arm_bitmasks {

}