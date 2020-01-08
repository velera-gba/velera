extern crate cpu;
extern crate memory;

#[cfg(test)]
mod tests {
    #[test]
    fn my_test() {
        let mut test_cpu: cpu::cpu::CPU = Default::default();
        cpu::cpu::run_rom(&mut test_cpu, "../test_roms/test1.gba");
    }
}