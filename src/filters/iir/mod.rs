use num_complex::Complex;
mod low_pass;
mod formula;

pub enum FormulaEnum {
    LowPass,
    HighPass
}

pub fn run_formula(formula: FormulaEnum, signal: &mut [Complex<f32>]) -> Vec<Complex<f32>> {
    let mut previous: Complex<f32> = Complex::new(0,0);
    for (index, signal_value) in signal[1..].iter().enumerate() {
        match formula {
            FormulaEnum::LowPass => (),
            FormulaEnum::HighPass => (),
        }
    }
}