#[cfg(not(feature = "fbdev"))]
mod sdl2;
#[cfg(not(feature = "fbdev"))]
pub type Backend = sdl2::Backend;

#[cfg(feature = "fbdev")]
mod linux_framebuffer;
#[cfg(feature = "fbdev")]
pub type Backend = linux_framebuffer::Backend;

/// A BGR555 colour
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BGR555(pub u16);

impl BGR555 {
    #[inline]
    /// Approximate an 8bit intensity as a 5bit intensity
    fn bits8_to_5(byte: u8) -> u8 {
        (byte as f32 / 255.0 * 31.0) as u8 & 0b11111
    }
}

impl From<&[u8]> for BGR555 {
    fn from(slice: &[u8]) -> Self {
        match slice.len() {
            1 => { eprintln!(
                "At least 2 bytes are needed to convert to BGR555. Instead recieved {:?}",
                slice
            ); BGR555(0) },
            3 => [slice[0], slice[1], slice[2]].into(),
            2 | _ => [slice[0], slice[1]].into(),
        }
    }
}

impl From<[u8; 2]> for BGR555 {
    #[inline]
    fn from(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
}

impl From<[u8; 3]> for BGR555 {
    fn from(bytes: [u8; 3]) -> Self {
        Self(
            (Self::bits8_to_5(bytes[0]) as u16)
                | (Self::bits8_to_5(bytes[1]) as u16) << 5
                | (Self::bits8_to_5(bytes[2]) as u16) << 10,
        )
    }
}

impl From<u32> for BGR555 {
    fn from(rgba: u32) -> Self {
        Self(
            (Self::bits8_to_5((rgba & 0xFF0000 >> 16) as u8) as u16)
                | (Self::bits8_to_5((rgba & 0xFF00 >> 8) as u8) as u16) << 5
                | (Self::bits8_to_5((rgba & 0xFF) as u8) as u16) << 10,
        )
    }
}

impl From<RGBA> for BGR555 {
    fn from(rgba: RGBA) -> Self {
        Self(
            (Self::bits8_to_5((*rgba & 0xFF0000 >> 16) as u8) as u16)
                | (Self::bits8_to_5((*rgba & 0xFF00 >> 8) as u8) as u16) << 5
                | (Self::bits8_to_5((*rgba & 0xFF) as u8) as u16) << 10,
        )
    }
}

impl std::ops::Deref for BGR555 {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A RGBA colour
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGBA(pub u32);

impl RGBA {
    #[inline]
    /// Approximate a 5bit intensity as an 8bit intensity
    fn bits5_to_8(byte: u8) -> u8 {
        (byte as f32 / 31.0 * 255.0) as u8
    }
}

impl From<BGR555> for RGBA {
    fn from(bgr: BGR555) -> Self {
        Self(
            (Self::bits5_to_8((*bgr & 0b000000000011111) as u8) as u32) << 16
                | (Self::bits5_to_8(((*bgr & 0b000001111100000) >> 5) as u8) as u32) << 8
                | (Self::bits5_to_8(((*bgr & 0b111110000000000) >> 10) as u8) as u32),
        )
    }
}

impl Into<(u8, u8, u8)> for RGBA {
    fn into(self) -> (u8, u8, u8) {
        (
            ((*self & 0xFF0000) >> 16) as u8,
            ((*self & 0xFF00) >> 8) as u8,
            (*self & 0xFF) as u8,
        )
    }
}

impl std::ops::Deref for RGBA {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Maps scancodes to keys
pub struct Keymap {
    // GBA keys
    pub a:      u8,
    pub b:      u8,
    pub select: u8,
    pub start:  u8,
    pub right:  u8,
    pub left:   u8,
    pub up:     u8,
    pub down:   u8,
    pub r:      u8,
    pub l:      u8,

