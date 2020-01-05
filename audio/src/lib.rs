#![allow(dead_code)]
use std::mem;

const GBA_AUDIO_SAMPLES: u16 = 2048; // 2048 Samples per second 
const GBA_AUDIO_FIFO_SIZE: u32 = 8 * mem::size_of::<i32>() as u32; // First-in First out Size
const GBA_AUDIO_VOLUME_MAX: i16 = 0x100; // Max volume

static CLOCKS_PER_FRAME: u32 = 0x800; // For syncing CPU clock time with Audio


// Apply Bias
// Create Sample object

/* Initiate, Reset, De-initiate, Resize Buffer, Schedule FIFO DMA */

    // Initiate GBA audio

    // Reset GBA audio

    // De-initiate GBA audio

    // Resize Buffer

    // Schedule First-in First out Direct Memory Access

/* Audio Related Registers */

pub mod registers {
    pub const REG_SOUND1CNT_L: u32 = 0x04000060;    // Sound 1 Sweep control
    pub const REG_SOUND1CNT_H: u32 = 0x04000062;    // Sound 1 Length, wave duty and envelope control
    pub const REG_SOUND1CNT_X: u32 = 0x04000064;    // Sound 1 Frequency, reset and loop control
    pub const REG_SOUND2CNT_L: u32 = 0x04000068;    // Sound 2 Length, wave duty and envelope control
    pub const REG_SOUND2CNT_H: u32 = 0x0400006C;    // Sound 2 Frequency, reset and loop control
    pub const REG_SOUND3CNT_L: u32 = 0x04000070;    // Sound 3 Enable and wave ram bank control
    pub const REG_SOUND3CNT_H: u32 = 0x04000072;    // Sound 3 Sound length and output level control
    pub const REG_SOUND3CNT_X: u32 = 0x04000074;    // Sound 3 Frequency, reset and loop control
    pub const REG_SOUND4CNT_L: u32 = 0x04000078;    // Sound 4 Length, output level and envelope control
    pub const REG_SOUND4CNT_H: u32 = 0x0400007C;    // Sound 4 Noise parameters, reset and loop control
    pub const REG_SOUNDCNT_L: u32 = 0x04000080;     // Sound 1-4 Output level and Stereo control
    pub const REG_SOUNDCNT_H: u32 = 0x04000082;     // Direct Sound control and Sound 1-4 output ratio
    pub const REG_SOUNDCNT_X: u32 = 0x04000084;     // Master sound enable and Sound 1-4 play status
    pub const REG_SOUNDBIAS: u32 = 0x04000088;      // Sound bias and Amplitude resolution control
    pub const REG_WAVE_RAM0_L: u32 = 0x04000090;	// Sound 3 samples 0-3
    pub const REG_WAVE_RAM0_H: u32 = 0x04000092;	// Sound 3 samples 4-7
    pub const REG_WAVE_RAM1_L: u32 = 0x04000094;	// Sound 3 samples 8-11
    pub const REG_WAVE_RAM1_H: u32 = 0x04000096;	// Sound 3 samples 12-15
    pub const REG_WAVE_RAM2_L: u32 = 0x04000098;	// Sound 3 samples 16-19
    pub const REG_WAVE_RAM2_H: u32 = 0x0400009A;	// Sound 3 samples 20-23
    pub const REG_WAVE_RAM3_L: u32 = 0x0400009C;	// Sound 3 samples 23-27
    pub const REG_WAVE_RAM3_H: u32 = 0x0400009E;	// Sound 3 samples 28-31
    pub const REG_FIFO_A_L: u32 = 0x040000A0;   	// Direct Sound channel A samples 0-1
    pub const REG_FIFO_A_H: u32 = 0x040000A2;       // Direct Sound channel A samples 2-3
    pub const REG_FIFO_B_L: u32 = 0x040000A4;	    // Direct Sound channel B samples 0-1
    pub const REG_FIFO_B_H: u32 = 0x040000A6;   	// Direct Sound channel B samples 2-3
    
    pub const fn local(address: u32) -> usize {
        address as usize - 0x4000000
    }
}

/* Write to Registers */

    // Write to REG_SOUND1CNT_L
    // Write to REG_SOUND1CNT_H
    // Write to REG_SOUND1CNT_X

    // Write to REG_SOUND2CNT_L
    // Write to REG_SOUND2CNT_H

    // Write to REG_SOUND3CNT_L
    // Write to REG_SOUND3CNT_H
    // Write to REG_SOUND3CNT_X

    // Write to REG_SOUND4CNT_L
    // Write to REG_SOUND4CNT_H

    // Write to REG_SOUNDCNT_L
    // Write to REG_SOUNDCNT_H
    // Write to REG_SOUNDCNT_X

    // Write to REG_SOUNDBIAS

    // Write to WaveRAM
    // Write to FIFO

/* Sample First-in First-out, Bias, Sample */
    // SampleFIFO
    // Apply Bias
    // Sample

/* Serialize, De-serialize, and Calculate ratio */
    // Serialize
    // Deserialize
    // Calculate Ratio
