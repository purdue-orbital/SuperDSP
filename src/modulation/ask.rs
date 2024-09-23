//WARNING! This hasn't been fully tested - I'm just putting it here so that we've got something to work with It should... mostly work.
//OTHER WARNING! My *dheubhokhersos* didn't realize we're using the alloc crate, so this uses stock standard arrays. Prepare for stack overflows until I fix it.
use libm::cosf;

pub struct ASK<const NUM_BYTES: usize, const FINAL_BYTES: usize>{
    frequency: f32,
    sample_rate: f32,
    samples_per_bit: usize,
}

impl<const NUM_BYTES: usize, const FINAL_BYTES: usize> ASK<NUM_BYTES, FINAL_BYTES>{
    pub fn new(freq: f32, sample_rate: f32, samples_per_bit: usize) -> Self{
        return ASK { frequency: (freq), sample_rate:(sample_rate), samples_per_bit: (samples_per_bit) }
    }
    pub fn run (self, data: [u8; NUM_BYTES], time: f32) -> [f32; FINAL_BYTES]{
        //println!("Does this work???");
        let pi = core::f32::consts::PI;
        let mut index: usize = 0;
        let timestep = 1.0/self.sample_rate;
        let mut time_offset = 0.0;
        let mut signal = [0.0f32; FINAL_BYTES];
        for mut current_byte in data{
            //println!("Working with byte {current_byte}");
            let bit_one = current_byte&0x01; current_byte >>= 1;
            let bit_two = current_byte&0x01; current_byte >>= 1;
            let bit_three = current_byte&0x01; current_byte >>= 1;
            let bit_four = current_byte&0x01; current_byte >>= 1;
            let bit_five = current_byte&0x01; current_byte >>= 1;
            let bit_six = current_byte&0x01; current_byte >>= 1;
            let bit_seven = current_byte&0x01; current_byte >>= 1;
            let bit_eight = current_byte;
            let bit_array = [bit_one, bit_two, bit_three, bit_four, bit_five, bit_six, bit_seven, bit_eight];
            for bit in bit_array{
                //println!("Bit here is {bit}");
                let mut bit_type = 1.0;
                if bit==0{bit_type = 0.1;}
                for _ in 0..self.samples_per_bit{
                    let current_radian = 2.0*pi*((time+time_offset)*self.frequency);
                    //println!("Current radian is {current_radian}");
                    signal[index] = cosf(current_radian)*bit_type;
                    //let dummy = cosf(current_radian)*bit_type;
                    //println!("Writing {dummy} to {index}");
                    index+=1;
                    time_offset+=timestep;
                }
            }
        }
        return signal;
    }
}