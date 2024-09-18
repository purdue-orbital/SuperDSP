use core::f32;

#[cfg(feature = "fixed-point")]
use fixed::types::I16F16;

use nalgebra::SMatrix;
#[cfg(feature = "std")]
use nalgebra::DMatrix;
use num::Complex;

#[cfg(not(feature = "fixed-point"))]
pub fn calculate_root_of_unity(n: f32, i: f32, j: f32) -> Complex<f32> {
    let theta = (-2.0 * f32::consts::PI * (i * j)) / n;
    Complex::new(libm::cosf(theta), libm::sinf(theta))
}

#[cfg(not(feature = "fixed-point"))]
pub fn make_basis_vector<const N: usize>(i: f32) -> SMatrix<Complex<f32>, N, 1> {
    let mut vector = SMatrix::<Complex<f32>, N, 1>::zeros();

    let scalar = 1.0 / libm::sqrtf(N as f32);

    for j in 0..N {
        vector[j] = calculate_root_of_unity(N as f32, i, j as f32) * scalar;
    }

    vector
}

#[cfg(not(feature = "fixed-point"))]
pub fn make_basis<const N: usize>() -> SMatrix<Complex<f32>, N, N> {
    let mut basis = SMatrix::<Complex<f32>, N, N>::zeros();

    for i in 0..N {
        let vector: SMatrix<Complex<f32>, N, 1> = make_basis_vector(i as f32);

        for j in 0..N {
            basis[i + (j * N)] = vector[j];
        }
    }

    basis
}

#[cfg(not(feature = "fixed-point"))]
#[cfg(feature = "std")]
pub fn dynamic_make_basis<const N: usize>() -> DMatrix<Complex<f32>> {
    let s = make_basis::<N>();
    static_to_dynamic(&s)
}

#[cfg(not(feature = "fixed-point"))]
pub fn make_inverse_basis<const N: usize>() -> SMatrix<Complex<f32>, N, N> {
    let mut basis = SMatrix::<Complex<f32>, N, N>::zeros();

    for i in 0..N {
        let vector: SMatrix<Complex<f32>, N, 1> = make_basis_vector(-(i as f32));

        for j in 0..N {
            basis[i + (j * N)] = vector[j];
        }
    }

    basis
}

#[cfg(not(feature = "fixed-point"))]
pub fn make_forward_shift<const N: usize>(mv_amount: usize) -> SMatrix<Complex<f32>, N, N> {
    let mut shift = SMatrix::<Complex<f32>, N, N>::zeros();

    for i in 0..N {
        shift[((i + mv_amount) % N) + (i * N)] = Complex::new(1.0, 0.0);
    }

    shift
}
#[cfg(not(feature = "fixed-point"))]
pub fn fft_shift<const N: usize>() -> SMatrix<Complex<f32>, N, N> {
    let uneven_split = (N - 1) / 2;

    make_forward_shift(uneven_split)
}
#[cfg(not(feature = "fixed-point"))]
pub fn fft_shift_inverse<const N: usize>() -> SMatrix<Complex<f32>, N, N> {
    let uneven_split = N - ((N - 1) / 2);

    make_forward_shift(uneven_split)
}
#[cfg(not(feature = "fixed-point"))]
#[cfg(feature = "std")]
pub fn static_to_dynamic<const N: usize>(s: &SMatrix<Complex<f32>,N,N>) -> DMatrix<Complex<f32>> {
    let mut d = DMatrix::<Complex<f32>>::zeros(N, N);
    
    for i in 0..N {
        for j in 0..N {
            d[(i, j)] = s[(i, j)];
        }
    }
    
    d
}
#[cfg(not(feature = "fixed-point"))]
#[cfg(feature = "std")]
pub fn fft_shift_dynamic<const N: usize>() -> DMatrix<Complex<f32>> {
    let s = fft_shift::<N>();
    
    static_to_dynamic(&s)
}
#[cfg(not(feature = "fixed-point"))]
#[cfg(feature = "std")]
pub fn fft_shift_inverse_dynamic<const N: usize>() -> DMatrix<Complex<f32>> {
    let s = fft_shift::<N>();

    static_to_dynamic(&s)
}

#[cfg(feature = "fixed-point")]
pub fn calculate_root_of_unity(n: I16F16, i: I16F16, j: I16F16) -> Complex<I16F16> {
    let theta = (I16F16::from(-2i8) * I16F16::PI * (i * j)) / n;
    Complex::new(cordic::cos(theta), cordic::sin(theta))
}

#[cfg(feature = "fixed-point")]
pub fn make_basis_vector<const N: usize>(i: I16F16) -> SMatrix<Complex<I16F16>, N, 1> {
    let mut vector = SMatrix::<Complex<I16F16>, N, 1>::from([[Complex::new(I16F16::from(0u8), I16F16::from(0u8)); N]; 1]);

    let scalar: I16F16 = I16F16::from(1i8) / I16F16::from(N as i16).sqrt();

    for j in 0..N {
        let val = calculate_root_of_unity(I16F16::from(N as i16), i, I16F16::from(j as i16));
        vector[j] = Complex::new(val.re * scalar, val.im * scalar);
    }

    vector
}

#[cfg(feature = "fixed-point")]
pub fn make_basis<const N: usize>() -> SMatrix<Complex<I16F16>, N, N> {
    let mut basis = SMatrix::<Complex<I16F16>, N, N>::from([[Complex::new(I16F16::from(0u8), I16F16::from(0u8)); N]; N]);

    for i in 0..N {
        let vector: SMatrix<Complex<I16F16>, N, 1> = make_basis_vector(I16F16::from(i as i16));

        for j in 0..N {
            basis[i + (j * N)] = vector[j];
        }
    }

    basis
}