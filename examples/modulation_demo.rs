use std::thread::spawn;
use num_complex::Complex;
use rustdsp::elements::builder::PipelineBuilder;
use rustdsp::elements::code_sink::{CodeSinkF32Array};
use rustdsp::elements::data_trigger::{DataTriggerComplex};
use rustdsp::elements::waterfall_chart::WaterfallChart;

fn main() {
    let sps = 16;
    let sample_rate = 500.0;
    let frequency = 125.0;

    let mut builder = PipelineBuilder::new();
    let (element, trigger) = DataTriggerComplex::new(sps);
    let (sink_elem, rx) = CodeSinkF32Array::new();
    
    builder.add(element);
    builder.add(WaterfallChart::new());
    
    
    spawn(move || {
        loop {
            trigger.send(vec![Complex::new(0.0,0.0);16]).unwrap();
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