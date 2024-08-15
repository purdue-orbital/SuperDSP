use rustdsp::objects;
use rustdsp::objects::object::DSPObject;

#[test]
pub fn test_wave_gen() {
    let n = 4;
    let expected = Vec::from([0.0, 1.0, 0.0, -1.0]);
    let mut wave_gen = objects::wave_gen::WaveStepGen::new(1.0, 1.0, 0.0, 4.0);

    let mut result = Vec::new();
    for _ in 0..n {
        wave_gen.process();
        result.push(*wave_gen.get_output_buffer().lock());
    }

    assert_eq!(result, expected);
}