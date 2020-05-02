use std::fs::File;
use std::io::{Read, Result};
use crate::enums::ShiftType;

/// Load a GBA binary ROM to a Vector.
pub fn read_rom_to_memory(rom_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(rom_path)?;
    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;
    Ok(rom)
}

/// Get bit in a certain position
pub fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        return input & (1 << n) != 0;
    }
    false
}

/// Get num of 1s in a u32
/// Example: count_set_bits(4) // 1
pub fn count_set_bits(n: u32) -> u32 {
    let mut n = n;
    let mut ret = 0;

    while n > 0 {
        ret += (n & 1) as u32;
        n >>= 1;
    }

    ret
}

/// Get n last bits from u32
pub fn get_last_bits(input: u32, n: u8) -> u32 {
    if n < 32 {
        return input & ((1 << n) - 1);
    }
    0
}

/// Transforms a number into its equivalent ShiftType variant.
pub fn get_shift_type(shift_type: u32) -> ShiftType {
    match shift_type {
        0 => ShiftType::LSL,
        1 => ShiftType::LSR,
        2 => ShiftType::ASR,
        3 => ShiftType::ROR,
        x => {
            eprintln!("unexpected shift type while decoding: {:?}", x);
            ShiftType::LSL
        }
    }
}
