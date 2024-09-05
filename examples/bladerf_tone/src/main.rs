use superdsp::objects::object::DSPObject;
use superdsp::radios;

fn main() {
    let mut gen = superdsp::objects::wave_gen_complex::WaveStepGenComplex::new(250_000.0, 1.0, 0.0, 1_000_000.0);
    let mut sink = radios::bladerf::sink::BladeRfSink::new(915000000, 1_000_000, 1_000_000, 1_000_000, 1024);

    let s = gen.get_bus();
    sink.set_bus(s);

    gen.start();
}