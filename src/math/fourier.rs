use core::f64;
use num::Complex;

#[cfg(not(feature = "std"))]
pub fn calculate_root_of_unity(n: f64, i: f64, j: f64) -> num::Complex<f64> {
    let theta = (-2.0 * core::f64::consts::PI * (i * j)) / n;
    num::Complex::new(libm::cos(theta), libm::sin(theta))
}

#[cfg(feature = "std")]
pub fn calculate_root_of_unity(n: f64, i: f64, j: f64) -> num::Complex<f64> {
    let theta = (-2.0 * core::f64::consts::PI * (i * j)) / n;
    num::Complex::new(theta.cos(), theta.sin())
}

pub fn make_basis_vector(i: f64, n: usize) -> ndarray::Array1<num::Complex<f64>> {
    let mut vector = ndarray::Array1::default(n);

    for j in 0..n {
        vector[j] = calculate_root_of_unity(n as f64, i, j as f64);
    }

    vector
}

pub fn make_basis(n: usize) -> ndarray::Array2<num::Complex<f64>> {
    let mut basis: ndarray::Array2<num::Complex<f64>> = ndarray::Array2::default((n, n));

    for i in 0..n {
        let vector = make_basis_vector(i as f64, n);

        for j in 0..n {
            basis[[i, j]] = vector[j];
        }
    }

    basis * (1.0 / libm::sqrt(n as f64))
}

pub fn make_inverse_basis(n: usize) -> ndarray::Array2<num::Complex<f64>> {
    let mut basis: ndarray::Array2<num::Complex<f64>> = ndarray::Array2::default((n, n));

    for i in 0..n {
        let vector = make_basis_vector(-(i as f64), n);

        for j in 0..n {
            basis[[i, j]] = vector[j];
        }
    }

    basis * (1.0 / libm::sqrt(n as f64))
}

pub fn fft_shift(n: usize) -> ndarray::Array2<Complex<f64>> {
    let mut shift: ndarray::Array2<Complex<f64>> = ndarray::Array2::zeros((n, n));

    for i in 0..n {
        shift[[i, ((i + (n / 2) + 1) % n)]] = Complex::new(1.0,0.0);
    }

    shift
}

pub fn fft_shift_inverse(n: usize) -> ndarray::Array2<Complex<f64>> {
    let mut shift: ndarray::Array2<Complex<f64>> = ndarray::Array2::zeros((n, n));

    for i in 0..n {
        shift[[i, (i + ((n - 1) / 2)) % n]] = Complex::new(1.0,0.0);
    }

    shift
}
