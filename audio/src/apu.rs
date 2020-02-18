/* There are 2 components to the Gameboy Advance Audio Processing Unit;
   The first 4 Channels, which did not change from the Gameboy Sound System, which are referred to as "Dot Matrix Game"(DMG) Channels
   The next 2 Channels are new in the Gameboy Advance and are called "DirectSound Channels"

   ### Acronyms/Terminology ###
    - APU: Audio Processing Unit; The component of hardware responsible for computing, processing, and output of sound.
    - Mixer: The volume, effects, and other signal processing and balancing of seperate audio channels
    - Channel: A reserved track/layer of audio that can play a series of sounds with one generator at a time.
    - PCM: Pulse Code Modulation; Digital representation of an analog wave; Digital Sound Data represented by a series of pulses at different amplitudes.
    - Wavetable: A fixed-sample/fixed bit-depth representation of a waveform's shape.
    - Noise: Random or Psuedo random frequencies that do not repeat. (Waveform is non-repeating and random from a range of frequencies)
    - Bit Reduction: The resolution of ampltitude is divided by a binary polynomial(Divistion of the Amplitude Resolution by a power of 2; /2../2^8 etc)
    - SampleRate/Resampling: The resolution and changing of resolution of the frequency/time components of a sound or waveform.(Double samplerate has to account for a half'd frequency).
    - DirectSound: The Gameboy Advance specific sound Channels that can store and process raw wave data(PCM)
    - DMG: Dot Matrix Game; Refers to Gameboy Sound Modules
    - PWM: Pulse Width Modulation; A square wave where peaks have a length; (ex:___-___-___-___-/_----_----_----_----)
    - Sweep: Change in Frequency over Time
    - Duty: The Ratio of a Pulse/Pulse Width; 50% being an exact Square wave(ex:___---___---___---)
    - Phase: The offset in time that a cycle of a waveform starts; (in 3 dimentional representation, this is the "rotation" of the signal from 0 to 360 degrees)
    - Envelope: The time over which a signal goes from the initial volume to the full volume.
    - Frequency: The pitch or speed of a (repeating) signal/wave/oscilation; Measured in Hertz(cycles per second)
    - Loop: Repetition of a sample, signal, or Note
    - Reset: A generator will stop a note from processing and reset the synthesis/generator to initial parameters
    - Bank: A collection of Samples, Sounds, or Instuments
    - Sample: A single "slice" of a sound, represented by a single value with a set time; Also can refer to a complete set of sound data, such as a 128-sample Wave soundbank(Wavetable)
    - Signed/Unsigned: Whether data of sound is represented by a range starting at zero to maximum value or spanning from maximum negative and positive values.(0 to 44100Hz signed values, -1 to +1 floating point unsigned values, etc)
    - DC Offset: Direct Current Offset/Displacement; a mean amplitude displacement from zero of a sound or waveform. Often related to "clicks" and Digital Amplifiers
    
    
    +++ Extra Concepts +++
    - Subtractive Synthesis: Starting with the max harmonics in a waveform(like Square and Saw waves) and subtracting and shaping the harmonics of the waveform.
    - Additive Synthesis: Building up a sound with individual harmonics of a waveform. The ordering(odd/even/every nth) individual sine-wave or harmonic/partial of a waveform plays a large part in the tonality of the sound.
        --- Every possible timbre/tonality of a sound is composed of (infinite) octaves(harmonics) of pure sine waves with different ratios/amplitudes, and orders
        --- All harmonics at 100% amplitude will result in a Sawtooth wave
        --- Every other(Odd/Even) Harmonic with gradually decreasing amplitude approximates a square wave.
        --- Essentially a "ground up" or "from scratch" form of synthesis, starting with pure sine waves [displacement@time(d(y)) = Amplitude*cos(freq*time) + phase ] or [y = sin (kx − ωt)]
    - FM Synthesis: Frequency Modulation Synthesis, A series of oscilators that modulate one another to form different timbres.(Oscilators can be referred to as Operators in FM Synthesis)
        --- 
    - AM Synthesis: Similar to FM synthesis, but amplitude is modulated on top of itself, resulting in 
    - Ring Modulation: Ring Modulation, where a Carrier signal is modulated(in amplitude) by a modulator signal. Vocoders/Talkboxes implement this.
    - Modular Synthesis: Individual components, such as signal generators, filters, signal processors, and effects can have their parameters chained together to result in a certain sound.
    - Wavetable Synthesis: A small table of samples is looped to approximate the shape of a basic waveform.
       --- Basic Wavetable has a bit-depth and sample resolution, such as 16-bit depth, 256-samples. The higher the samples, the higher the range of frequency, the higher bit-depth, the more variety of amplitude can be represented.
    - Physical Modeling Synthesis: Physical interactions of physical sound waves are modeled with mathimatical functions and processed step by step to emulate how a vibration travels through a physical medium.
       --- Starting with the force and properties of an object, such as a bow, pick, finger, mallet, or drumstick, a force can propagate through an instrument, body, or resonator, changing the quality of the soundwave allong the way.
       --- Soundwaves are modeled to approximate how different parameters of the physical instrument function, such as the size or material of a violin's body or a cymbal's metal density, etc
       --- This is also applied with Effects as well, such as Reverb, where the size, air density, shape, and room materials are mathematically simulated.
    - FFT: Fast-Fourier Transform; An algorithm used to change a 2D Amplitude/Time representation(such as an Osciliscope) of a sound to the 3D Time/Frequency/Amplitude Domain (A spectrograph)
    - HRTF: Head Related Transfer Function; A spatial mathmatical function to approximate 3d locality of a sound source. (Used to process a localized/positioned source to stereo listener)
    - Holophonics: A recording technique that uses a (usually silicon) model of the human head and ears with two recording points inside the ears of the model to reintroduce the spatial interaction of the outter ear and head that plays a role in localization.
    
    
    ### Gameboy and Gameboy Advanced Sound Hardware and Synthesis Emulation ###
    
    ++ PULSE1, PULSE2, WAVE, AND NOISE SIGNAL GENERATORS
        [Gameboy] #Channel 1 - Pulse Oscilator 1
            - Sweep[]
                Shift: 0-7 [210] //
                Increase/Decrease: (0 = inc, 1 = dec) [0-1] //boolean
                Time: [000-111]
            - Length
            - Duty
            - Envelope
            - Frequency
            - Loop
            - Reset
        [Gameboy] #Channel 2 - Pulse Oscilator 2
            - Length
            - Duty
            - Envelope
            - Frequency
            - Loop
            - Reset
        [Gameboy] #Channel 3 - Wave Oscilator[4-bit, 8 Samples per word]
            - Bank Mode
            - Bank Select
            - Enable Output
            - Sound Length
            - Output Volume Ratio
            - Frequency
            - Loop
            - Reset
            -- Sample 0[4-bit, 128 Samples]
            -- Sample 1[4-bit, 128 Samples]
            -- Sample 2[4-bit, 128 Samples]
            -- Sample 3[4-bit, 128 Samples]
        [Gameboy] #Channel 4 - Noise Generator[7/15 stages linear-feedback shift register(LFSR)]
            - Length
            - Envelope Time
            - Envelope Increase/Decrease
            - Initial Envelope Value
            - Clock Divider Frequency[Freq. - 2x to 1/7]
            - Counter Stages(4 or 8 bit Noise Depth)[7 or 15 stages]
            - Counter Pre-Stepper frequency (Counter Polynomial / Reduction Factor)[Q/2 - Q/2^14]
            - Timed Mode (Loop)
            - Reset
        [DirectSound] #Channel 5 - DirectSound Channel A
            - Right Output
            - Left Output
            - Sample Rate Timer[0,1]
            - First-in First-Out(FIFO) Reset
        [DirectSound] #Channel 6(Right) - DirectSound Channel B
            - Right Output
            - Left Output
            - Sample Rate Timer[0,1]
            - First-in First-Out(FIFO) Reset

    ++ Channel Monitor/Mixer and Output Control
        [Master] Sound Output Control/Status
            - DMG Channel 1 Status [On/Off]
            - DMG Channel 2 Status [On/Off]
            - DMG Channel 3 Status [On/Off]
            - DMG Channel 4 Status [On/Off]
            - DMG Circuit Power [On/Off]

        [Audio Settings]
            - DC Offset(BIAS)
            - Pulse Width Modulation(PWM) Resampling Resolution:
                1. [00]::[9-bit @  32,768 Hz](144 samples per second)
                2. [01]::[8-bit @  65,536 Hz](256 samples per second)
                3. [10]::[7-bit @ 131,072 Hz](448 samples per second)
                4. [11]::[6-bit @ 262,144 Hz](+1.5)
*/

