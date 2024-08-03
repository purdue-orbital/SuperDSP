use core::f32;

pub fn make_basis_vector(i:f32, n:usize) -> ndarray::Array1<num::Complex<f32>> {
    let mut vector = ndarray::Array1::default(n);

    for j in 0..n{
        let theta = (-2.0 * core::f32::consts::PI * (i * j as f32)) / (n as f32); 
        vector[j] = num::Complex::new(libm::cosf(theta).trunc(), libm::sinf(theta).trunc()); 
    }

    vector.into()
}

pub fn make_basis(n:usize) -> ndarray::Array2<num::Complex<f32>>{
    let mut basis: ndarray::Array2<num::Complex<f32>> = ndarray::Array2::default((n,n)).into(); 
    
    for i in 0..n{
        let vector = make_basis_vector(i as f32,n);
        
        for j in 0..n{
            basis[[i,j]] = vector[j];
        }
    }

    basis * (1.0 / libm::sqrtf(n as f32))
}

pub fn make_inverse_basis(n: usize) -> ndarray::Array2<num::Complex<f32>>{
    let mut basis: ndarray::Array2<num::Complex<f32>> = ndarray::Array2::default((n,n));

    for i in 0..n{
        let vector = make_basis_vector(-(i as f32), n);

        for j in 0..n {
            basis[[i,j]] = vector[j];
        }
    }

    basis * (1.0 / libm::sqrtf(n as f32))
}

pub fn fft_shift(n:usize) -> ndarray::Array2<f32>{
    let mut shift: ndarray::Array2<f32> = ndarray::Array2::zeros((n,n));

    for i in 0..n{
        shift[[i,((i+(n/2) + 1) % n)]] = 1.0;
    }

    shift
}

pub fn fft_shift_inverse(n:usize) -> ndarray::Array2<f32>{
    let mut shift: ndarray::Array2<f32> = ndarray::Array2::zeros((n,n));

    for i in 0..n{
        shift[[i,(i + ((n-1)/2)) % n]] = 1.0;
    }

    shift
}
