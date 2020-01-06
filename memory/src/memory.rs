// this section would not be possible without the help of mGBA's source
// and https://www.akkit.org/info/gbatek.htm.
// thank you!
use graphics::{Display, Memory};

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

pub struct MMU {
    wram: Box<[u8]>,
    iwram: Box<[u8]>,
    bios: Box<[u8]>,
    rom: Box<[u8]>,
    gpu_mem: Memory,
    gpu: Display,
}

impl MMU {
    /// Create a new instance of the MMU
    pub fn new(display_scale: u32) -> Result<Self, String> {
        let (gpu_mem, gpu) = Display::init(display_scale)?;
        Ok(Self {
            bios: Box::new([0; sizes::BIOS_SIZE]),
            wram: Box::new([0; sizes::WRAM_SIZE]),
            iwram: Box::new([0; sizes::IWRAM_SIZE]),
            rom: Box::new([0; sizes::CART0_SIZE * 3]),
            gpu,
            gpu_mem,
        })
    }

    /// Reads a byte from memory
    pub fn load8(&self, addr: u32) -> u8 {
        match addr as usize {
            base_addrs::WORKING_RAM_ADDR..=0x0203_FFFF => {
                self.wram[addr as usize - base_addrs::WORKING_RAM_ADDR]
            }
            base_addrs::WORKING_IRAM_ADDR..=0x0300_7FFF => {
                self.iwram[addr as usize - base_addrs::WORKING_RAM_ADDR]
            }
            base_addrs::IO_REGISTERS_ADDR..=0x07FF_FFFF => self.gpu_mem.read(addr),
            base_addrs::CART0_ADDR..=0x0DFF_FFFF => {
                self.rom[addr as usize - base_addrs::CART0_ADDR]
            }
            base_addrs::CART_SRAM_ADDR..=0x0E00_FFFF => 0, // SRAM unimplemented
            base_addrs::CART_SRAM_MIRROR_ADDR..=0x0F00_FFFF => 0, // SRAM unimplemented
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
            0x0..=base_addrs::BIOS_ADDR => self.bios[addr as usize] = val,
            base_addrs::WORKING_RAM_ADDR..=0x0203_FFFF => {
                self.wram[addr as usize - base_addrs::WORKING_RAM_ADDR] = val
            }
            base_addrs::WORKING_IRAM_ADDR..=0x0300_7FFF => {
                self.iwram[addr as usize - base_addrs::WORKING_IRAM_ADDR] = val
            }
            base_addrs::IO_REGISTERS_ADDR..=0x07FF_FFFF => self.gpu_mem.write(addr, val),
            base_addrs::CART0_ADDR..=0x0DFF_FFFF => (), // writing in ROM
            base_addrs::CART_SRAM_ADDR..=0x0E00_FFFF => (), // SRAM unimplemented
            base_addrs::CART_SRAM_MIRROR_ADDR..=0x0F00_FFFF => (),

            _ => {}
        }
    }

    /// Write an aligned half-word into memory
    #[allow(overflowing_literals)]
    pub fn store16(&mut self, addr: u32, val: u32) {
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
