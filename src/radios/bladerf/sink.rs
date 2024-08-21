use crate::objects::object::{Bus, DSPObject, Type};
use bladerf::{bladerf_channel, bladerf_direction_BLADERF_TX, bladerf_init_devinfo, bladerf_open_with_devinfo};
use num::Complex;
use spin::Mutex;
use std::ffi::c_uint;
use std::os::raw::c_void;
use std::prelude::rust_2021::Vec;
use std::ptr::null_mut;
use std::sync::Arc;
use std::{mem, println, vec};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct BladeRfSink {
    pub frequency: u64,
    pub sample_rate: u32,
    pub gain: i32,
    pub bandwidth: u32,
    pub num_samples: usize,

    pub buffer: Arc<Mutex<VecDeque<Complex<f64>>>>,
    pub counter: usize,

    pub bus: Bus<'static>,

    pub dev: Arc<Mutex<*mut bladerf::bladerf>>,
}

impl BladeRfSink {

    /// Create a new BladeRF Sink object with the given parameters and return it as a BladeRF object
    /// instance.
    /// - frequency: u64 - The frequency to set the BladeRF to (in Hz) (min: 237500000, max: 3800000000)
    /// - sample_rate: u32 - The sample rate to set the BladeRF to (in Hz) (min: 80000, max: 40000000)
    /// - gain: i32 - The gain to set the BladeRF to (in dB) (min: 5, max: 66)
    /// - bandwidth: u32 - The bandwidth to set the BladeRF to (in Hz) (min: 1500000, max: 28000000)
    /// - num_samples: usize - The number of samples to read from the BladeRF (must be a multiple of 1024)
    pub fn new(frequency: u64, sample_rate: u32, gain: i32, bandwidth: u32, num_samples: usize) -> BladeRfSink {
        assert_eq!(num_samples % 1024, 0);
        
        let channel = ((0) << 1 | 0x1) as bladerf_channel;

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
            bladerf::bladerf_set_frequency(dev, channel, frequency);

            // Set the sample rate
            println!("Setting Sample Rate BladeRF");
            bladerf::bladerf_set_sample_rate(dev, channel, sample_rate, std::ptr::null_mut());

            // Set the gain
            println!("Setting Gain BladeRF");
            bladerf::bladerf_set_gain(dev, channel, gain);

            // Set the bandwidth
            println!("Setting Bandwidth BladeRF");
            bladerf::bladerf_set_bandwidth(dev, channel, bandwidth, std::ptr::null_mut());

            // Configure the sync interface
            let min_buf_size = 2 * num_samples * 1 * 16;
            bladerf::bladerf_sync_config(dev, bladerf::bladerf_channel_layout_BLADERF_TX_X1, bladerf::bladerf_format_BLADERF_FORMAT_SC16_Q11, 16, min_buf_size as c_uint, 8, 3500);

            // Enable Stream
            bladerf::bladerf_enable_module(dev, channel, true);

            dev
        };

        let num_samples =  2 * num_samples * 1 * 16;

        BladeRfSink {
            frequency,
            sample_rate,
            gain,
            bandwidth,
            num_samples,
            
            buffer: Arc::new(Mutex::new(VecDeque::from(vec![Complex::new(0.0, 0.0); num_samples]))),
            counter: 0,

            bus: Bus::new_complex(),
            
            dev: Arc::new(Mutex::new(dev)),
        }
    }
}

unsafe impl Send for BladeRfSink {}
unsafe impl Sync for BladeRfSink {}


impl DSPObject for BladeRfSink {
    fn return_type(&self) -> Type {
        Type::NONE
    }

    fn input_type(&self) -> Type {
        Type::Complex
    }

    fn get_bus(&mut self) -> &mut Bus<'static> {
        panic!("BladeRfSink does not listen on a bus");
    }

    fn set_bus(&mut self, bus: &mut Bus<'static>) {
        self.bus = *bus;
        bus.subscribe(self);
    }

    fn process(&mut self) {
        let mut i16_buffer = vec![(self.bus.buffer_complex.unwrap().read().re * 2048.0) as i16, (self.bus.buffer_complex.unwrap().read().im * 2048.0) as i16];
        unsafe { bladerf::bladerf_sync_tx(*self.dev.lock(), i16_buffer.as_mut_ptr() as *mut c_void, self.num_samples as c_uint, null_mut(), 10000); }
    }

    fn start(&mut self) {}
}