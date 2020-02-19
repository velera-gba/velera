#[allow(dead_code)]
/////////////////////////
// Velera Sound Module //
/////////////////////////
mod noise;
extern crate rodio;
extern crate sample;
use rodio::Sink;
use rodio::buffer::SamplesBuffer;

pub struct GbaAudio {
    channels: [Sink; 6],  // Array of channels
    #[allow(dead_code)]
    pulses: [[i16;8];4],  // Array of Pulse Samples
    #[allow(dead_code)]
    wavetable:[i16;129],  // Array of Wavetable Samples
}

impl GbaAudio {
    pub fn init_audio() -> GbaAudio{
        let device = rodio::default_output_device().unwrap();
        GbaAudio{
            channels: [
                Sink::new(&device),
                Sink::new(&device),
                Sink::new(&device),
                Sink::new(&device),
                Sink::new(&device),
                Sink::new(&device),
            ],
            //Pulse Table
            #[allow(dead_code)]
            pulses: [
                [1,-1,-1,-1,-1,-1,-1,-1], //12.5% Duty(PWM)
                [1,-1,-1,-1,-1,-1,-1,1],  //25% Duty(PWM)
                [1,1,-1,-1,-1,-1,1,1],    //50% Duty(PWM)
                [1,1,1,1,-1,-1,1,1]      //75% Duty(PWM)
                ],
            //Wavetable Sample (Triangle Wave)
            wavetable: [512i16,1024,1536,2048,2560,3072,3584,4096,4608,5120,5632,6144,6656,7168,7680,8192,8704,9216,9728,10240,10752,11264,11776,12288,
            12800,13312,13824,14336,14848,15360,15872,16384,16895,17407,17919,18431,18943,19455,19967,20479,20991,21503,22015,22527,23039,23551,24063,24575,
            25087,25599,26111,26623,27135,27647,28159,28671,29183,29695,30207,30719,31231,31743,32255,32767,32255,31743,31231,30719,30207,29695,29183,28671,
            28159,27647,27135,26623,26111,25599,25087,24575,24063,23551,23039,22527,22015,21503,20991,20479,19967,19455,18943,18431,17919,17407,16895,16384,
            15872,15360,14848,14336,13824,13312,12800,12288,11776,11264,10752,10240,9728,9216,8704,8192,7680,7168,6656,6144,5632,5120,4608,4096,
            3584,3072,2560,2048,1536,1024,512,0,-512,],
        }
    }
    #[allow(unused_variables, unused_mut)]
    pub fn test_tone(&self, _freq: u32, _channel: usize) {
        //Init Channels
        let pulse_a = &self.channels[0];
        let pulse_b = &self.channels[1];
        let wave = &self.channels[2];
        let noise = &self.channels[3];
        let _directsound_a = &self.channels[4];
        let _directsound_b = &self.channels[5];

        // Pulse Sample with 50% Duty
        let test_pulse_a = [ 
            -22050i16, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050,
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050,
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050,
            ];
        //Pulse Sample with 75% Duty
        let test_pulse_b = [
            -22050i16, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050,
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 
            ];
        let test_wavetable = &self.wavetable;
        let mut test_noise; //Returns Vec<i16>
        let mut time: u16 = 1;
        loop{
            if pulse_a.empty(){
                let buffer_pulse_a = SamplesBuffer::new(1, 44100, test_pulse_a.to_vec());
                pulse_a.append(buffer_pulse_a);
            }
            if pulse_b.empty(){ 
                let buffer_pulse_b = SamplesBuffer::new(1, 44100, test_pulse_b.to_vec());
                pulse_b.append(buffer_pulse_b);
            }
            if wave.empty(){
                let buffer_wave = SamplesBuffer::new(1, 44100, test_wavetable.to_vec());
                wave.append(buffer_wave);
            }

            //The noise is sampled at an extremely low rate and is affecting the wavetable in a cool way! xD
            if noise.empty(){
                test_noise = noise::gen_noise(44100);
                let buffer_noise = SamplesBuffer::new(1, time.into(), test_noise.to_vec());
                noise.append(buffer_noise);
            }
            if time < 44100 {
                time += 1;
            }
            else {
                time = 1;
            }
        }
    }
}

