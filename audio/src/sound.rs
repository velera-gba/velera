/////////////////////////
// Velera Sound Module //
/////////////////////////
extern crate rodio;
extern crate sample;
use rodio::Sink;
use rodio::buffer::SamplesBuffer;

pub struct GbaAudio {
    channels: [Sink; 6],  // Array of channels
    #[allow(dead_code)]
    pulses: [[i16;8];4],  // Array of Pulse Samples
    #[allow(dead_code)]
    wavetable:[i16;130],  // Array of Wavetable Samples
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
            wavetable: [0i16,512,1024,1536,2048,2560,3072,3584,4096,4608,5120,5632,6144,6656,7168,7680,8192,8704,9216,9728,10240,10752,11264,11776,12288,
            12800,13312,13824,14336,14848,15360,15872,16384,16895,17407,17919,18431,18943,19455,19967,20479,20991,21503,22015,22527,23039,23551,24063,24575,
            25087,25599,26111,26623,27135,27647,28159,28671,29183,29695,30207,30719,31231,31743,32255,32767,32255,31743,31231,30719,30207,29695,29183,28671,
            28159,27647,27135,26623,26111,25599,25087,24575,24063,23551,23039,22527,22015,21503,20991,20479,19967,19455,18943,18431,17919,17407,16895,16384,
            15872,15360,14848,14336,13824,13312,12800,12288,11776,11264,10752,10240,9728,9216,8704,8192,7680,7168,6656,6144,5632,5120,4608,4096,
            3584,3072,2560,2048,1536,1024,512,0,-512,],
        }
    }
    
    pub fn test_tone(&self, _freq: u32, _channel: usize) {
        //Init Channels
        let pulse_a = &self.channels[0];
        let pulse_b = &self.channels[1];
        let _wave = &self.channels[2];
        let _noise = &self.channels[3];
        let _directsound_a = &self.channels[4];
        let _directsound_b = &self.channels[5];

        // Pulse Sample with 50% Duty
        let test_pulse_a = [0i16, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050,
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050,
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050,
            ];
        //Pulse Sample with 75% Duty
        let test_pulse_b = [0i16, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050,
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            -22050, -22050,-22050, -22050, -22050, -22050,-22050, -22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 
            22050, 22050, 22050, 22050, 22050, 22050, 22050, 22050, 
            ];
        loop{
            if pulse_a.empty(){
                let buffer_pulse_a = SamplesBuffer::new(2, 22050, test_pulse_a.to_vec());
                pulse_a.append(buffer_pulse_a);
                pulse_a.sleep_until_end();
            }
            if pulse_b.empty(){ 
                let buffer_pulse_b = SamplesBuffer::new(2, 44100, test_pulse_b.to_vec());
                pulse_b.append(buffer_pulse_b);
                pulse_b.sleep_until_end();
            }
        }
    }
}

