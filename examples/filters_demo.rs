use rustdsp::elements::builder::{PipelineBuilder};
use rustdsp::elements::gain::Gain;
use rustdsp::elements::signal_generator::SignalGenerator;
use rustdsp::elements::time_chart::TimeChart;
use rustdsp::elements::waterfall_chart::WaterfallChart;
use rustdsp::elements::interpolator::Interpolator;

fn main() {
    let sps = 16;
    let sample_rate = 500.0;
    let frequency = 125.0;

    let mut builder = PipelineBuilder::new();
    let signal_generator: SignalGenerator = SignalGenerator::new(frequency, sample_rate, sps, false); // samples per symbol

    builder.add(signal_generator);
    
    builder.add(WaterfallChart::new());

    builder.add(Interpolator::new(16, sample_rate, frequency, 1.0));
    builder.add(Gain::new(16.0));

    builder.add(WaterfallChart::new());
    
    builder.add(TimeChart::new(10 * 2 * sps));
    

    builder.build().run();
}