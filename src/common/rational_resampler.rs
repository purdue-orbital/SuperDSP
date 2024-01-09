use num_complex::Complex;

fn get_slope(first_amplitiude: Complex<f32>, second_amplitude: Complex<f32>, interval_size: u8) -> Complex<f32> {
    (second_amplitude - first_amplitiude) / (f32::from((interval_size + 1) - 0))
}

fn generate_values(slope: Complex<f32>, intercept: Complex<f32>, interval_size: u8) -> Vec<Complex<f32>> {
    let mut interval: Vec<Complex<f32>> = Vec::new();
    for x_value in 1..(interval_size + 1) {
        interval.push((slope * f32::from(x_value)) + intercept);
    }
    interval
}

fn insert_upsample_values(signal: &mut Vec<Complex<f32>>, upsample_values: Vec<Complex<f32>>, mut index: usize) {
    for value in upsample_values {
        signal.insert(index, value);
        index += 1;
    }
}

fn upsample(signal: &mut Vec<Complex<f32>>, upsample_factor: usize) {
    let original_length = signal.len();
    let mut index: usize = 0;

    while signal.len() < original_length * upsample_factor
    {
        let intercept = signal[index];
        let slope = get_slope(intercept, signal[index] + (1 as f32), upsample_factor as u8);

        let insertion_values: Vec<Complex<f32>> = generate_values(slope, intercept, upsample_factor as u8);

        let insertion_interval_length = insertion_values.len();

        insert_upsample_values(signal, insertion_values, index);

        index += insertion_interval_length;
    }
    if signal.len() - 1 == original_length * upsample_factor
    {
        signal.pop();
    }
}

fn downsample(signal: &mut Vec<Complex<f32>>, downsample_factor: usize) {
    let original_length = signal.len();
    let mut index: usize = 0;

    while signal.len() > (original_length / downsample_factor) && index < signal.len()
    {
        for i in 0..(downsample_factor - 1) {
            if index < signal.len()
            {
                print!("{} ", signal[index]);
                signal.remove(index);
            }
        }
        index += 1;
    }
    if signal.len() - 1 == original_length / downsample_factor
    {
        signal.pop();
    }
    println!("{}", signal.len());
}

/// upsample value = size * 3, downsample value means size / value
pub fn rational_resample(mut upsample_factor: u8, mut downsample_factor: u8, signal: &mut Vec<Complex<f32>>)
{
    if upsample_factor == 0 {
        upsample_factor += 1;
    }
    if downsample_factor == 0 {
        downsample_factor += 1;
    }

    if upsample_factor > 1 {
        upsample(signal, usize::from(upsample_factor));
    }
    if downsample_factor > 1 {
        downsample(signal, usize::from(downsample_factor));
    }
} 