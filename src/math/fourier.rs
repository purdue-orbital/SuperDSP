use core::f32;
use nalgebra::SMatrix;

use num::Complex;

#[cfg(not(feature = "std"))]
pub fn calculate_root_of_unity(n: f32, i: f32, j: f32) -> num::Complex<f32> {
    let theta = (-2.0 * core::f32::consts::PI * (i * j)) / n;
    num::Complex::new(libm::cosf(theta), libm::sinf(theta))
}

#[cfg(feature = "std")]
pub fn calculate_root_of_unity(n: f32, i: f32, j: f32) -> num::Complex<f32> {
    let theta = (-2.0 * core::f32::consts::PI * (i * j)) / n;
    num::Complex::new(theta.cos(), theta.sin())
}

pub fn make_basis_vector<const N: usize>(i: f32) -> SMatrix<Complex<f32>, N, 1>{
    let mut vector = SMatrix::<Complex<f32>, N, 1>::zeros();

    let scalar = 1.0 / libm::sqrtf(N as f32);

    for j in 0..N {
        vector[j] = calculate_root_of_unity(N as f32, i, j as f32) * scalar;
    }

    vector
}

pub fn make_basis<const N: usize>() -> SMatrix<Complex<f32>, N, N> {
    let mut basis = SMatrix::<Complex<f32>, N, N>::zeros();

    for i in 0..N {
        let vector: SMatrix<Complex<f32>, N,1> = make_basis_vector(i as f32);

        for j in 0..N {
            basis[i + (j * N)] = vector[j];
        }
    }

    basis
}

pub fn make_inverse_basis<const N: usize>() -> SMatrix<Complex<f32>, N, N> {
    let mut basis = SMatrix::<Complex<f32>, N, N>::zeros();

    for i in 0..N {
        let vector: SMatrix<Complex<f32>, N,1> = make_basis_vector(-(i as f32));

        for j in 0..N {
            basis[i + (j * N)] = vector[j];
        }
    }

    basis
}

fn make_forward_shift<const N: usize>(mv_amount: usize) -> SMatrix<Complex<f32>, N, N> {
    let mut shift = SMatrix::<Complex<f32>, N, N>::zeros();
    
    for i in 0..N{
        shift[((i + mv_amount) % N) + (i * N)] = Complex::new(1.0, 0.0);
    }

    shift
}

pub fn fft_shift<const N: usize>() -> SMatrix<Complex<f32>, N, N>  {
    let uneven_split = (N - 1) / 2;

    make_forward_shift(uneven_split)
}

pub fn fft_shift_inverse<const N: usize>() -> SMatrix<Complex<f32>, N, N>  {
    let uneven_split = N - ((N - 1) / 2);

    make_forward_shift(uneven_split)
}
