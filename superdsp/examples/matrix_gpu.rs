use rustdsp::gpu::pipeline::PipelineType::Loop;
use rustdsp::gpu::pipeline::{Pipeline, PipelineSettings};
use rustdsp::gpu::{list_devices, GpuType};
use rustdsp::prelude::matrix_multiplication::MatrixMultiplication;
use rustdsp::prelude::wave_gen::WaveGen;
use rustdsp::prelude::PipelineBuilder;

#[tokio::main]
async fn main() {
    let mut settings = PipelineSettings::default();

    let devices = list_devices();
    let i = devices.iter().position(|x| x.get_type() == GpuType::Discrete).unwrap();
    let device = devices[i].clone();

    settings.set_frequency(1.0)
        .set_sample_rate(16.0)
        .set_sps(8)
        .set_num_taps(8)
        .set_device(device)
    ;

    let mut pipeline: Pipeline<Vec<()>, Vec<()>> = PipelineBuilder::default()
        .add_first_stage(Box::new(WaveGen::default()))
        .add_last_stage(Box::new(MatrixMultiplication::default()))
        .build(Loop, &mut settings)
        .await;

    loop {}
}