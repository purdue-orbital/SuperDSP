use num_complex::Complex;

pub struct PILoop{
    alpha: f32,
    beta: f32,
    integral_prev: Complex<f32>,
}

impl PILoop{
    pub fn new(proportional_gain: f32, integral_gain:f32) -> PILoop{
        PILoop{
            alpha: proportional_gain,
            beta: integral_gain,

            integral_prev: Complex::new(0.0,0.0)
        }
    }

    pub fn run(&mut self, sample: Complex<f32>) -> Complex<f32>{

        // calculate proportional and integral
        let p = self.alpha * sample;
        let i = self.beta * sample;

        // add loop back to integral
        let i_with_prev = i + self.integral_prev;

        // save delay of integral
        self.integral_prev = i_with_prev;

        p + i_with_prev
    }
}