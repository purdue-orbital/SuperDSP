use nalgebra::{Complex, SMatrix};

fn hermitian_transpose<const N: usize>(vector: SMatrix<Complex<f32>, N,1>) -> SMatrix<Complex<f32>,1,N> {
    let mut out = SMatrix::<Complex<f32>,1,N>::zeros();

    for i in 0..N {
        out[i] = vector[i].conj();
    }

    out
}

pub fn create_autocorrelation_matrix<const N: usize>(target_wave: SMatrix<Complex<f32>,N,1>) -> SMatrix<Complex<f32>,N,N> {
    target_wave * hermitian_transpose(target_wave)
}