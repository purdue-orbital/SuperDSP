use rustdsp::math::cpu::CPUCommandBuilder;

fn main() {
    let mut src = vec![3.0;4];
    let mut dest = vec![2.0;4];
    
    let mut builder = CPUCommandBuilder::default();
    
    builder.elementwise_multiply_f32(src.as_mut_slice(), dest.as_mut_slice());

    let mut pipeline = builder.build();

    pipeline.run();

    dbg!(dest);
}