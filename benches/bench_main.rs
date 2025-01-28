use crate::benchmarks::matrix_multiplication::benches;
use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benches,
}