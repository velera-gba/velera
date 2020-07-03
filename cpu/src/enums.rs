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
    ADDPC,
    ADDSP,
    ADDI,
    ADDHI,
    AND,
    ASRALU,
    B,
    BCond,
    BKPT,
    BIC,
    BL,
    BX,
    CMN,
    CMP,
    CMPI,
    CMPHI,
    EOR,
    LDM,
    LDMIA,
    LDR,
    LDRB,
    LDRPC,
    LDRH,
    LDRSB,
    LDRSH,
    LDRSPREL,
    LDSB,
    LDSH,
    LSLALU,
    LSRALU,
    MLA,
    MOV,
    MOVHI,
    MOVI,
    MRS,
    MSR,
    MUL,
    MVN,
    NEG,
    ORR,
    POP,
    POPPC,
    PUSH,
    PUSHLR,
    ROR,
    RORALU,
    RSB,
    RSC,
    SBC,
    SHIFT,
    SMLAL,
    SMULL,
    STM,
    STMIA,
    STR,
    STRB,
    STRH,
    STRSPREL,
    SUB,
    SUBI,
    SWI,
    SWP,
    TEQ,
    TST,
    UMLAL,
    UMULL,
    OffsetADD,
    OffsetSUB,
    Undefined,
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

#[derive(Debug, PartialEq, Clone)]
pub enum ShiftType {
    LSL,
    LSR,
    ASR,
    ROR,
}

impl Default for ShiftType {
    #[inline]
    fn default() -> Self {
        Self::LSL
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProcessorMode {
    User,
    FIQ,
    IRQ,
    Supervisor,
    Abort,
    Undefined,
    System,
}

impl Default for ProcessorMode {
    #[inline]
    fn default() -> Self {
        Self::User
    }
}
