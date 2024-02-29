use num_complex::{Complex, ComplexFloat};
use rustdsp::elements::prefabs::dft::{dft, idft};
use rustdsp::math::prelude::*;

#[test]
fn test_dft_round(){
    // create builder
    let mut builder = WorkflowBuilder::default();

    let cmp = [Complex::new(1.0,0.0),Complex::new(-1.0,0.0),Complex::new(1.0,0.0),Complex::new(-1.0,0.0)];

    // create elements
    let src = ComplexF32::new(cmp.to_vec());
    let dest = ComplexF32::new(vec![Complex::new(0.0,0.0),Complex::new(0.0,0.0),Complex::new(0.0,0.0),Complex::new(0.0,0.0)]);

    // add dft
    dft(&mut builder,&src,&dest);

    idft(&mut builder,&dest,&src);

    // build
    let mut pipeline = builder.build();

    // Run once
    pipeline.run();

    dbg!(src.to_vec());

    for (index, x) in  src.to_vec().iter().enumerate(){
        assert!((cmp[index] - x).abs() < 0.001);
    }
}