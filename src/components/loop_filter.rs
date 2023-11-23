//! The loop filter is a critical tool to implement advance advance systems and digital circuits
//! a good resource for this implementation is here https://ieeexplore.ieee.org/document/6154200

static DAMPING_COEFFICIENT:f32 = 0.707;

pub struct LoopFilter {
    naf:f32,
    time:f32,
    sigma:f32,
    time_step:f32,
    loop_gain:f32,
}

impl LoopFilter {


    /// This calculates C1 which is a coefficient in a loop filter
    fn c1(&self,time:f32) -> f32{
        (1.0 / self.loop_gain) * ((8.0 * DAMPING_COEFFICIENT * self.naf * time) / (4.0 + (4.0 * DAMPING_COEFFICIENT * time) + (self.naf * time).powi(2)))
    }

    /// This calculates C2 which is a coefficient in a loop filter
    fn c2(&self,time:f32) -> f32{
        (1.0 / self.loop_gain) * ((4.0 * (DAMPING_COEFFICIENT * self.naf * time).powi(2)) / (4.0 + (4.0 * DAMPING_COEFFICIENT * time) + (self.naf * time).powi(2)))
    }

    fn natural_angular_frequency(noise_bandwidth:f32) -> f32{
        (8.0 * DAMPING_COEFFICIENT * noise_bandwidth) / ((4.0 * DAMPING_COEFFICIENT.powi(2)) + 1.0)
    }

    pub fn run(&mut self, sample:f32) -> f32{
        // get coefficients
        let c1 = self.c1(self.time);
        let c2 = self.c2(self.time);

        // add sample to sigma
        self.sigma += sample;

        // add time step
        self.time += self.time_step;

        // return
        c1*sample + c2 * self.sigma
    }


    /// This will create a new loop filter instance. Loop gain is going to be 1 in most cases
    pub fn new(noise_bandwidth:f32, sample_rate:f32, loop_gain:f32) -> LoopFilter{
        LoopFilter{
            naf:LoopFilter::natural_angular_frequency(noise_bandwidth),
            time: 0.0,
            sigma: 0.0,
            time_step: 1.0 / sample_rate,
            loop_gain
        }
    }
}