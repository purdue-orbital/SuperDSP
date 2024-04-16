use std::f32::consts::PI;
use rustdsp::dft::dft_with_shift;
use rustdsp::filters::{create_filter_matrix_with_window, RectangleWindow};
use rustdsp::math::expj;
use rustdsp::math::matrix::Matrix;

#[test]
pub fn rectangle(){
    let frequency = 2.0;
    let sample_rate = 4.0;
    let num_samples = 16;
    let inverse_sample_rate =  1.0 / sample_rate;

    let mut wave = Vec::new();
    for x in 0..num_samples{
        wave.push(expj(x as f32 * 2.0 * PI * frequency * inverse_sample_rate))
    }
    let input = Matrix::from_vec(vec![wave]);
    
    let rec = RectangleWindow::new(-1.0,1.0,sample_rate,num_samples);
    let filter = create_filter_matrix_with_window(rec);
    
    let main_matrix = dft_with_shift(num_samples) * filter;
    
    dbg!(input * main_matrix);
}