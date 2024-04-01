use std::thread::{sleep, spawn};
use std::time::Duration;
use rustdsp::elements::agc::AGC;
use rustdsp::elements::bit_trigger::BitTrigger;
use rustdsp::elements::builder::PipelineBuilder;
use rustdsp::elements::conditional_elements::ConditionalBitMatch;
use rustdsp::elements::data_bucket::DataBucket;
use rustdsp::elements::frequency_demodulation::FrequencyDemodulation;
use rustdsp::elements::frequency_modulation::FrequencyModulation;
use rustdsp::elements::gain::Gain;
use rustdsp::elements::sliding_buffer::{InverseSlidingBuffer, SlidingBuffer};
use rustdsp::elements::time_chart::TimeChart;
use rustdsp::elements::waterfall_chart::WaterfallChart;

fn main() {
    let sps = 16;
    let sample_rate = 500.0;
    let frequency = 125.0;

    let mut builder = PipelineBuilder::new();
    let (element, trigger) = BitTrigger::new();
    let (reset,conditional) = ConditionalBitMatch::new(&[0,0,0,0,1,0,0,1]);
    let (bucket,bucket_elem) = DataBucket::new(16);
    
    builder.add(element);
    builder.add(FrequencyModulation::new(sps,frequency,sample_rate));
    builder.add(Gain::new(0.1));
    //builder.add(TimeChart::new(160));
    builder.add(AGC::new(0.1,1.0,1.0,100.0));
    builder.add(TimeChart::new(160));
    builder.add(WaterfallChart::new());
    builder.add(FrequencyDemodulation::new(sps,frequency,sample_rate,8.0));
    builder.add(SlidingBuffer::new(8));
    builder.add(conditional);
    builder.add(InverseSlidingBuffer::new());
    builder.add(bucket_elem);
    
    spawn(move || {
        loop {
            trigger.send(vec![9]).unwrap();

            sleep(Duration::from_secs(1));
        }
    });

    spawn(move || {
        loop {
            let data = bucket.recv().unwrap();

            reset.send(()).unwrap();
            
            dbg!(&data);
        }
    });
    
    
    builder.build().run();
    
}