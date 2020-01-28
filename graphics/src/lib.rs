/*#[cfg(not(feature = "fbdev"))]
mod graphics;

#[cfg(feature = "fbdev")]
mod fb_graphics;

#[cfg(not(feature = "fbdev"))]
use self::graphics::Graphics;
#[cfg(not(feature = "fbdev"))]
pub use self::graphics::{CacheInstance, CacheObject, Interrupt, State};

#[cfg(feature = "fbdev")]
use self::fb_graphics::Graphics;
#[cfg(feature = "fbdev")]
pub use self::fb_graphics::{CacheInstance, CacheObject, Interrupt, State};*/

mod backend;
use backend::*;

pub struct Memory {
    pub palette: Box<[u8]>,
    pub vram: Box<[u8]>,
    pub oam: Box<[u8]>,

    pub lcd: Box<[u8]>,
    pub keypad: Box<[u8]>,
}

impl Memory {
    /// Read an address from memory and returns it. Called by the MMU.
    /// Assumes address is in the internal display memory map (04000000 - 08000000).
    pub fn read(&self, addr: u32) -> u8 {
        match addr as usize {
            0x0400_0000..=0x0400_0056 => self.lcd[(addr - 0x0400_0000) as usize],
            0x0400_0130..=0x0400_0132 => self.keypad[(addr - 0x0400_0130) as usize],
            0x0500_0000..=0x0500_03FF => self.palette[(addr - 0x0500_0000) as usize],
            0x0600_0000..=0x0601_7FFF => self.vram[(addr - 0x0600_0000) as usize],
            0x0700_0000..=0x0700_03FF => self.oam[(addr - 0x0700_03FF) as usize],
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u32, val: u8) {
        match addr as usize {
            0x0400_0000..=0x0400_0056 => self.lcd[(addr - 0x0400_0000) as usize] = val,
            0x0400_0132 => self.keypad[2] = val,
            0x0500_0000..=0x0500_03FF => self.palette[(addr - 0x0500_0000) as usize] = val,
            0x0600_0000..=0x0601_7FFF => self.vram[(addr - 0x0600_0000) as usize] = val,
            0x0700_0000..=0x0700_03FF => self.oam[(addr - 0x0700_03FF) as usize] = val,
            _ => (),
        }
    }
}

/// Indicates when an interrupt is fired
pub struct Interrupt {
    pub vblank: bool,
    pub vcounter: bool,
    pub hblank: bool,
}

impl Interrupt {
    pub const fn none() -> Self {
        Self {
            vblank: false,
            vcounter: false,
            hblank: false,
        }
    }

    pub fn vblank(&mut self) {
        self.vblank = true
    }
    pub fn vcounter(&mut self) {
        self.vcounter = true
    }
    pub fn hblank(&mut self) {
        self.hblank = true
    }
}

/// Propogates meta state to the CPU module
pub enum State {
    Exited,
    Running,
}

/// Emulates the functionality of the GBA display and keypad hardware
///
/// Usage:
/// ```rust
/// ```
///
/// The memory struct contains boxed slices of the displays memory segments.
/// The registers module contains convenience constants and a local function for getting the memory address relative to the boxed slice
pub struct Display {
    backend: Backend,

    // There is no register for this so count here
    hcount: usize,
}

impl Display {
    pub fn init(scale: u32) -> Result<(Memory, Self), String> {
        let backend = backend::Backend::setup(scale)?;

        Ok((
            Memory {
                palette: vec![0; 1024].into_boxed_slice(),
                vram: vec![0; 96 * 1024].into_boxed_slice(),
                oam: vec![0; 1024].into_boxed_slice(),
                lcd: vec![0; 0x56].into_boxed_slice(),
                keypad: vec![0; 4].into_boxed_slice(),
            },
            Self {
                backend,
                hcount: 0
            },
        ))
    }

    /// A graphics cycle is done every 4 cpu cycles
    pub fn cycle<'r>(&mut self, memory: &mut Memory) -> (State, Interrupt) {
        let mut interrupts = Interrupt::none();

        // Only bits 0-7 are used of this register
        let mut vcount = memory.read(registers::VCOUNT) as usize;
        let vcount_setting = (memory.read(registers::DISPSTAT) >> 7) as usize;
        let vblank = vcount > 160;
        let hblank = self.hcount > 240;