    // Emulator keys
    pub exit:   u8,
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            a:      44,
            b:      45,
            select: 57,
            start:  28,
            right:  106,
            left:   105,
            up:     103,
            down:   108,
            r:      82,
            l:      97,

            exit:   1,
        }
    }

    #[inline]
    pub fn match_key(&self, key: u8, states: &mut InputStates) {
        match key {
            k if k == self.exit      => states.exit   = true,
            k if k == self.a         => states.a      = true,
            k if k == self.b         => states.b      = true,
            k if k == self.select    => states.select = true,
            k if k == self.start     => states.start  = true,
            k if k == self.right     => states.right  = true,
            k if k == self.left      => states.left   = true,
            k if k == self.up        => states.up     = true,
            k if k == self.down      => states.down   = true,
            k if k == self.r         => states.r      = true,
            k if k == self.l         => states.l      = true,

            _ => ()
        }
    }

    pub fn bind<F: FnMut(&str, u8) -> u8>(mut prompt: F) -> Self {
        let mut map = Self::new();
        map.exit    = prompt("exit",            map.exit);
        map.a       = prompt("a",               map.a);
        map.b       = prompt("b",               map.b);
        map.select  = prompt("select",          map.select);
        map.start   = prompt("start",           map.start);
        map.right   = prompt("right",           map.right);
        map.left    = prompt("left",            map.left);
        map.up      = prompt("up",              map.up);
        map.down    = prompt("down",            map.down);
        map.r       = prompt("right bumper",    map.r);
        map.l       = prompt("left bumper",     map.l);

        map
    }
}

pub struct InputStates {
    // GBA keys
    pub a:      bool,
    pub b:      bool,
    pub select: bool,
    pub start:  bool,
    pub right:  bool,
    pub left:   bool,
    pub up:     bool,
    pub down:   bool,
    pub r:      bool,
    pub l:      bool,

    // Emulator keys
    pub exit:   bool,
}

impl InputStates {
    pub fn new() -> Self {
        Self {
            a:      false,
            b:      false,
            select: false,
            start:  false,
            right:  false,
            left:   false,
            up:     false,
            down:   false,
            r:      false,
            l:      false,

            exit:   false,
        }
    }

    pub fn from_u16(raw: u16) -> Self {
        Self {
            a:      raw & 1 != 0,
            b:      raw & (1 << 1) != 0,
            select: raw & (1 << 2) != 0,
            start:  raw & (1 << 3) != 0,
            right:  raw & (1 << 4) != 0,
            left:   raw & (1 << 5) != 0,
            up:     raw & (1 << 6) != 0,
            down:   raw & (1 << 7) != 0,
            r:      raw & (1 << 8) != 0,
            l:      raw & (1 << 9) != 0,

            exit:   false,
        }
    }

    pub fn to_u16(&self) -> u16 {
        self.a as u16
            | (self.b as u16) << 1
            | (self.select as u16) << 2
            | (self.start as u16) << 3
            | (self.right as u16) << 4
            | (self.left as u16) << 5
            | (self.up as u16) << 6
            | (self.down as u16) << 7
            | (self.r as u16) << 8
            | (self.l as u16) << 9
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bgr555_rgb888() {
        use super::BGR555;
        use super::RGBA;
        assert_eq!(BGR555(0b0111110000000000), [0, 0, 0xFF].into());
        assert_eq!(BGR555(0b0000001111100000), [0, 0xFF, 0].into());
        assert_eq!(BGR555(0b0000000000011111), [0xFF, 0, 0].into());

        assert_eq!(RGBA::from(BGR555(0b0111111111111111)), RGBA(0xFFFFFF));

        assert_eq!(RGBA::from(BGR555(0b0111110000000000)), RGBA(0x0000FF));
        assert_eq!(RGBA::from(BGR555(0b0000001111100000)), RGBA(0x00FF00));
        assert_eq!(RGBA::from(BGR555(0b0000000000011111)), RGBA(0xFF0000));
    }
}
