use std::thread::spawn;
use rustdsp::elements::builder::PipelineBuilder;
use rustdsp::elements::code_sink::{CodeSinkF32Array};
use rustdsp::elements::data_trigger::DataTrigger;
use rustdsp::elements::frequency_demodulation::FrequencyDemodulation;
use rustdsp::elements::frequency_modulation::FrequencyModulation;
use rustdsp::elements::sliding_buffer::SlidingBuffer;

fn main() {
    let sps = 16;
    let sample_rate = 500.0;
    let frequency = 125.0;

    let mut builder = PipelineBuilder::new();
    let (element, trigger) = DataTrigger::new();
    let (sink_elem, rx) = CodeSinkF32Array::new();
    
    builder.add(element);
    builder.add(FrequencyModulation::new(sps,frequency,sample_rate));
    
    builder.add(FrequencyDemodulation::new(sps,frequency,sample_rate,sps as f32 / 2.0));
    
    builder.add(SlidingBuffer::new(16));
    
    builder.add(sink_elem);
    
    
    spawn(move || {
        loop {
            trigger.send([1.0].to_vec()).unwrap();
            trigger.send([0.0].to_vec()).unwrap();
        }
    });

    spawn(move || {
        loop {
            let data = rx.recv().unwrap();
            
            dbg!(&data);
        }
    });
    
    
    builder.build().run();
    
}