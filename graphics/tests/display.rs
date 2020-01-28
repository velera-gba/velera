#[cfg(test)]
mod tests {
    #[test]
    fn mode3_test() -> Result<(), String> {
        let mut memory = memory::MMU::new();
        let mut display = graphics::Display::init(4)?;

        memory.store8(graphics::registers::DISPCNT, 0b00000011);
        // Draw rgb pixels at (80,80)
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 80 * 2, 0b00011111);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 80 * 2 + 1 ,0b00000000);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 81 * 2, 0b11100000);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 81 * 2 + 1, 0b00000011);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 82 * 2, 0b00000000);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 82 * 2 + 1, 0b01111100);

        let now = std::time::Instant::now();

        use graphics::State;
        loop {
            if now.elapsed() > std::time::Duration::from_secs(10) {
                break Ok(());
            }

            match display.cycle(&mut memory) {
                (State::Exited, _) => break Ok(()),
                _ => (),
            }
        }
    }
}
