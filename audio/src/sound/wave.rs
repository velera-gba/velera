pub fn wave_table() {
    if wave.empty(){
            
        let sample3 = SamplesBuffer::new(1,44100, self.wavetable.to_vec());
        wave.append(sample3);
        wave.sleep_until_end();
    }
}