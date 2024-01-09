use num_complex::Complex;

/// split a vector and reverse the two halves, combine them, then return that vector
pub fn split_reverse(signal: &mut [Complex<f32>]) {
    let half = signal.len() / 2;

    let first_half_copy = signal[..half].to_vec().clone();
    let second_half_ref = &signal[half..].to_vec();

    for index in 0..signal.len() {
        signal[index] = if half > index {
            second_half_ref[index]
        } else {
            first_half_copy[index - half]
        }
    }
}