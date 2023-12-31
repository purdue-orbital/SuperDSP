use std::thread::{sleep, spawn};
use std::time::Duration;
use num_complex::Complex;
use rustdsp::elements::builder::PipelineBuilder;
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::time_chart::TimeChart;

fn main() {

    let sps = 10;
    let sample_rate = 1e6;
    let frequency = 1e3;

    let mut builder = PipelineBuilder::new();

    builder.add(
        SignalGenerator::new(
            frequency,
            sample_rate,
            sps
        )
    );

    builder.add(
        TimeChart::new(
            sps * 100
        )
    );

    let (tx, rx, mut pipeline) = builder.build(sps);

    spawn(move ||{
        let vec_complex = vec![Complex::new(0.0,0.0); sps];

        loop{
            tx.send(vec_complex.clone()).unwrap();

            sleep(Duration::from_millis( 10 ));
        }

    });

    pipeline.run();
}