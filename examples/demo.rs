use std::thread::{sleep, spawn};
use std::time::Duration;

use rustdsp::elements::builder::PipelineBuilder;
use rustdsp::elements::constellation_chart::ConstellationChart;
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::time_chart::TimeChart;
use rustdsp::elements::waterfall_chart::WaterfallChart;

fn main() {
    let sps = 10;
    let sample_rate = 1e3;
    let frequency = 0.5;

    let mut builder = PipelineBuilder::new();

    builder.add(
        SignalGenerator::new(
            frequency,
            sample_rate,
            sps,
        )
    );

    builder.add(
        TimeChart::new(
            (sample_rate / frequency) as usize
        )
    );

    builder.add(
        ConstellationChart::new(
            sps * 100
        )
    );

    builder.add(
        WaterfallChart::new(
            sps
        )
    );


    let mut pipeline = builder.build(sps);

    spawn(move || {
        loop {
            sleep(Duration::from_secs_f32(sps as f32 / sample_rate));
        }
    });

    pipeline.run();
}