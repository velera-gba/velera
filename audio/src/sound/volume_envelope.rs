struct VolumeEnvelope {
    time : u8,         //Current time/position of the envelope
    mode : bool,       //Decrease or Increase volume
    step_time : u8,    //Amount increased or decreased per step
    initial : u8,      //Initial Volume of the Envelope; 0-15
    volume : u8,       //Current Volume of the Envelope          
}

impl VolumeEnvelope {
    fn create() -> VolumeEnvelope {
        VolumeEnvelope {
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