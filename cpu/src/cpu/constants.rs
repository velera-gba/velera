pub mod cond_arm {
    const ARM_CONDITION_EQ: u8 = 0x0;
    const ARM_CONDITION_NE: u8 = 0x1;
    const ARM_CONDITION_CS: u8 = 0x2;
    const ARM_CONDITION_CC: u8 = 0x3;
    const ARM_CONDITION_MI: u8 = 0x4;
    const ARM_CONDITION_PL: u8 = 0x5;
    const ARM_CONDITION_VS: u8 = 0x6;
    const ARM_CONDITION_VC: u8 = 0x7;
    const ARM_CONDITION_HI: u8 = 0x8;
    const ARM_CONDITION_LS: u8 = 0x9;
    const ARM_CONDITION_GE: u8 = 0xA;
    const ARM_CONDITION_LT: u8 = 0xB;
    const ARM_CONDITION_GT: u8 = 0xC;
    const ARM_CONDITION_LE: u8 = 0xD;
    const ARM_CONDITION_AL: u8 = 0xE;
    const ARM_CONDITION_NV: u8 = 0xF;
}

pub mod registers {
    const STACK_POINTER: u8 = 13;
    const LINK_REGISTER: u8 = 14;
    const PROGRAM_COUNTER: u8 = 15;
}

pub mod cpsr_flags {
    const SIGNED: u8 = 31;
    const ZERO: u8 = 30;
    const CARRY: u8 = 29;
    const OVERFLOW: u8 = 28;
    const STICKY_OVERFLOW: u8 = 27;
    const IRQ_DISABLE: u8 = 7;
    const FIQ_DISABLE: u8 = 6;
    const STATE_BIT: u8 = 5;
    const MODE4: u8 = 4;
    const MODE3: u8 = 3;
    const MODE2: u8 = 2;
    const MODE1: u8 = 1;
    const MODE0: u8 = 0;
}

pub mod cpu_modes {
    const USER: u8 = 0b10000;
    const FIQ: u8 = 0b10001;
    const IRQ: u8 = 0b10010;
    const SUPERVISOR: u8 = 0b10011;
    const ABORT: u8 = 0b10111;
    const UNDEFINED: u8 = 0b11011;
    const SYSTEM: u8 = 0b11111;
}