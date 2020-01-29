use memory;
use graphics;
use cpu;
use audio;

fn main() {
    let mut memory = memory::MMU::new();
    let mut display = graphics::Display::init(4).unwrap();

    // Simulate a test mode-3 cartridge
    {
        // Change graphics mode
        memory.store8(graphics::registers::DISPCNT, 0b00000011);
        // Draw rgb pixels at (80,80)
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 80 * 2, 0b00011111);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 80 * 2 + 1 ,0b00000000);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 81 * 2, 0b11100000);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 81 * 2 + 1, 0b00000011);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 82 * 2, 0b00000000);
        memory.store8(memory::base_addrs::VRAM_ADDR as u32 + 80 * 480 + 82 * 2 + 1, 0b01111100);
    }

    use graphics::State;
    loop {
        match display.cycle(&mut memory) {
            (State::Exited, _) => break,
            _ => (),
        }
    }
}
