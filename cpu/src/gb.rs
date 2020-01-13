/// the GBA has a coprocessor for backwards compatibility with the GameBoy, based off the Sharp LR35902 (original GameBoy CPU)
/// a regular GBA should never switch into this mode, so we'll implement this in case we want backward compatibility
pub struct LR35902 {}

impl Default for LR35902 {
    fn default() -> Self {
        Self {}
    }
}
