// this section would not be possible without the help of mGBA's source
// and https://www.akkit.org/info/gbatek.htm.
// thank you!
use velera_graphics::{Display, Memory};

pub mod sizes {
    pub const BIOS_SIZE: usize = 0x0004000;
    pub const WRAM_SIZE: usize = 0x0040000;
    pub const IWRAM_SIZE: usize = 0x0008000;
    pub const IO_REGISTERS_SIZE: usize = 0x0000400;
    pub const PALETTE_RAM_SIZE: usize = 0x0000400;
    pub const VRAM_SIZE: usize = 0x0018000;
    pub const OAM_SIZE: usize = 0x0000400;
    pub const CART0_SIZE: usize = 0x2000000;
    pub const CART1_SIZE: usize = 0x2000000;
    pub const CART2_SIZE: usize = 0x2000000;
    pub const CART_SRAM_SIZE: usize = 0x0008000;
    pub const CART_FLASH512_SIZE: usize = 0x0010000;
    pub const CART_FLASH1M_SIZE: usize = 0x0020000;
    pub const CART_EEPROM_SIZE: usize = 0x0002000;
    pub const CART_EEPROM512_SIZE: usize = 0x0000200;
}

pub mod base_addrs {
    pub const BIOS_ADDR: usize = 0x0000000;
    pub const WORKING_RAM_ADDR: usize = 0x2000000;
    pub const WORKING_IRAM_ADDR: usize = 0x3000000;
    pub const IO_REGISTERS_ADDR: usize = 0x4000000;
    pub const PALETTE_RAM_ADDR: usize = 0x5000000;
    pub const VRAM_ADDR: usize = 0x6000000;
    pub const OAM_ADDR: usize = 0x7000000;
    pub const CART0_ADDR: usize = 0x8000000;
    pub const CART0_EX_ADDR: usize = 0x9000000;
    pub const CART1_ADDR: usize = 0xA000000;
    pub const CART1_EX_ADDR: usize = 0xB000000;
    pub const CART2_ADDR: usize = 0xC000000;
    pub const CART2_EX_ADDR: usize = 0xD000000;
    pub const CART_SRAM_ADDR: usize = 0xE000000;
    pub const CART_SRAM_MIRROR_ADDR: usize = 0xF000000;
}

struct MMU {
    wram: Box<[u8; sizes::WRAM_SIZE]>,
    iwram: Box<[u8; sizes::IWRAM_SIZE]>,
    bios: Box<[u8; sizes::BIOS_SIZE]>,
    rom: Box<[u8; sizes::CART0_SIZE * 3]>,
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
    pub fn rb(&self, addr: u32, val: u8) -> u8 {
        return match addr as usize {
            base_addrs::WORKING_RAM_ADDR..=0x0203FFFF => {
                self.wram[addr as usize - base_addrs::WORKING_RAM_ADDR]
            }
            base_addrs::WORKING_IRAM_ADDR..=0x03007FFF => {
                self.iwram[addr as usize - base_addrs::WORKING_RAM_ADDR]
            }
            base_addrs::IO_REGISTERS_ADDR..=0x08000000 => self.gpu_mem.read(addr),
            base_addrs::CART0_ADDR..=0x0DFFFFFF => self.rom[addr as usize - base_addrs::CART0_ADDR],
            base_addrs::CART_SRAM_ADDR..=0x0E00FFFF => 0, // SRAM unimplemented
            base_addrs::CART_SRAM_MIRROR_ADDR..=0x0F00FFFF => 0, // SRAM unimplemented
            _ => 0,
        };
    }

    /// Write a byte into memory
    pub fn wb(&mut self, addr: u32, val: u8) {
        match addr as usize {
            0x0..=base_addrs::BIOS_ADDR => self.bios[addr as usize] = val,
            base_addrs::WORKING_RAM_ADDR..=0x0203FFFF => {
                self.wram[addr as usize - base_addrs::WORKING_RAM_ADDR] = val
            }
            base_addrs::WORKING_IRAM_ADDR..=0x03007FFF => {
                self.iwram[addr as usize - base_addrs::WORKING_IRAM_ADDR] = val
            }
            base_addrs::IO_REGISTERS_ADDR..=0x08000000 => self.gpu_mem.write(addr, val),
            base_addrs::CART0_ADDR..=0x0DFFFFFF => (), // writing in ROM
            base_addrs::CART_SRAM_ADDR..=0x0E00FFFF => (), // SRAM unimplemented
            base_addrs::CART_SRAM_MIRROR_ADDR..=0x0F00FFFF => (),

            _ => {}
        }
    }

    /// Write an aligned word into memory
    #[allow(overflowing_literals)]
    pub fn ww(&mut self, addr: u32, val: u32) {
        self.wb(addr, val as u8);
        self.wb(addr + 1, (val >> 8) as u8);
        self.wb(addr + 2, (val >> 16) as u8);
        self.wb(addr + 3, (val >> 24) as u8);
    }
}
