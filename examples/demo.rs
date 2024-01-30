use std::thread::{sleep, spawn};
use std::time::Duration;
use rustdsp::elements::builder::{PipelineBuilder, Pipeline};

use rustdsp::elements::constellation_chart::ConstellationChart;
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::time_chart::TimeChart;
use rustdsp::elements::waterfall_chart::WaterfallChart;
use rustdsp::math::builder::WorkflowBuilder;

fn main() {
    let sps = 10;
    let sample_rate = 1e3;
    let frequency = 0.5;

    let mut builder = PipelineBuilder::new();

    builder.add()
}