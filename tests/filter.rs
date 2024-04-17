use rustdsp::basic::dft::dft_with_shift;
use rustdsp::basic::etc::generate_wave;
use rustdsp::basic::filters::{create_filter_matrix_with_window, RectangleWindow};

#[test]
pub fn rectangle() {
    let frequency = 1.0;
    let sample_rate = 4.0;
    let num_samples = 16;

    let matrix = dft_with_shift(num_samples);
    let filter1 = create_filter_matrix_with_window(RectangleWindow::new(1.0, 1.0, sample_rate, num_samples));
    let filter2 = create_filter_matrix_with_window(RectangleWindow::new(-1.0, -1.0, sample_rate, num_samples));
    let input = generate_wave(frequency, sample_rate, 0.0, num_samples);

    let main_matrix1 = matrix.clone() * filter1;
    let main_matrix2 = matrix * filter2;

    assert_eq!((input.clone() * main_matrix1).cpu_matrix[0][12].re, 4.0);
    assert_eq!((input * main_matrix2).cpu_matrix[0][12].re, 0.0);
}