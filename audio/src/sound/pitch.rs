use rodio::Sample;
// Change Sample Pitch
pub fn set_frequency(samples: Vec<i16>, frequency:u32) -> Vec<i16>{
    let mut result = vec!(0i16,);
    let length = samples.len() as u32;
    for sample in samples{
        result.push(Sample::lerp(sample, sample, 1, 1));
    };

    return result;
}