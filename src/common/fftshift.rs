use num_complex::Complex;

/// split a vector and reverse the two halves, combine them, then return that vector
pub fn split_reverse(signal: &mut[Complex<f32>]) -> Vec<Complex<f32>> {
    let (first_half, second_half) = signal.split_at_mut(signal.len() / 2);

    let mut combined = second_half.to_vec();

    combined.append(&mut first_half.to_vec());

    combined
}