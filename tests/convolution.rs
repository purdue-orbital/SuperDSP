use num_complex::Complex;
use rustdsp::common::convolution;

#[test]
pub fn convolution_test(){
    let hold = Complex::new(1.0,1.0);

    let out = convolution(&[hold,2.0*hold,3.0*hold],&[0.0*hold,1.0*hold,0.5*hold]);

    assert_eq!(out, [0.0*hold,hold,2.5*hold,4.0*hold,1.5*hold])
}