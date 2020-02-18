
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
        let stream = &self.Channels[channel];
        let mut noise = rand::random::<i16>();
        let mut time = 0;
        print!("{}", noise);
        loop{
            time += 1;
            noise = rand::random::<i16>();
            //print!("::{}", &noise);
            let sample1 = StaticSamplesBuffer::new(1, 44100, &[1i16, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048]);
            let sample2 = StaticSamplesBuffer::new(1, 44100, &[1i16, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, -2048, -2048,-2048, -2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048]);
            let sample3 = SamplesBuffer::new(1, 44100, vec!(1i16, noise));
            
            stream.append(sample1);
            stream.append(sample2);
            stream.sleep_until_end();
            //stream.append(sample2);
            //stream.sleep_until_end();
            //   stream.append(repeat);
            //stream.sleep_until_end();
        }
    }
}

// Add a dummy source of the sake of the example.
