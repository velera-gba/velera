// Noise Generator
// (Does not yet implement the true random noise generator for Gameboy, just random values.)
extern crate rand;
use rand::random;
// Test Noise Generator
pub fn gen_noise(samples: u16) -> Vec<i16> {
    let mut sample_noise = vec![];
    for _sample in 0..samples {
        sample_noise.push(random::<i16>());
    }
    return sample_noise;
}