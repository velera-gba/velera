use std::fs::File;
use std::io::{Read, Result};

/// Load a GBA binary ROM to a Vector.
pub fn read_rom_to_memory(rom_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(rom_path)?;
    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;
    Ok(rom)
}
