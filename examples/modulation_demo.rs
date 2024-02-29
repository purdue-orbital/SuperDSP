use std::thread::spawn;
use rustdsp::elements::builder::PipelineBuilder;
use rustdsp::elements::data_trigger::DataTrigger;
use rustdsp::elements::frequency_demodulation::FrequencyDemodulation;
use rustdsp::elements::frequency_modulation::FrequencyModulation;
use rustdsp::elements::waterfall_chart::WaterfallChart;

fn main() {

    let sps = 16;
    let sample_rate = 500.0;
    let frequency = 125.0;

    let mut builder = PipelineBuilder::new();
    let (element, trigger) = DataTrigger::new();
    
    builder.add(element);
    builder.add(FrequencyModulation::new(sps,frequency,sample_rate));
    
    builder.add(WaterfallChart::new());
    
    builder.add(FrequencyDemodulation::new(sps,frequency,sample_rate,sps as f32 / 2.0));
    
    spawn(move || {
        loop {
            trigger.send([1.0].to_vec()).unwrap();
            trigger.send([0.0].to_vec()).unwrap();
        }
    });
    
    
    builder.build().run();
    
}