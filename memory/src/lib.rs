// this section would not be possible without the help of mGBA's source
// and https://www.akkit.org/info/gbatek.htm.
// thank you!

pub mod sizes {
    pub const BIOS_SIZE: usize = 0x000_4000;
    pub const WRAM_SIZE: usize = 0x004_0000;
    pub const IWRAM_SIZE: usize = 0x000_8000;
    pub const IO_REGISTERS_SIZE: usize = 0x000_0400;
    pub const PALETTE_RAM_SIZE: usize = 0x000_0400;
    pub const VRAM_SIZE: usize = 0x001_8000;
    pub const OAM_SIZE: usize = 0x000_0400;
    pub const CART0_SIZE: usize = 0x200_0000;
    pub const CART1_SIZE: usize = 0x200_0000;
    pub const CART2_SIZE: usize = 0x200_0000;
    pub const CART_SRAM_SIZE: usize = 0x000_8000;
    pub const CART_FLASH512_SIZE: usize = 0x001_0000;
    pub const CART_FLASH1M_SIZE: usize = 0x002_0000;
    pub const CART_EEPROM_SIZE: usize = 0x000_2000;
    pub const CART_EEPROM512_SIZE: usize = 0x000_0200;
}

pub mod base_addrs {
    pub const BIOS_ADDR: usize = 0x000_0000;
    pub const WORKING_RAM_ADDR: usize = 0x200_0000;
    pub const WORKING_IRAM_ADDR: usize = 0x300_0000;
    pub const IO_REGISTERS_ADDR: usize = 0x400_0000;
    pub const PALETTE_RAM_ADDR: usize = 0x500_0000;
    pub const VRAM_ADDR: usize = 0x600_0000;
    pub const OAM_ADDR: usize = 0x700_0000;
    pub const CART0_ADDR: usize = 0x800_0000;
    pub const CART0_EX_ADDR: usize = 0x900_0000;
    pub const CART1_ADDR: usize = 0xA00_0000;
    pub const CART1_EX_ADDR: usize = 0xB00_0000;
    pub const CART2_ADDR: usize = 0xC00_0000;
    pub const CART2_EX_ADDR: usize = 0xD00_0000;
    pub const CART_SRAM_ADDR: usize = 0xE00_0000;
    pub const CART_SRAM_MIRROR_ADDR: usize = 0xF00_0000;
}

#[derive(Default, Clone)]
pub struct MMU {
    wram: Box<[u8]>,
    iwram: Box<[u8]>,
    bios: Box<[u8]>,
    rom: Box<[u8]>,
    registers: Box<[u8]>,
    palette: Box<[u8]>,
    vram: Box<[u8]>,
    oam: Box<[u8]>,
}

impl MMU {
    /// Create a new instance of the MMU
    pub fn new() -> Self {
        Self {
            bios: vec![0; sizes::BIOS_SIZE].into_boxed_slice(),
            wram: vec![0; sizes::WRAM_SIZE].into_boxed_slice(),
            iwram: vec![0; sizes::IWRAM_SIZE].into_boxed_slice(),
            rom: vec![0; sizes::CART0_SIZE].into_boxed_slice(),
            registers: vec![0; sizes::IO_REGISTERS_SIZE].into_boxed_slice(),
            palette: vec![0; sizes::PALETTE_RAM_SIZE].into_boxed_slice(),
            vram: vec![0; sizes::VRAM_SIZE].into_boxed_slice(),
            oam: vec![0; sizes::OAM_SIZE].into_boxed_slice(),
        }
    }

