pub mod fsk;
pub mod etc;

use std::ffi::{c_uint, c_void};
use std::sync::Mutex;
use bladerf::{bladerf_channel, bladerf_init_devinfo, bladerf_open_with_devinfo};
use num_complex::Complex;

pub struct BladeRF {
    pub frequency: u64,
    pub sample_rate: u32,
    pub gain: i32,
    pub bandwidth: u32,
    pub num_samples: usize,
    pub dev: Mutex<*mut bladerf::bladerf>,
}

impl BladeRF{
    pub fn new() -> BladeRF {
        BladeRF {
            frequency: 915_000_000,
            sample_rate: 1_000_000,
            gain: 30,
            bandwidth: 1_500_000,
            num_samples: 1024,
            dev: Mutex::new(std::ptr::null_mut()),
        }
    }

    pub fn new_with_params(frequency: u64, sample_rate: u32, gain: i32, bandwidth: u32, num_samples: usize) -> BladeRF {
        BladeRF {
            frequency,
            sample_rate,
            gain,
            bandwidth,
            num_samples,
            dev: Mutex::new(std::ptr::null_mut()),
        }
    }

    pub fn open(&mut self) -> Result<(), String> {
        let dev_info = std::ptr::null_mut();
        let mut dev = std::ptr::null_mut();

        unsafe {
            bladerf_init_devinfo(dev_info);
            bladerf_open_with_devinfo(&mut dev, dev_info);
        };

        self.dev = Mutex::new(dev);
        Ok(())
    }

    pub unsafe fn configure_rx(&mut self){
        let channel = ((0) << 1 | 0x0) as bladerf_channel;
        let dev = self.dev.lock().unwrap();
        
        let radio =self.dev.lock().unwrap().offset(0);

        // Set the frequency
        println!("Setting Frequency BladeRF");
        bladerf::bladerf_set_frequency(radio, channel, self.frequency);

        // Set the sample rate
        println!("Setting Sample Rate BladeRF");
        bladerf::bladerf_set_sample_rate(radio, channel, self.sample_rate, std::ptr::null_mut());

        // Set the gain
        println!("Setting Gain BladeRF");
        bladerf::bladerf_set_gain(radio, channel, self.gain);

        // Set the bandwidth
        println!("Setting Bandwidth BladeRF");
        bladerf::bladerf_set_bandwidth(radio, channel, self.bandwidth, std::ptr::null_mut());

        // Configure the sync interface
        let min_buf_size = 2 * self.num_samples * 1 * 16;
        bladerf::bladerf_sync_config(radio, bladerf::bladerf_channel_layout_BLADERF_RX_X1, bladerf::bladerf_format_BLADERF_FORMAT_SC16_Q11, 16, min_buf_size as c_uint, 8, 3500);

        // Enable Stream
        bladerf::bladerf_enable_module(radio, channel, true);
    }

    pub unsafe fn configure_tx(&mut self){
        let channel = ((0) << 1 | 0x1) as bladerf_channel;
        let dev = self.dev.lock().unwrap();

        let radio =self.dev.lock().unwrap().offset(0);

        // Set the frequency
        println!("Setting Frequency BladeRF");
        bladerf::bladerf_set_frequency(radio, channel, self.frequency);

        // Set the sample rate
        println!("Setting Sample Rate BladeRF");
        bladerf::bladerf_set_sample_rate(radio, channel, self.sample_rate, std::ptr::null_mut());

        // Set the gain
        println!("Setting Gain BladeRF");
        bladerf::bladerf_set_gain(radio, channel, self.gain);

        // Set the bandwidth
        println!("Setting Bandwidth BladeRF");
        bladerf::bladerf_set_bandwidth(radio, channel, self.bandwidth, std::ptr::null_mut());

        // Configure the sync interface
        let min_buf_size = 2 * self.num_samples * 1 * 16;
        bladerf::bladerf_sync_config(radio, bladerf::bladerf_channel_layout_BLADERF_RX_X1, bladerf::bladerf_format_BLADERF_FORMAT_SC16_Q11, 16, min_buf_size as c_uint, 8, 3500);

        // Enable Stream
        bladerf::bladerf_enable_module(radio, channel, true);
    }
    
    pub unsafe fn receive(&mut self) -> Vec<Complex<f32>> {
        let mut samples = vec![0; self.num_samples];
        let radio = self.dev.lock().unwrap().offset(0);
        let samples_ptr = samples.as_mut_ptr();

        bladerf::bladerf_sync_rx(radio, samples_ptr as *mut c_void, self.num_samples as u32, std::ptr::null_mut(), 3500);
        
        samples.iter().step_by(2).enumerate().map(|(index,&x)| Complex::new(x as f32 / 2048.0 ,samples[index+1] as f32 / 2048.0)).collect()
    }
    
    pub unsafe fn transmit(&mut self, samples: Vec<Complex<f32>>) {
        let radio = self.dev.lock().unwrap().offset(0);
        
        let mut c_samples: Vec<i16> = Vec::with_capacity(samples.len() * 2);
        
        // complex f32 to i12
        for sample in samples {
            c_samples.push((sample.re * 2048.0) as i16);
            c_samples.push((sample.im * 2048.0) as i16);
        }
        
        let samples_ptr = c_samples.as_ptr();

        bladerf::bladerf_sync_tx(radio, samples_ptr as *mut c_void, self.num_samples as u32, std::ptr::null_mut(), 3500);
    }
}