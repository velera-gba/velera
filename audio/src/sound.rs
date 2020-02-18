
extern crate rodio;
//use rodio::Device;
use rodio::Sink;
use rodio::buffer::SamplesBuffer;
use rodio::static_buffer::StaticSamplesBuffer;
use std::time::Duration;
use rand::random;

pub struct GbaAudio {
    Channels: [Sink; 6],  // Array of channels with a string
}
impl GbaAudio {
    pub fn init_audio() -> GbaAudio{
        let device = rodio::default_output_device().unwrap();
        GbaAudio{
            Channels: [
                Sink::new(&device),
                Sink::new(&device),
                Sink::new(&device),
                Sink::new(&device),
                Sink::new(&device),
                Sink::new(&device),
            ]
                
        }
    }

    pub fn test_tone(&self, freq: u32, channel: usize) {
        let sine = rodio::source::SineWave::new(freq);
        let pulseA = &self.Channels[0];
        let pulseB = &self.Channels[1];
        let noise = &self.Channels[3];
        let mut noise_sample = rand::random::<i16>();
        let mut time = 0;
        print!("::{:?}", noise_sample);
        loop{
            time += 1;
            noise_sample = rand::random::<i16>();
            //print!("::{}", &noise);
            let sample1 = StaticSamplesBuffer::new(1, 44100, &[1i16, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048]);
            let sample2 = StaticSamplesBuffer::new(1, 44100, &[1i16, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048]);
            let sample3 = SamplesBuffer::new(1, 44100, vec!(1i16, noise_sample));
            
            pulseA.append(sample1);
            pulseA.sleep_until_end();
            pulseB.append(sample2);
            pulseB.sleep_until_end();
            //noise.append(sample3);
            //stream.sleep_until_end();
            //   stream.append(repeat);
            //stream.sleep_until_end();
        }
    }
}

// Add a dummy source of the sake of the example.
