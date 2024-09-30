use nalgebra::SMatrix;
use superdsp::math::correlation::create_autocorrelation_matrix;
use num::Complex;

#[test]
pub fn autocorrelation(){
    let wave = SMatrix::from([Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)]);

    let flip_wave = SMatrix::from([Complex::new(-1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0), Complex::new(1.0, 0.0)]);
    
    let bad_wave = SMatrix::from([Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-0.5, 0.0), Complex::new(0.5, 0.0)]);

    let autocorrelation = create_autocorrelation_matrix(wave);
    assert_eq!(autocorrelation[(0,0)], Complex::new(1.0, 0.0));


    dbg!(autocorrelation * wave);
    dbg!(autocorrelation * flip_wave);
    dbg!(autocorrelation * bad_wave);
    
}