    /// Reads a byte from memory
    pub fn load8(&self, addr: u32) -> u8 {
        match addr as usize {
            base_addrs::BIOS_ADDR..=0x0000_3FFF => self.bios[addr as usize - base_addrs::BIOS_ADDR],
            base_addrs::WORKING_RAM_ADDR..=0x0203_FFFF => {
                self.wram[addr as usize - base_addrs::WORKING_RAM_ADDR]
            }
            base_addrs::WORKING_IRAM_ADDR..=0x0300_7FFF => {
                self.iwram[addr as usize - base_addrs::WORKING_IRAM_ADDR]
            }
            base_addrs::IO_REGISTERS_ADDR..=0x0400_03FE => {
                self.registers[addr as usize - base_addrs::IO_REGISTERS_ADDR]
            }
            base_addrs::PALETTE_RAM_ADDR..=0x0500_03FF => {
                self.palette[addr as usize - base_addrs::PALETTE_RAM_ADDR]
            }
            base_addrs::VRAM_ADDR..=0x0601_7FFF => self.vram[addr as usize - base_addrs::VRAM_ADDR],
            base_addrs::OAM_ADDR..=0x0700_03FF => self.oam[addr as usize - base_addrs::OAM_ADDR],
            base_addrs::CART0_ADDR..=0x0DFF_FFFF => {
                self.rom[addr as usize - base_addrs::CART0_ADDR]
            }
            base_addrs::CART_SRAM_ADDR..=0x0E00_FFFF => unimplemented!(), // SRAM unimplemented
            base_addrs::CART_SRAM_MIRROR_ADDR..=0x0F00_FFFF => unimplemented!(), // SRAM unimplemented
            _ => 0,
        }
    }

    /// Reads a half-word from memory
    pub fn load16(&self, addr: u32) -> u16 {
        let mut x = self.load8(addr) as u16;
        x <<= 8;
        x += self.load8(addr + 1) as u16;

        x
    }

    /// Reads a word from memory
    pub fn load32(&self, addr: u32) -> u32 {
        let mut x = self.load16(addr) as u32;
        x <<= 16;
        x += self.load16(addr + 2) as u32;

        x
    }

    /// Write a byte into memory
    pub fn store8(&mut self, addr: u32, val: u8) {
        match addr as usize {
            base_addrs::BIOS_ADDR..=0x0000_3FFF => {
                self.bios[addr as usize - base_addrs::BIOS_ADDR] = val
            }
            base_addrs::WORKING_RAM_ADDR..=0x0203_FFFF => {
                self.wram[addr as usize - base_addrs::WORKING_RAM_ADDR] = val
            }
            base_addrs::WORKING_IRAM_ADDR..=0x0300_7FFF => {
                self.iwram[addr as usize - base_addrs::WORKING_IRAM_ADDR] = val
            }
            base_addrs::IO_REGISTERS_ADDR..=0x0400_03FE => {
                self.registers[addr as usize - base_addrs::IO_REGISTERS_ADDR] = val
            }
            base_addrs::PALETTE_RAM_ADDR..=0x0500_03FF => {
                self.palette[addr as usize - base_addrs::PALETTE_RAM_ADDR] = val
            }
            base_addrs::VRAM_ADDR..=0x0601_7FFF => {
                self.vram[addr as usize - base_addrs::VRAM_ADDR] = val
            }
            base_addrs::OAM_ADDR..=0x0700_03FF => {
                self.oam[addr as usize - base_addrs::OAM_ADDR] = val
            }
            base_addrs::CART0_ADDR..=0x0DFF_FFFF => {
                self.rom[addr as usize - base_addrs::CART0_ADDR] = val
            }
            base_addrs::CART_SRAM_ADDR..=0x0E00_FFFF => unimplemented!(), // SRAM unimplemented
            base_addrs::CART_SRAM_MIRROR_ADDR..=0x0F00_FFFF => unimplemented!(), // SRAM unimplemented
            _ => (), // Consider making this an error
        }
    }

    /// Write an aligned half-word into memory
    #[allow(overflowing_literals)]
    pub fn store16(&mut self, addr: u32, val: u16) {
        self.store8(addr, val as u8);
        self.store8(addr + 1, (val >> 8) as u8);
    }

    /// Write an aligned word into memory
    #[allow(overflowing_literals)]
    pub fn store32(&mut self, addr: u32, val: u32) {
        self.store8(addr, val as u8);
        self.store8(addr + 1, (val >> 8) as u8);
        self.store8(addr + 2, (val >> 16) as u8);
        self.store8(addr + 3, (val >> 24) as u8);
    }
}
