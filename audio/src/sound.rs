
extern crate rodio;
extern crate sample;
//use rodio::Device;
use sample::{signal, Signal};
use sample::interpolate::Linear;
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
    pub fn gen_noise(samples: u16) -> Vec<i16> {
        let mut signal_noise = vec![0i16];
        for x in 0..samples {
            signal_noise.push(rand::random::<i16>());
        }
        print!("Noise:{:?}", signal_noise);
        return signal_noise;
    }

    pub fn test_tone(&self, freq: u32, channel: usize) {
        //let sine = rodio::source::SineWave::new(freq);
        let pulseA = &self.Channels[0];
        let pulseB = &self.Channels[1];
        let wave = &self.Channels[2];
        let noise = &self.Channels[3];
        let directsoundA = &self.Channels[4];
        let directsoundB = &self.Channels[5];
        let mut time: i16 = -32767;
        let mut time_step: i16 = 1;
        //print!("::{:?}", noise_sample);
        let sample3 = SamplesBuffer::new(6, 44100,              //Triangle Wavetable!
            vec!(
                1i16, 512,1024,1536,22050,2560,3072,3584,4096,
                4608,5120,5632,6144,6656,7168,7680,8192,
                8704,9216,9728,10240,10752,11264,11776,12288,
                12800,13312,13824,14336,14848,15360,15872,16384,
                16895,17407,17919,18431,18943,19455,19967,20479,
                20991,21503,22015,22527,23039,23551,24063,24575,
                25087,25599,26111,26623,27135,27647,28159,28671,
                29183,29695,30207,30719,31231,31743,32255,32767,
                32255,31743,31231,30719,30207,29695,29183,28671,
                28159,27647,27135,26623,26111,25599,25087,24575,
                24063,23551,23039,22527,22015,21503,20991,20479,
                19967,19455,18943,18431,17919,17407,16895,16384,
                15872,15360,14848,14336,13824,13312,12800,12288,
                11776,11264,10752,10240,9728,9216,8704,8192,
                7680,7168,6656,6144,5632,5120,4608,4096,
                3584,3072,2560,22050,1536,1024,512,0,
            ));
        //wave.append(sample3);
        let foo = [[1], [1], [-1], [-1], [-1], [-1], [1], [1]];
        let mut source = signal::from_iter(foo.iter().cloned());
        let interp = Linear::from_source(&mut source);
        let frames: Vec<_> = source.scale_hz(interp, freq as f64).take(8).collect();
        let mut square_wave = frames.clone();
        print!("Foo: {:?} \n", frames);
        let sample4 = SamplesBuffer::new(2, 22050, GbaAudio::gen_noise(44100));
                    //let mut sample4 = SamplesBuffer::new(2, 44100, vec!(1i16, square_wave));
        
        loop{
            //time += time_step;
            
            //print!("::{}", &noise);
            if pulseA.empty(){
                let sample1 = StaticSamplesBuffer::new(2, 22050, &[0i16, -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050]);
                pulseA.append(sample1);
                pulseA.sleep_until_end();
            }
            if pulseB.empty(){
                let sample2 = StaticSamplesBuffer::new(2, 44100, &[0i16, -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050]);
                pulseB.append(sample2);
                pulseB.sleep_until_end();
            }
                //print!("Noise Buffer Empty?: {:?} \n", noise.empty());
                //print!("Noise Sample:{:?}\n", GbaAudio::gen_noise(128));
                
                    //print!("{:?}",GbaAudio::gen_noise(22050).clone());
                    //noise.play();
                    //noise.append(sample4); //Uncomment this
                    //noise.sleep_until_end(); //and this for noise!
                
                //noise.sleep_until_end();
                //print!("Noise buffer Empty?: {:?} \n", noise.empty());
            
            //let sweep_test = SamplesBuffer::new(1,44100, vec!(1i16, time, -time));
            //pulseA.sleep_until_end();
            //pulseB.append(sample2);
            //pulseB.sleep_until_end();
            

            //wave.sample3.speed(time/128);
            /*
            if time >= 32767 {
                time = 0;
                print!("time reset");
                if time_step >= 32767{
                    time_step = 1;
                    print!("time_step reset")
                }
                time_step += 1;
            }
            */
            //wave.sleep_until_end();
            //directsoundA.append(sweep_test);
            //directsoundA.sleep_until_end();
            
            //directsoundA
            //directsoundB
            //stream.sleep_until_end();
            //   stream.append(repeat);
            //stream.sleep_until_end();
        }
    }
}

// Add a dummy source of the sake of the example.