        if !vblank && !hblank {
            // Generate draw closure based on video mode
            let pixel = match memory.lcd[registers::local(registers::DISPCNT)] & 0b111 {
                0 => unimplemented!(),
                1 => unimplemented!(),
                2 => unimplemented!(),
                3 => BGR555::from([memory.vram[(self.hcount * 2) + vcount * SCREEN_WIDTH * 2], memory.vram[(self.hcount * 2) + vcount * SCREEN_WIDTH * 2 + 1]]),
                4 => unimplemented!(),
                5 => unimplemented!(),
                6 | 7 => panic!("Program attempted to use undefined video mode"),
                _ => unreachable!(),
            };

            self.backend.draw_pixel((self.hcount, vcount), pixel.into())
        }

        // Increment the hcount
        self.hcount = if self.hcount < 307 {
            // Set hblank flag
            memory.write(
                registers::DISPSTAT,
                memory.read(registers::DISPSTAT) | 0b10u8,
            );
            // Check if hblank IRQ is set
            if memory.read(registers::DISPSTAT) & 0b10000u8 != 0 {
                interrupts.hblank()
            };
            self.hcount + 1
        } else {
            // Unset hblank flag
            memory.write(
                registers::DISPSTAT,
                memory.read(registers::DISPSTAT) & !0b10u8,
            );



            // Increment or reset the VCOUNT register
            vcount = if vcount < 227 {
                // Set vblank flag
                memory.write(
                    registers::DISPSTAT,
                    memory.read(registers::DISPSTAT) | 0b1u8,
                );
                // Check if vblank IRQ is set
                if memory.read(registers::DISPSTAT) & 0b1000u8 != 0 {
                    interrupts.vblank()
                };
                vcount + 1
            } else {
                // Unset vblank flag
                memory.write(
                    registers::DISPSTAT,
                    memory.read(registers::DISPSTAT) & !0b1u8,
                );
                0
            };

            0
        };

        memory.write(registers::VCOUNT, vcount as u8);

        (State::Running, interrupts)
    }
}

/// Constants for lcd register address mapping
pub mod registers {
    pub const DISPCNT: u32 = 0x400_0000;
    pub const DISPSTAT: u32 = 0x400_0004;
    pub const VCOUNT: u32 = 0x400_0006;
    pub const BG0CNT: u32 = 0x400_0008;
    pub const BG1CNT: u32 = 0x400_000A;
    pub const BG2CNT: u32 = 0x400_000C;
    pub const BG3CNT: u32 = 0x400_000E;
    pub const BG0HOFS: u32 = 0x400_0010;
    pub const BG0VOFS: u32 = 0x400_0012;
    pub const BG1HOFS: u32 = 0x400_0014;
    pub const BG1VOFS: u32 = 0x400_0016;
    pub const BG2HOFS: u32 = 0x400_0018;
    pub const BG2VOFS: u32 = 0x400_001A;
    pub const BG3HOFS: u32 = 0x400_001C;
    pub const BG3VOFS: u32 = 0x400_001E;
    pub const BG2PA: u32 = 0x400_0020;
    pub const BG2PB: u32 = 0x400_0022;
    pub const BG2PC: u32 = 0x400_0024;
    pub const BG2PD: u32 = 0x400_0026;
    pub const BG2X: u32 = 0x400_0028;
    pub const BG2Y: u32 = 0x400_002C;
    pub const BG3PA: u32 = 0x400_0030;
    pub const BG3PB: u32 = 0x400_0032;
    pub const BG3PC: u32 = 0x400_0034;
    pub const BG3PD: u32 = 0x400_0036;
    pub const BG3X: u32 = 0x400_0038;
    pub const BG3Y: u32 = 0x400_003C;
    pub const WIN0H: u32 = 0x400_0040;
    pub const WIN1H: u32 = 0x400_0042;
    pub const WIN0V: u32 = 0x400_0044;
    pub const WIN1V: u32 = 0x400_0046;
    pub const WININ: u32 = 0x400_0048;
    pub const WINOUT: u32 = 0x400_004A;
    pub const MOSAIC: u32 = 0x400_004C;
    pub const BLDCNT: u32 = 0x400_0050;
    pub const BLDALPHA: u32 = 0x400_0052;
    pub const BLDY: u32 = 0x400_0054;

    pub const fn local(address: u32) -> usize {
        address as usize - 0x400_0000
    }
}

// Other constants
pub const SCREEN_WIDTH:  usize = 240;
pub const SCREEN_HEIGHT: usize = 160;

