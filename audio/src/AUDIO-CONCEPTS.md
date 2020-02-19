 ..::Sound Processing Unit [Gameboy and Gameboy Advance]::..

### Summary:
There are 2 components to the Gameboy Advance Audio Processing Unit; The first 4 Channels, which did not change from the Gameboy Sound System, which are referred to as "Dot Matrix Game"(DMG) Channels. The next 2 Channels are new in the Gameboy Advance and are called "DirectSound Channels".

### Acronyms and Terminology
- [APU] Audio Processing Unit;
-- The component of hardware responsible for computing, processing, and output of sound.

- [Mixer]
-- The volume, effects, and other signal processing and balancing of seperate audio channels.
- [Channel]
-- A reserved track/layer of audio that can play a series of sounds with one generator at a time. Music production applications and MIDI can play multiple voices(Polyphonic), but the GB/GBA has only one note/voice per channel (Monophonic). Arpegiation is often used rather than chords on the same channel because of only being able to play one note at a time.
- [PCM] Pulse Code Modulation;
-- Digital representation of an analog wave; Digital Sound Data represented by a series of pulses at different amplitudes.
- [Wavetable]
- A fixed-sample/fixed bit-depth representation of a waveform's shape.
- [Noise]
-- Random or pseudo random frequencies that do not repeat. (Waveform is non-repeating and random from a range of frequencies)
- [Bit Reduction]
-- The resolution of amplitude is divided by a binary polynomial(Division of the Amplitude Resolution by a power of 2; /2../2^8 etc)
- [SampleRate/Resampling]
-- The resolution and changing of resolution of the frequency/time components of a sound or waveform.(Double samplerate has to account for a half'd frequency).
- [DirectSound]
-- The Gameboy Advance specific sound Channels that can store and process raw wave data(PCM)
- [DMG] Dot Matrix Game; 
 -- Refers to Gameboy Sound Channels
- [PWM] Pulse Width Modulation; 
-- A square wave where the duration ratio of peaks changes.(Modulating the Duty Cycle)
- [Sweep] 
-- Change in Frequency over Time; A pitch envelope.
- [Duty/Duty Cycle] 
-- The Ratio of the peaks to the whole cycle ; 50% being an exact Square wave.
- [Phase]
 -- The offset in time that a cycle of a waveform starts; in a 3 dimensional representation, this is the "rotation" of the signal from 0 to 360 degrees.
 - [Envelope]
 -- The time over which a signal goes from the initial volume to the full volume. Other parameters, such as pitch can also use an envelope.
 - [Frequency]
 - The pitch or speed of a (repeating) signal/wave/oscilation; Measured in Hertz(cycles per second) or period length divided by time.
 - [Loop]
  -- Repetition of a sample, signal, or other pattern
- [Reset] 
-- A generator will stop a note or signal from processing and reset the synthesis/generator to initial parameters.
- [Bank]
-- A collection of Samples, Sounds, or Instruments.
- [Sample]
-- A single "slice" of a sound, represented by a single value(Amplitude) with a set time; In music production, a sample can mean a series of samples or slices, such as a drum sound or instrument sample.
 - [Signed/Unsigned] 
 -- Whether data of sound is represented by a range starting at zero to maximum value or spanning from maximum negative and positive values.(0 to 44100Hz signed values, -1 to +1 floating point unsigned values, etc)
 - [DC Offset] Direct Current Offset/Displacement;
 --  A mean amplitude displacement from zero of a sound or waveform. Often related to "clicks" and Digital Amplifiers

### Extra Concepts  
- Subtractive Synthesis: 
  - Starting with the max harmonics in a waveform(like Square and Saw waves) and subtracting and shaping the harmonics of the waveform.
- Additive Synthesis: 
   - Building up a sound with individual harmonics of a waveform. The ordering(odd/even/every nth) individual sine-wave or harmonic/partial of a waveform plays a large part in the tonality of the sound.
    - Every possible timbre/tonality of a sound is composed of (infinite) octaves(harmonics) of pure sine waves with different ratios/amplitudes, and orders

  

      - All harmonics at 100% amplitude will result in a Sawtooth wave

  

      - Every other(Odd/Even) Harmonic with gradually decreasing amplitude approximates a square wave.

  

  - Essentially a "ground up" or "from scratch" form of synthesis, starting with pure sine waves [displacement@time(d(y)) = Amplitude*cos(freq*time) + phase ] or [y = sin (kx − ωt)]

  

 - FM Synthesis: 
   - Frequency Modulation Synthesis, A series of oscilators that modulate one another to form different timbres.(Oscilators can be referred to as Operators in FM Synthesis)
 - AM: 
    - Amplitude Modulation; Similar to FM synthesis, but amplitude is modulated on top of itself, resulting in

- RM: 
  - Ring Modulation; Where a Carrier signal is modulated(in amplitude) by a modulator signal. Vocoders/Talkboxes implement this.

- Modular Synthesis: Individual components, such as signal generators, filters, signal processors, and effects can have their parameters chained together to result in a certain sound.

- Wavetable Synthesis: A small table of samples is looped to approximate the shape of a basic waveform.

  - Basic Wavetable has a bit-depth and sample resolution, such as 16-bit depth, 256-samples. The higher the samples, the higher the range of frequency, the higher bit-depth, the more variety of amplitude can be represented.

- Physical Modeling Synthesis:
  - Physical interactions of physical sound waves are modeled with mathimatical functions and processed step by step to emulate how a vibration travels through a physical medium.

  - Starting with the force and properties of an object, such as a bow, pick, finger, mallet, or drumstick, a force can propagate through an instrument, body, or resonator, changing the quality of the soundwave allong the way.

  - Soundwaves are modeled to approximate how different parameters of the physical instrument function, such as the size or material of a violin's body or a cymbal's metal density, etc

  - This is also applied with Effects as well, such as Reverb, where the size, air density, shape, and room materials are mathematically simulated.

- FFT: 
  - Fast-Fourier Transform; An algorithm used to change a 2D Amplitude/Time representation(such as an Osciliscope) of a sound to the 3D Time/Frequency/Amplitude Domain (A spectrograph)

- HRTF: 
  - Head Related Transfer Function; A spatial mathmatical function to approximate 3d locality of a sound source. (Used to process a localized/positioned source to stereo listener)

- Holophonics: 
  - A recording technique that uses a (usually silicon) model of the human head and ears with two recording points inside the ears of the model to reintroduce the spatial interaction of the outter ear and head that plays a role in localization.

### Gameboy and Gameboy Advanced Sound Hardware and Synthesis Emulation ###

- PULSE1, PULSE2, WAVE, AND NOISE SIGNAL GENERATORS

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

1. [00]::[9-bit @ 32,768 Hz](144 samples per second)

 2. [01]::[8-bit @ 65,536 Hz](256 samples per second)

 3. [10]::[7-bit @ 131,072 Hz](448 samples per second)

 4. [11]::[6-bit @ 262,144 Hz](+1.5)

