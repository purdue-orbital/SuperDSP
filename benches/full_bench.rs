use criterion::{criterion_group, criterion_main};
use crate::ask::ask_benchmark;
use crate::fsk::fsk_benchmark;
use crate::qpsk::qpsk_benchmark;
use crate::bpsk::bpsk_benchmark;

mod qpsk;
mod bpsk;
mod ask;
mod fsk;


criterion_group!(benches, qpsk_benchmark);
criterion_group!(benches, bpsk_benchmark);
criterion_group!(benches, fsk_benchmark);
criterion_group!(benches, ask_benchmark);

criterion_main!(benches);
