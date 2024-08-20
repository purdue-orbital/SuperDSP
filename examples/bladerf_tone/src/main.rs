use rustdsp::objects::Pipeline;
use rustdsp::objects::wave_gen_complex::WaveStepGenComplex;
use rustdsp::objects::wave_gen_time_complex::WaveStepGenTimeComplex;
use rustdsp::radios;

fn main() {
    let mut pipeline = Pipeline::new();

    let mut radio = radios::bladerf::sink::BladeRfSink::new(916000000, 100000, 20, 200000, 1024);
    let mut gen = WaveStepGenComplex::new(100000.0 / 4.0, 1.0, 0.0, 100000.0);

    pipeline.add_object(&mut gen);
    pipeline.add_object(&mut radio);

    pipeline.process();
}