use std::thread::{sleep, spawn};
use std::time::Duration;
use rustdsp::elements::builder::{PipelineBuilder, Pipeline};
use rustdsp::elements::lpf::LPF;
use rustdsp::elements::bpf::BPF;

use rustdsp::elements::constellation_chart::ConstellationChart;
use rustdsp::elements::signal_adder::SignalAdder;
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::time_chart::TimeChart;
use rustdsp::elements::waterfall_chart::WaterfallChart;
use rustdsp::math::builder::WorkflowBuilder;

fn main() {
    let sps = 100;
    let sample_rate = 1e3;
    let frequency = 1.0;

    let mut builder = PipelineBuilder::new();
    let mut signal_generator: SignalGenerator = SignalGenerator::new(frequency, sample_rate, sps); // samples per system
    let mut signal_2: SignalAdder = SignalAdder::new(50.0, sample_rate, sps);
    let mut signal_3: SignalAdder = SignalAdder::new(100.0, sample_rate, sps);

    builder.add(signal_generator);
    builder.add(signal_2);
    builder.add(signal_3);

    builder.add(WaterfallChart::new(sps));
    builder.add(BPF::new(sample_rate, 1.0, 45.0, 55.0));
    builder.add(WaterfallChart::new(sps));
    

    builder.build().run();
}