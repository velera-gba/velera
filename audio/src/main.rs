//mod apu;
mod sound;

fn main(){
    let gba_audio = sound::GbaAudio::init_audio();
    print!("Initializing GBA_Audio...\n");
    
    print!("Playing Test Tone...\n");
    //sound::GbaAudio::test_tone(&gba_audio, 440, 0);
    sound::GbaAudio::test_tone(&gba_audio, 22050, 1);
    /*
    let pulseA = Pulse::create();
    print!("Pulse A created [pulseA]...\n");
    let pulseB = Pulse::create();
    print!("Pulse B created [pulseB]...\n");

    let pulseA_envelope = Envelope::create();
    print!("New Envelope created [pulseA_envelope]...\n");

    print!("\n");
    print!("Pulse A parameters [pulseA]:\n");
    print!("Duty: {:?}", pulseA.get_duty());
    print!("Frequency: {}", pulseA.freq);

    print!("\n");
    print!("Pulse A Envelope Parameters [pulseA_envelope]:\n");
    print!("Time: {}", pulseA_envelope.time);
    */
}