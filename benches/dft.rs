use criterion::{Criterion, criterion_group, criterion_main};
use num_complex::Complex;
use rustdsp::elements::prefabs::dft::{dft, idft};
use rustdsp::elements::prefabs::wave_generators::wave_generator_complex_time_banked;
use rustdsp::math::prelude::*;

fn build() -> Workflow{
    let mut builder = WorkflowBuilder::default();

    let src = wave_generator_complex_time_banked(&mut builder,32e3,1e3,1024);
    let dest = ComplexF32::new(vec![Complex::new(0.0,0.0);1024]);

    // add dft
    dft(&mut builder, &src, &dest);
    idft(&mut builder,&dest,&src);

    // build
    builder.build()
}

fn bench_dft(c: &mut Criterion) {

    let mut pipeline = build();

    c.bench_function("dft", |b| {
        b.iter(|| {
            pipeline.run();
        });
    });
}

criterion_group!(benches, bench_dft);
criterion_main!(benches);