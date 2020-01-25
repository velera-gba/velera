use crate::arm;

/// Possible ARM processor modes
#[derive(Clone)]
pub enum InstructionType {
    Thumb(u16),
    ARM(arm::ARMInstruction),
}

/// List of ARM instruction mnemonics
#[derive(Debug, PartialEq, Clone)]
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
    LDRH,
    LDRSB,
    LDRSH,
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
    STRH,
    SUB,
    SWI,
    SWP,
    TEQ,
    TST,
    UMLAL,
    UMULL,
    MAX,
}

impl Default for MnemonicARM {
    #[inline]
    fn default() -> Self {
        Self::ILL
    }
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
