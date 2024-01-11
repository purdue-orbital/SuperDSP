use rustdsp::math::builder::WorkflowBuilder;
use rustdsp::math::ElementParameter;

fn main() {
    let test1 = vec![1.0,2.0,3.0];
    let test2 = vec![0.0,1.0,0.5];
    let test3 = vec![1.0;4];

    let mut p1 = ElementParameter::new_f32(test1.as_slice());
    let mut p2 = ElementParameter::new_f32(test2.as_slice());
    let mut p3 = ElementParameter::new_f32(test3.as_slice());

    let mut builder = WorkflowBuilder::default();

    builder.convolution_f32(&mut p1, &mut p2, &mut p3);

    let mut flow = builder.build();

    flow.run();
    flow.run();

    dbg!(p3.get_f32_array());
}