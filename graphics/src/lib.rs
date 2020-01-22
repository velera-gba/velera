mod graphics;
use self::graphics::Graphics;
pub use self::graphics::{CacheInstance, CacheObject, Interrupt, State};

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
        return match addr as usize {
            0x0400_0000..=0x0400_0056 => self.lcd[(addr - 0x0400_0000) as usize],
            0x0400_0130..=0x0400_0132 => self.keypad[(addr - 0x0400_0130) as usize],
            0x0500_0000..=0x0500_03FF => self.palette[(addr - 0x0500_0000) as usize],
            0x0600_0000..=0x0601_7FFF => self.vram[(addr - 0x0600_0000) as usize],
            0x0700_0000..=0x0700_03FF => self.oam[(addr - 0x0700_03FF) as usize],
            _ => 0,
        };
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

/// Emulates the functionality of the GBA display and keypad hardware
///
/// Usage:
/// ```rust
/// use graphics::{ Display, State };
/// const SCREEN_SCALE: u32 = 2;
/// 
/// // Initialise the graphics backend and SDL
/// let (mut memory, mut display) = Display::init(SCREEN_SCALE).unwrap();
/// // These 2 structures cannot be owned by Display and as such exist here
/// let cache = display.graphics_cache();
/// let mut cache_instance = Display::instanciate_cache(&cache);
///
/// loop {
///     // To be called when a new scanline is to be drawn
///     
///     memory = match display.cycle(&mut cache_instance, memory) {
///         (State::Exited, _, _) => break (),
///         (_, _, memory) => memory,
///     }
/// }
/// ```
///
/// The memory struct contains boxed slices of the displays memory segments.
/// The registers module contains convenience constants and a local function for getting the memory address relative to the boxed slice
pub struct Display {
    graphics: Graphics,
}

impl Display {
    pub fn init(scale: u32) -> Result<(Memory, Self), String> {
        let graphics = Graphics::setup(scale)?;

        Ok((
            Memory {
                palette: vec![0; 1024].into_boxed_slice(),
                vram: vec![0; 96 * 1024].into_boxed_slice(),
                oam: vec![0; 1024].into_boxed_slice(),
                lcd: vec![0; 0x56].into_boxed_slice(),
                keypad: vec![0; 4].into_boxed_slice(),
            },
            Self { graphics },
        ))
    }

    pub fn graphics_cache(&self) -> CacheObject {
        self.graphics.graphics_cache()
    }

    pub fn instanciate_cache(cache: &CacheObject) -> CacheInstance {
        Graphics::instanciate_cache(cache)
    }

    /// Draw a the next scan line
    pub fn cycle<'r>(&mut self, cache_instance: &mut CacheInstance<'r>, mut memory: Memory) -> (State, Interrupt, Memory) {
        let mut interrupts = Interrupt::none();

        // Only bits 0-7 are used of this register
        let vcount = memory.read(registers::VCOUNT) as usize;
        let vblank = if vcount > 160 { true } else { false };

        let state = if !vblank {
            // Generate draw closure based on video mode
            let scanline = match memory.lcd[registers::local(registers::DISPCNT)] & 0b111 {
                0 => unimplemented!(),
                1 => unimplemented!(),
                2 => unimplemented!(),
                3 => &memory.vram[240 * 2 * vcount..(vcount + 1) * 240 * 2],
                4 => unimplemented!(),
                5 => unimplemented!(),
                6 | 7 => panic!("Program attempted to use undefined video mode"),
                _ => unreachable!(),
            };
    
            self.graphics.drawline(cache_instance, vcount, scanline)
        } else {
            State::Blanking
        };

        // Increment or reset the VCOUNT register
        let vcount =
            if vcount < 227 {
                // Set vblank flag
                memory.write(registers::DISPSTAT, memory.read(registers::DISPSTAT) | 0b1u8);
                // Check if vblank IRQ is set
                if memory.read(registers::DISPSTAT) | 0b1000u8 != 0 { interrupts.vblank() };
                vcount + 1
            } else {
                // Unset vblank flag
                memory.write(registers::DISPSTAT, memory.read(registers::DISPSTAT) & !0b1u8);
                0
            };
        memory.write(registers::VCOUNT, vcount as u8);

        (state, interrupts, memory)
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

#[cfg(test)]
mod tests {
    #[test]
    fn mode3_test() -> Result<(), String> {
        let (mut memory, mut display) = super::Display::init(4)?;
        let cache = display.graphics_cache();
        let mut cache_instance = super::Display::instanciate_cache(&cache);

        memory.lcd[super::registers::local(super::registers::DISPCNT)] = 0b00000011;
        // Draw rgb pixels at (80,80)
        memory.vram[80 * 480 + 160] = 0b00011111;
        memory.vram[80 * 480 + 161] = 0b00000000;
        memory.vram[80 * 480 + 162] = 0b11100000;
        memory.vram[80 * 480 + 163] = 0b00000011;
        memory.vram[80 * 480 + 164] = 0b00000000;
        memory.vram[80 * 480 + 165] = 0b01111100;

        use super::State;
        loop {
            memory = match display.cycle(&mut cache_instance, memory) {
                (State::Exited, _, _) => break Ok(()),
                (_, _, memory) => memory,
            }
        }
    }
}
