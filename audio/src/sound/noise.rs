extern crate rand;
use rand::random;
// Test Noise Generator
pub fn gen_noise(samples: u16) -> Vec<i16> {
    let mut sample_noise = vec![0i16];
    for x in 0..samples {
        sample_noise.push(rand::random::<i16>());
    }
    //print!("Noise:{:?}", signal_noise);
    return sample_noise;
}