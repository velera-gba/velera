use memory::memory::MMU;
// move all of these enums and modules to a new file
enum Mode {
    Thumb,
    ARM,
}

mod cond_arm {
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

enum MnemonicARM {
    ILL,
    ADC,
    ADD,
    AND,
    ASR,
    B,
    BIC,
    BKPT,
    BL,
    BX,
    CMN,
    CMP,
    EOR,
    LDM,
    LDR,
    LSL,
    LSR,
    MLA,
    MOV,
    MRS,
    MSR,
    MUL,
    MVN,
    NEG,
    ORR,
    ROR,
    RSB,
    RSC,
    SBC,
    SMLAL,
    SMULL,
    STM,
    STR,
    SUB,
    SWI,
    SWP,
    TEQ,
    TST,
    UMLAL,
    UMULL,
    MAX,
}

pub struct ARM {
    mem: MMU,
}
