/*
Pulse Generation Steps:
1. Take One raw 8-bit samples of the 4 pulse duty arrays (12.5, 25, 50,and 75) depending on the register
2. Multiply the Sample by the volume of the channel(Normalize)
3. Resample based on register(There are four samplerates and bitdepth combinations to choose from)
4. Send Sample to channel, where sweep, and volume values can be applied.
5. Listen for reset.
*/

// Pulse Struct
struct Pulse {
    duty: i8,                 // [0-3] an array from PULSE_SAMPLES
    freq: i16,                // [0-2047] Value X F(hz)=4194304/(32*(2048-X))
    length: i8,               // [0-7] 6-bit value X with Length=(64-X)*(1/256) seconds.
    timed_mode: bool,         // [0-1] 0:Continuous, 1:Timed
    sweep_shifts: i8,         // [0-7] T=T±T/(2^n) Where N is the number of shifts, T is Time
    sweep_direction: bool,    // [0-1] Increasing is 0, Decreasing is 1
    sweep_time: i8,           // [0-7] 0: Off, 1: 1/128Khz, 2: 2/128Khz, 3: 3/128Khz
    envelope_step_time: i8,   // [0-7] Value N T(step)=N*(1/64);
    envelope_mode: bool,      // [0-1] 0:Increases, 1:Decreases
    envelope_initial: i16,    // [0-15] Envelope Initial Value; 4-bit amplitude value

    sample: [i32; 8],        // An 8-bit Sample; bit-depth is 16.
    reset: bool,
}

// Create Pulse
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
    
}