use rustdsp::math::prelude::*;

#[test]
fn test_convolution(){
    // create builder
    let mut builder = WorkflowBuilder::default();

    // create elements
    let src1 = ElementParameter::new_f32_array(&[1.0,2.0,0.0,-3.0]);
    let src2 = ElementParameter::new_f32_array(&[1.0,1.0,1.0]);
    let mut dest = ElementParameter::new_f32_array(&[0.0,0.0,0.0,0.0,0.0,0.0,0.0]);

    // add convolution
    builder.convolution_f32(&src1, &src2, &mut dest);

    // build
    let mut pipeline = builder.build();

    // Run once
    pipeline.run();

    // test
    assert_eq!(dest.get_f32_array(),vec![1.0,3.0,3.0,-1.0,-3.0,-3.0])
}