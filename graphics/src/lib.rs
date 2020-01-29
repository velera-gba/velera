use memory;

mod backend;
use backend::*;

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
/// const SCALE: u32 = 4;
/// let mut display = graphics::Display::init(SCALE).unwrap();
/// let mut memory = memory::MMU::new();
/// 
/// use graphics::State;
/// loop {
///     match display.cycle(&mut memory) {
///         (State::Exited, _) => break,
///         _ => (),
///     }
/// }
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
    pub fn init(scale: u32) -> Result<Self, String> {
        let backend = backend::Backend::setup(scale)?;

        Ok(Self { backend, hcount: 0 })
    }

    /// A graphics cycle is done every 4 cpu cycles
    pub fn cycle(&mut self, memory: &mut memory::MMU) -> (State, Interrupt) {
        let mut interrupts = Interrupt::none();

        // Get user input
        let input = self.backend.get_input();

        // Only bits 0-7 are used of this register
        let mut vcount = memory.load8(registers::VCOUNT) as usize;
        let vcount_setting = (memory.load8(registers::DISPSTAT) >> 7) as usize;
        let vblank = vcount > 160;
        let hblank = self.hcount > 240;

        if !vblank && !hblank {
            // Generate draw closure based on video mode
            let pixel = match memory.load8(registers::DISPCNT) & 0b111 {
                0 => unimplemented!(),
                1 => unimplemented!(),
                2 => unimplemented!(),
                3 => BGR555(memory.load16((memory::base_addrs::VRAM_ADDR + (self.hcount * 2) + vcount * SCREEN_WIDTH * 2) as u32)),
                4 => unimplemented!(),
                5 => unimplemented!(),
                6 | 7 => { eprintln!("Program attempted to use undefined video mode"); return (State::Exited, interrupts) },
                _ => unreachable!(),
            };

            self.backend.draw_pixel((self.hcount, vcount), pixel.into())
        }

        // Increment the hcount
        self.hcount = if self.hcount < 307 {
            // Set hblank flag
            memory.store8(
                registers::DISPSTAT,
                memory.load8(registers::DISPSTAT) | 0b10u8,
            );
            // Check if hblank IRQ is set
            if memory.load8(registers::DISPSTAT) & 0b10000u8 != 0 {
                interrupts.hblank()
            };
            self.hcount + 1
        } else {
            // Unset hblank flag
            memory.store8(
                registers::DISPSTAT,
                memory.load8(registers::DISPSTAT) & !0b10u8,
            );

            // Increment or reset the VCOUNT register
            vcount = if vcount < 227 {
                // Set vblank flag
                memory.store8(
                    registers::DISPSTAT,
                    memory.load8(registers::DISPSTAT) | 0b1u8,
                );
                // Check if vblank IRQ is set
                if memory.load8(registers::DISPSTAT) & 0b1000u8 != 0 {
                    interrupts.vblank()
                };
                vcount + 1
            } else {
                // Unset vblank flag
                memory.store8(
                    registers::DISPSTAT,
                    memory.load8(registers::DISPSTAT) & !0b1u8,
                );
                0
            };

            0
        };

        memory.store8(registers::VCOUNT, vcount as _);

        (if input.exit { State::Exited } else { State::Running }, interrupts)
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
}

// Other constants
pub const SCREEN_WIDTH: usize = 240;
pub const SCREEN_HEIGHT: usize = 160;
