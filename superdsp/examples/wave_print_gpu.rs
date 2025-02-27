use superdsp::gpu::pipeline::PipelineType::Loop;
use superdsp::gpu::pipeline::{Pipeline, PipelineSettings};
use superdsp::gpu::{list_devices, GpuType};
use superdsp::prelude::print_debug::PrintDebug;
use superdsp::prelude::wave_gen::WaveGen;
use superdsp::prelude::PipelineBuilder;

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
        .add_first_stage::<WaveGen<Vec<()>>>()
        .add_last_stage::<PrintDebug<Vec<()>>>()
        .build(Loop, &mut settings)
        .await;

    loop {}
}