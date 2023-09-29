use std::f32::consts::PI;
use num_complex::Complex;
use rustdsp::common::constellation::{Constellation, ConstellationPoint};

#[test]
fn test_const_bpsk(){
    let one = Complex::new(1.0,0.0);
    let neg_one = Complex::new(-1.0,0.0);

    let zero_signal = vec![one,neg_one,one ,neg_one,one,neg_one];
    let one_signal = vec![neg_one,one,neg_one,one,neg_one,one];

    let mut c = Constellation::new(zero_signal.as_slice());

    let bin_zero = ConstellationPoint::new(0, 0.0, 0.0, 1.0);
    let bin_one = ConstellationPoint::new(1, PI, 0.0, 1.0);

    c.add_state(&bin_zero);
    c.add_state(&bin_one);

    let test1 = c.evaluate(one_signal.as_slice());
    let test2 = c.evaluate(zero_signal.as_slice());

    assert_eq!(test1,1);
    assert_eq!(test2,0);
}