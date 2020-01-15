/// Possible ARM processor modes
pub enum InstructionType {
    Thumb(u16),
    ARM(u32),
}

/// List of ARM instruction mnemonics
pub enum MnemonicARM {
    ILL, // illegal
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

/// Thumb instruction mode bitmask constants.
pub enum ThumbFirst3Bits {
    ShiftAddSub,
    Immediate,
    AluHighRegOps,
    LoadStoreImmediateOffset,
    LoadStoreHalfwordSP,
    RelativeAddrStackOps,
    MultiLoadStoreCondBranchSWI,
    UncondBranch,
}