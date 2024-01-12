use rustdsp::math::cpu::CPUCommandBuilder;

pub struct Test{
    src: Vec<f32>,
    dest: Vec<f32>
}

fn borrow_src(x: &mut Test) -> &mut [f32] {
    &mut x.src
}

fn borrow_dest(x: &mut Test) -> &mut [f32] {
    &mut x.dest
}

fn main() {

    let mut test = Test{
        src: vec![3.0;4],
        dest: vec![2.0;4],
    };

    let mut var = 0.0;
    
    let mut builder = CPUCommandBuilder::default();

    builder.elementwise_multiply_f32(borrow_src(&mut test), borrow_dest(&mut test));
    builder.scalar_multiply_f32(test.src.as_mut_slice(),&mut var);

    let mut pipeline = builder.build();

    pipeline.run();

    dbg!(borrow_dest(&mut test));
}