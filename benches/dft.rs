use criterion::{Criterion, criterion_group, criterion_main};
use num_complex::Complex;
use rustdsp::elements::macros::dft::{dft, idft};
use rustdsp::math::prelude::*;

fn bench_dft(c: &mut Criterion) {

    let mut builder = WorkflowBuilder::default();
    let cmp = [Complex::new(1.0,0.0),Complex::new(-1.0,0.0),Complex::new(1.0,0.0),Complex::new(-1.0,0.0)];
    // create elements
    let src = ComplexF32::new(cmp.to_vec());
    let dest = ComplexF32::new(vec![Complex::new(0.0,0.0),Complex::new(0.0,0.0),Complex::new(0.0,0.0),Complex::new(0.0,0.0)]);

    // add dft
    dft(&mut builder, &src, &dest);

    idft(&mut builder,&dest,&src);

    // build
    let mut pipeline = builder.build();

    c.bench_function("dft", |b| {
        b.iter(|| {
            pipeline.run();
        });
    });
}

criterion_group!(benches, bench_dft);
criterion_main!(benches);