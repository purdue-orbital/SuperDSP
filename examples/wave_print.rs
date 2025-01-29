use rustdsp::pipeline::PipelineType::OnRecv;
use rustdsp::pipeline::{PipelineBuilder, PipelineSettings};
use rustdsp::prelude::print::Print;
use rustdsp::prelude::wave_gen::WaveGen;
use rustdsp::prelude::Pipeline;

#[tokio::main]
async fn main() {
    let mut settings = PipelineSettings::default();

    settings.set_frequency(1.0)
        .set_sample_rate(2.0)
        .set_sps(1)
        .set_num_taps(8);

    let mut pipeline: Pipeline<Vec<()>, Vec<f64>> = PipelineBuilder::default()
        .add_first_stage(Box::new(WaveGen::new()))
        .add_last_stage(Box::new(Print::default()))
        .build(OnRecv, &mut settings)
        .await;

    loop {
        let out = pipeline.recv().await.unwrap();
        println!("{:?}", out);
    }
}