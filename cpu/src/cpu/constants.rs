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
    pub const RS: [u32; 16] = [0; 16];
    // CPU starts at user mode, with FIQ and IRQ disabled by default
    pub const CPSR: u32 = 0b11000000;
    pub const SPSR: u32 = 0x0;
}

////////////// THUMB INSTRUCTION BITMASK CONSTANTS //////////////

pub mod thumb_move_shifted_register {
    pub const START: u16 = 0b000;
    pub const SHIFT_TO_START: u16 = 13;

    pub const OPCODE: u16 = 11;
    pub const OP_SHIFT_LEFT: u16 = 0b00;
    pub const OP_LOGICAL_SHIFT_RIGHT: u16 = 0b01;
    pub const OP_ARITHMETIC_SHIFT_RIGHT: u16 = 0b10;

    pub const OFFSET: u16 = 6;

    pub const SOURCE_REGISTER: u16 = 3;

    pub const DESTINATION_REGISTER: u16 = 0;
}

pub mod thumb_add_sub {
    pub const START: u16 = 0b00011;
    pub const SHIFT_TO_START: u16 = 11;

    pub const OPCODE: u16 = 9;
    pub const ADD_REGISTER: u16 = 0;
    pub const SUB_REGISTER: u16 = 1;
    pub const ADD_IMMEDIATE: u16 = 2;
    pub const SUB_IMMEDIATE: u16 = 3;

    pub const REGISTER_OPERAND_IMMEDIATE_VALUE: u16 = 6;

    pub const SOURCE_REGISTER: u16 = 3;

    pub const DESTINATION_REGISTER: u16 = 0;
}
