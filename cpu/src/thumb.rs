use crate::enums::{MnemonicARM, ShiftType};
use crate::{cpu::CPU, thumb_decode};
use std::{collections::VecDeque, default::Default};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ThumbDecodedInstruction {
    pub instr: MnemonicARM,

    pub rn: Option<u8>, // index register
    pub rm: Option<u8>, // second index register
    pub rd: Option<u8>, // destination register
    pub rs: Option<u8>, // source register

    pub val1: Option<u8>, // multi-purpose value (can be a shift to apply, etc)
    pub val2: Option<u8>, // ^

    pub offset: Option<i32>, // offset for branching

    pub shift_type: Option<ShiftType>, // 0=LSL, 1=LSR, 2=ASR, 3=ROR
    pub set_cond: Option<bool>,        // choose if should set condition codes
    pub imm: Option<bool>,             // whether the values come from registers or not
    pub acc: Option<bool>,             // whether the values should accumulate
}

pub fn decode_thumb(cpu: &mut CPU, instr: u16) -> VecDeque<fn(&mut CPU)> {

    unimplemented!();
}
