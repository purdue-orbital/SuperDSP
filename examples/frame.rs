use std::thread::{sleep, spawn};
use std::time::Duration;
use num_complex::Complex;
use rand::random;
use rustdsp::elements::builder::PipelineBuilder;
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::time_chart::TimeChart;
use rustdsp::ui::charts::builder::WindowBuilder;
use rustdsp::ui::charts::line_chart::LineChart;

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
            sps * 10
        )
    );

    let mut pipeline = builder.build(sps);


    let mut vec_complex = vec![Complex::new(0.0,0.0); sps];
    let sliced = vec_complex.as_mut_slice();

    loop{
        pipeline.run(sliced);

        sleep(Duration::from_secs_f64( sps as f64 / sample_rate as f64 ));
    }
}