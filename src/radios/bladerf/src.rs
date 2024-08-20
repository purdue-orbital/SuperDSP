use crate::objects::object::{DSPObject, Type};
use bladerf::{bladerf_init_devinfo, bladerf_open_with_devinfo, bladerf_sync_rx};
use num::Complex;
use spin::Mutex;
use std::ffi::c_uint;
use std::os::raw::c_void;
use std::prelude::rust_2021::Vec;
use std::ptr::null_mut;
use std::sync::Arc;
use std::{mem, println, vec};
use crate::objects::object::Type::Complex as OtherComplex;

#[derive(Debug, Clone)]
pub struct BladeRfSrc {
    pub frequency: u64,
    pub sample_rate: u32,
    pub gain: i32,
    pub bandwidth: u32,
    pub num_samples: usize,

    pub sample_buffer: Vec<Complex<i16>>,
    pub counter: usize,

    pub output_buffer: Arc<Mutex<Complex<f64>>>,
    pub dev: Arc<Mutex<*mut bladerf::bladerf>>,

}

impl BladeRfSrc {

    /// Create a new BladeRF object with the given parameters and return it as a BladeRF object
    /// instance.
    /// - frequency: u64 - The frequency to set the BladeRF to (in Hz) (min: 237500000, max: 3800000000)
    /// - sample_rate: u32 - The sample rate to set the BladeRF to (in Hz) (min: 80000, max: 40000000)
    /// - gain: i32 - The gain to set the BladeRF to (in dB) (min: 5, max: 66)
    /// - bandwidth: u32 - The bandwidth to set the BladeRF to (in Hz) (min: 1500000, max: 28000000)
    /// - num_samples: usize - The number of samples to read from the BladeRF (must be a multiple of 1024)
    pub fn new(frequency: u64, sample_rate: u32, gain: i32, bandwidth: u32, num_samples: usize) -> BladeRfSrc {
        assert_eq!(num_samples % 1024, 0);

        let dev = unsafe {
            println!("Getting Dev-Info BladeRF");
            let devinfo: *mut bladerf::bladerf_devinfo = std::mem::MaybeUninit::new(mem::zeroed()).assume_init_mut();
            bladerf_init_devinfo(devinfo);

            // Initialize the BladeRF device
            println!("Opening BladeRF");
            let mut dev: *mut bladerf::bladerf = std::ptr::null_mut();
            bladerf_open_with_devinfo(&mut dev, devinfo);


            // Set the frequency
            println!("Setting Frequency BladeRF");
            bladerf::bladerf_set_frequency(dev, 0, frequency);

            // Set the sample rate
            println!("Setting Sample Rate BladeRF");
            bladerf::bladerf_set_sample_rate(dev, 0, sample_rate, std::ptr::null_mut());

            // Set the gain
            println!("Setting Gain BladeRF");
            bladerf::bladerf_set_gain(dev, 0, gain);

            // Set the bandwidth
            println!("Setting Bandwidth BladeRF");
            bladerf::bladerf_set_bandwidth(dev, 0, bandwidth, std::ptr::null_mut());

            // Configure the sync interface
            let min_buf_size = 2 * num_samples * 1 * 16;
            bladerf::bladerf_sync_config(dev, bladerf::bladerf_channel_layout_BLADERF_RX_X1, bladerf::bladerf_format_BLADERF_FORMAT_SC16_Q11, 16, min_buf_size as c_uint, 8, 3500);

            // Enable Stream
            bladerf::bladerf_enable_module(dev, 0, true);

            dev
        };

        BladeRfSrc {
            frequency,
            sample_rate,
            gain,
            bandwidth,
            num_samples,

            sample_buffer: vec![Complex::new(0, 0); num_samples],
            counter: 0,

            output_buffer: Arc::new(Default::default()),
            dev: Arc::new(Mutex::new(dev)),
        }
    }
}

unsafe impl Send for BladeRfSrc {}
unsafe impl Sync for BladeRfSrc {}


impl DSPObject for BladeRfSrc {
    fn return_type(&self) -> Type {
        Type::Complex
    }

    fn input_type(&self) -> Type {
        Type::NONE
    }

    fn set_input_buffer(&mut self, buffer: Arc<spin::mutex::Mutex<f64>>) {
        // BladeRF does not take any input
        panic!("BladeRF does not have an input buffer");
    }
    fn get_output_buffer(&self) -> Arc<Mutex<f64>> {
        panic!("BladeRF does not have an output buffer");
    }

    fn set_input_buffer_complex(&mut self, buffer: Arc<spin::mutex::Mutex<Complex<f64>>>) {
        panic!("BladeRF does not have a complex input buffer");
    }

    fn get_output_buffer_complex(&self) -> Arc<spin::mutex::Mutex<Complex<f64>>> {
        self.output_buffer.clone()
    }

    fn set_input_buffer_vec(&mut self, buffer: Arc<spin::mutex::Mutex<Vec<f64>>>) {
        panic!("BladeRF does not have a vector input buffer");
    }
    fn get_output_buffer_vec(&self) -> Arc<spin::mutex::Mutex<Vec<f64>>> {
        panic!("BladeRF does not have a vector output buffer");
    }

    fn set_input_buffer_complex_vec(&mut self, buffer: Arc<spin::mutex::Mutex<Vec<Complex<f64>>>>) {
        panic!("BladeRF does not have a complex vector input buffer");
    }

    fn get_output_buffer_complex_vec(&self) -> Arc<spin::mutex::Mutex<Vec<Complex<f64>>>> {
        panic!("BladeRF does not have a complex vector output buffer");
    }

    fn process(&mut self) {
        if self.counter == 0{
            unsafe { bladerf_sync_rx(*self.dev.lock(), self.sample_buffer.as_mut_ptr() as *mut c_void, self.num_samples as c_uint, null_mut(), 1000); }
        }

        *self.output_buffer.lock() = Complex::new(self.sample_buffer[self.counter].re as f64 / 2048.0, self.sample_buffer[self.counter].re as f64 / 2048.0);
        self.counter = (self.counter + 1) % self.num_samples;
    }
}