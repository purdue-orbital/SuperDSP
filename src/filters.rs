use alloc::vec::Vec;
use crate::math::complex::Complex;
use crate::math::matrix::Matrix;

/// This is the framework for window functions
pub trait WindowFunction{
    fn generate(&self) -> Vec<Complex<f32>>;
}

pub struct RectangleWindow{
    vector: Vec<Complex<f32>>,
}

impl RectangleWindow {

    /// Generate a rectangle window given certain parameters
    pub fn new(start_frequency: f32, stop_frequency: f32, sample_rate: f32, num_samples: usize) -> RectangleWindow{
        let start = ((start_frequency * (num_samples as f32 / sample_rate)) as usize) + (num_samples / 2);
        let stop = ((stop_frequency * (num_samples as f32 / sample_rate)) as usize) + (num_samples / 2);

        let mut vector = Vec::new();

        for x in 0..num_samples{
            vector.push(
                if x >= start && x <= stop{
                    Complex::new(1.0,0.0)
                }else {
                    Complex::new(0.0,0.0)
                }
            )
        }

        RectangleWindow{
            vector
        }
    }
}

impl WindowFunction for RectangleWindow {
    fn generate(&self) -> Vec<Complex<f32>> {
        self.vector.clone()
    }
}


/// Creates a matrix for filtering. Must be on the right side of a matrix in order and be after fft shift
pub fn create_filter_matrix_with_window<T>(function: T) -> Matrix<Complex<f32>>
where T: WindowFunction {
    let mut matrix_vector = Vec::new();
    
    // get vector that will be applied to the filter matrix
    let vector = function.generate();
    let len = vector.len();

    // apply vector to an identity matrix
    for i in 0..len {
        let mut row = Vec::new();
        for j in 0..len{
            if i == j {
                row.push(vector[i])
            }else {
                row.push(Complex::new(0.0,0.0))
            }
        }
        matrix_vector.push(row);
    }
    
    Matrix::from_vec(matrix_vector)
}