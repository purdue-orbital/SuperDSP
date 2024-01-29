use std::thread::{sleep, spawn};
use std::time::Duration;

use rustdsp::elements::builder::PipelineBuilder;
use rustdsp::elements::constellation_chart::ConstellationChart;
use rustdsp::elements::lpf::LPF;
use rustdsp::elements::signal_adder::SignalAdder;
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::time_chart::TimeChart;
use rustdsp::elements::waterfall_chart::WaterfallChart;

fn main() {
    let sps = 1024;
    let sample_rate = 100e3;
    let frequency1 = 40e3;
    let frequency2 = 1e3;

    let mut builder = PipelineBuilder::new();

    builder.add(
        SignalGenerator::new(
            frequency1,
            sample_rate,
            sps,
        )
    );

    builder.add(
        SignalAdder::new(
            frequency2,
            sample_rate,
            sps
        )
    );

    builder.add(
        TimeChart::new(
            1000
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

    builder.add(
      LPF::new(30e3,sample_rate,0.1)
    );

    builder.add(
        TimeChart::new(
            1000
        )
    );



    let mut pipeline = builder.build();

    spawn(move || {
        loop {
            sleep(Duration::from_secs_f32(sps as f32 / sample_rate));
        }
    });

    pipeline.run();
}