//Audio Host
//Audio Device
//Init Channels/Sinks [6 Channels; 4 Gameboy, 2 Gameboy Advance DirectSound]

//Channel 1 - Pulse A

//Channel 2 - Pulse B

//Channel 3 - Wavetable

//Channel 4 - Noise

//Channel 5 - DirectSound A

//Channel 6 - DirectSound B

//Channel Mixer

//Master Output Control



//Static Audio Samples
//Default waveshape for wavetable
const WAVETABLE_SAMPLES : [[i32; 8]; 4] = [ //4 Pulse Widths with bit-depth of 16 (-15 to +15)
    [-1, -1, -1, -1, 1, -1, -1, -1],
    [-1, -1, -1, -1, 1, 1, -1, -1],
    [-1, -1, 1, 1, 1, 1, -1, -1],
    [1, 1, 1, 1, -1, -1, 1, 1]
    ];

const CLOCKS_PER_SECOND : u32 = 1 << 22;
const OUTPUT_SAMPLE_COUNT : usize = 2816; //22 clocksper 128




//Pulse PWM 2-bit unsigned samples(Downsampled to lowest denominator, x144 Samples gives us 1 second and 32,768 Hz Sample rate.).[00-11]

const PULSE_SAMPLES: [[i32; 8]; 4] = [
    [1, -1, -1, -1, -1, -1, -1, -1],           //12.5% PW/Duty   [-------+]
    [1, 1, -1, -1, 1, 1, -1, -1],              //25% PW/Duty     [+------+]
    [1, 1, 1, 1, -1, -1, -1, -1],              //50% PW/Duty     [+----+++]
    [1, 1, 1, 1, 1, 1, -1, -1]                 //75% PW/Duty     [-++++++-]
];



