//mod apu;
mod sound;

fn main(){
    let gba_audio = sound::GbaAudio::init_audio();
    print!("Initialized GbaAudio...\n");
    print!("Playing Test Tone...\n");
    sound::GbaAudio::test_tone(&gba_audio, 44, -22000.0, 0);
    
}