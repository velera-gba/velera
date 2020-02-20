use rodio::Sample;
// Change Sample Volume
pub fn set_amplitude(samples: Vec<i16>, amplitude:f32) -> Vec<i16>{
    let mut result = vec!(0i16);
    for sample in samples{
        result.push(Sample::amplify(sample, amplitude));
    }
    print!("{:?}",result);
    return result;
}