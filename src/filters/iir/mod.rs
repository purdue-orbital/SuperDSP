use num_complex::Complex;

mod formula;

pub enum FormulaEnum {
    LowPass,
    HighPass,
}

/// Basic implementation of IIR filtering function. input_signal is iterated over, and for every value of the input signal, the coefficients are applies
/// The number of coefficients is the order of the filter, determined through other calculations.
pub fn general_form_iir(formula_type: FormulaEnum, input_signal: &[Complex<f32>], coefficients: &[f32]) -> Vec<Complex<f32>> {
    let mut output_signal: Vec<Complex<f32>> = Vec::new();
    for (index, input_signal_value) in input_signal.iter().enumerate() {
        let mut output_signal_value: Complex<f32> = coefficients[0] * input_signal_value;
        for (coefficient_num, coefficient) in coefficients[1..].iter().enumerate() {
            if index >= coefficient_num {
                output_signal_value += output_signal[index - coefficient_num] * coefficient;
            }
        }
        output_signal.push(output_signal_value);
    }
    output_signal
}