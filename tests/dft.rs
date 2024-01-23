use num_complex::Complex;
use rustdsp::elements::macros::dft::{dft, idft};
use rustdsp::math::prelude::*;

#[test]
fn test_dft_round(){
    // create builder
    let mut builder = WorkflowBuilder::default();

    // create elements
    let src = ComplexF32::new(vec![Complex::new(0.0,0.0),Complex::new(1.0,0.0),Complex::new(0.0,0.0),Complex::new(-1.0,0.0)]);
    let dest = ComplexF32::new(vec![Complex::new(0.0,0.0),Complex::new(0.0,0.0),Complex::new(0.0,0.0),Complex::new(0.0,0.0)]);

    // add dft
    dft(&mut builder,&src,&dest,dest.to_vec().len());

    idft(&mut builder,&dest,&src,src.to_vec().len());

    // build
    let mut pipeline = builder.build();

    // Run once
    pipeline.run();

    assert_eq!(src.to_vec(), vec![Complex::new(0.0,0.0),Complex::new(1.0,0.0),Complex::new(0.0,0.0),Complex::new(-1.0,0.0)]);
}