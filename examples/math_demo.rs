use rustdsp::math::builder::WorkflowBuilder;
use rustdsp::math::ElementParameter;

fn main() {
    let test1 = vec![1.0,2.0,3.0];
    let test2 = vec![0.0,1.0,0.5];
    let test3 = vec![1.0;4];

    let test4 = 8.0;

    let p1 = ElementParameter::new_f32_array(test1.as_slice());
    let p2 = ElementParameter::new_f32_array(test2.as_slice());
    let mut p3 = ElementParameter::new_f32_array(test3.as_slice());
    let p4 = ElementParameter::new_f32(test4);

    let mut builder = WorkflowBuilder::default();

    builder.convolution_f32(&p1, &p2, &mut p3);
    builder.scalar_multiply_f32(&p3, &p4);

    let mut flow = builder.build();

    flow.run();

    dbg!(p3.get_f32_array());
}