use num_complex::Complex;

pub struct Interpolator{
    sample_rate:f32,
    inverse_sample_rate:f32
}

impl Interpolator{
    pub fn new(sample_rate:f32) -> Interpolator{
        Interpolator{
            sample_rate,
            inverse_sample_rate: 1.0 / sample_rate,
        }
    }

    /// This preform the lagrange formula from a given point ot another. m is sometimes written as n
    /// (little n)
    fn lagrange_formula(t:f32,i:usize,len:usize) -> f32{
        let mut sum = 1.0;

        for j in 0..=len{
            if j != i{

                dbg!((i as f32 - j as f32));

                sum *= (t - j as f32) / (i as f32 - j as f32);
            }
        }

        dbg!(&sum);

        sum
    }

    fn lagrange_interpolator(start_samples: &[Complex<f32>], index:f32) -> Complex<f32>{
        let mut to_return = Complex::new(0.0,0.0);

        for n in 0..start_samples.len(){
            to_return += start_samples[n] * Interpolator::lagrange_formula(index,n,start_samples.len())
        }

        to_return
    }


    pub fn lagrange(&self, upsample_rate:usize, samples: &[Complex<f32>]) -> Vec<Complex<f32>>{
        // create up sampled vector
        let mut to_return = Vec::with_capacity(samples.len() * upsample_rate);
        let step = 1.0 / upsample_rate as f32;
        let mut t = 0.0;

        // preform interpolation
        for x in samples{
            to_return.push(*x);

            t += step;

            for _ in 1..upsample_rate{
                to_return.push(Interpolator::lagrange_interpolator(samples,t));

                t += step;
            }
        }

        to_return
    }
}