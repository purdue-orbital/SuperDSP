// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use rustdsp::math::prelude::*;
// use std::sync::{Arc, Mutex};
// 
// fn fetch_f32_benchmark(c: &mut Criterion) {
//     let mut group = c.benchmark_group("FetchF32");
//     let mut operation = FetchF32::new();
//     let mut data = Data {
//         f32_arrays: vec![
//             Arc::new(Mutex::new(vec![1.0, 2.0, 3.0, 4.0, 5.0])),
//             Arc::new(Mutex::new(vec![0.0, 1.0, 2.0, 3.0, 4.0])),
//             Arc::new(Mutex::new(vec![0.0; 5])),
//         ],
//         f32_const: vec![],
//     };
// 
//     group.bench_function("run", |b| b.iter(|| operation.run(black_box(&mut data))));
//     group.finish();
// }
// 
// criterion_group!(benches, fetch_f32_benchmark);
// criterion_main!(benches);