//Synthesis

//[Pulse1] #Channel 1

//[0x060::REG_SOUND1CNT_L] #DMG Channel 1 Sweep control
//[0x062::REG_SOUND1CNT_H] #DMG Channel 1 Length, wave duty and envelope control
//[0x064::REG_SOUND1CNT_X] #DMG Channel 1 Frequency, reset and loop control
struct Envelope {
    time : u8,         //Current time/position of the envelope
    mode : bool,       //Decrease or Increase volume
    step_time : u8,    //Amount increased or decreased per step
    initial : u8,      //Initial Volume of the Envelope; 0-15
    volume : u8,       //Current Volume of the Envelope          
}

impl Envelope {
    fn create() -> Envelope {
        Envelope {
            time: 0,       // Start at T=0
            mode: false,   // Decrease volume
            step_time: 0,  // Disabled
            initial: 15,   // Full volume
            volume: 15,    // Start at full volume
        }
    }

    fn rw_reset(&mut self, address: u16, value: u8) { //Read+Write Reset
        match address {
            0xFF12 | 0xFF17 | 0xFF21 => {
                self.time= value & 0x7;            //Reset Envelope Registers (Pulse A, Pulse B, and Noise)
                self.mode = value & 0x8 == 0x8;  
                self.initial = value >> 4;
                self.volume = self.initial; 
            },
            0xFF14 | 0xFF19 | 0xFF23 if value & 0x80 == 0x80 => { //If Register values for resetting are true, reset
                self.step_time = self.time;
                self.volume = self.initial;
            },
            _ => (),
        }
    }

    fn step(&mut self) {
        if self.step_time > 1 {                                // If the envelope step time is more than 1, decrease the envelope step time
            self.step_time -= 1;
        }
        else if self.step_time == 1 {                          // If the envelope step time is equal to 1, reset the envelope to time
            self.step_time = self.time;
            if self.mode && self.volume < 15 {        // Increase the volume if mode is increasing[0] and volume is less than 15.
                self.volume += 1;
            }
            else if !self.mode && self.volume > 0 {   // Decrease the volume if the mode is decreasing[1] and volume is greater than 0
                self.volume -= 1;
            }
        }
    }
}

struct Pulse {
    duty: i8,                      // [0-3] an array from PULSE_SAMPLES
    freq: i16,                     // [0-2047] Value X F(hz)=4194304/(32*(2048-X))
    length: i8,                    // [0-7] 6-bit value X with Length=(64-X)*(1/256) seconds.
    timed_mode: bool,              // [0-1] 0:Continuous, 1:Timed
    sweep_shifts: i8,         // [0-7] T=T±T/(2^n) Where N is the number of shifts, T is Time
    sweep_direction: bool,    // [0-1] Increasing is 0, Decreasing is 1
    sweep_time: i8,           // [0-7] 0: Off, 1: 1/128Khz, 2: 2/128Khz, 3: 3/128Khz
    envelope_step_time: i8,    // [0-7] Value N T(step)=N*(1/64);
    envelope_mode: bool,      // [0-1] 0:Increases, 1:Decreases
    envelope_initial: i16,    // [0-15] Envelope Initial Value; 4-bit amplitude value

    sample: [i32; 8],        // An 8-bit Sample; bit-depth is 16.
    reset: bool,
}

impl Pulse {
    fn create() -> Pulse{
        Pulse{
            duty: 2,
            freq: 1024,
            length: 7,
            timed_mode:0,
            sweep_shifts:6,
            sweep_direction:true,
            sweep_time:3,
            envelope_step_time:4,
            envelope_mode:1,
            envelope_initial:15,
        }
    }

    fn set_duty(&self, value: i8){
        self.duty = PULSE_SAMPLES[&value];
        print!("Pulse A Duty Set: {:?}", self.duty);
        //write value to register MMU::write_register()
    }

    fn get_duty(){
        //read register
        let value = 1; //Test value
        self.duty = PULSE_SAMPLES[value as usize];
        print!("Pulse A Duty Get: {:?}", )
        
    }
}





