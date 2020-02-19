#![allow(dead_code)]
use std::mem;

#[allow(dead_code)]
const GBA_AUDIO_SAMPLES: u16 = 2048; // 2048 Samples per second 
#[allow(dead_code)]
const GBA_AUDIO_FIFO_SIZE: u32 = 8 * mem::size_of::<i32>() as u32; // First-in First out Size
#[allow(dead_code)]
const GBA_AUDIO_VOLUME_MAX: i16 = 0x100; // Max volume
#[allow(dead_code)]
static CLOCKS_PER_FRAME: u32 = 0x800; // For syncing CPU clock time with Audio

