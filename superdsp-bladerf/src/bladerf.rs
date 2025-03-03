use std::ffi::{c_uint, c_void};
use std::mem;
use bladerf::{bladerf_channel, bladerf_channel_layout_BLADERF_RX_X1, bladerf_channel_layout_BLADERF_TX_X1, bladerf_devinfo, bladerf_enable_module, bladerf_format_BLADERF_FORMAT_SC16_Q11, bladerf_init_devinfo, bladerf_open_with_devinfo, bladerf_set_bandwidth, bladerf_set_frequency, bladerf_set_gain, bladerf_set_sample_rate, bladerf_sync_config, bladerf_sync_rx};
use num::Complex;

#[derive(PartialEq, Copy, Clone, Eq)]
pub enum Channel{
    RX = 0,
    TX = 1,
}

pub struct BladeRf{
    pub dev: *mut bladerf::bladerf,
}

impl Default for BladeRf {
    fn default() -> Self {
        Self::new()
    }
}
impl BladeRf{
    pub fn new() -> BladeRf{
        unsafe {
            let devinfo: *mut bladerf_devinfo = std::mem::MaybeUninit::new(mem::zeroed()).assume_init_mut();
            bladerf_init_devinfo(devinfo);

            // Initialize the BladeRF device
            println!("Opening BladeRF");
            let mut dev: *mut bladerf::bladerf = std::ptr::null_mut();
            bladerf_open_with_devinfo(&mut dev, devinfo);

            BladeRf{
                dev,
            }
        }
    }

    pub fn set_frequency(&self, frequency: u64, channel: Channel){
        unsafe {
            bladerf_set_frequency(self.dev, channel as bladerf_channel, frequency);
        }
    }

    pub fn set_sample_rate(&self, sample_rate: u32, channel: Channel){
        unsafe {
            bladerf_set_sample_rate(self.dev, channel as bladerf_channel, sample_rate, std::ptr::null_mut());
        }
    }

    pub fn set_gain(&self, gain: i32, channel: Channel){
        unsafe {
            bladerf_set_gain(self.dev, channel as bladerf_channel, gain);
        }
    }

    pub fn set_bandwidth(&self, bandwidth: u32, channel: Channel){
        unsafe {
            bladerf_set_bandwidth(self.dev, channel as bladerf_channel, bandwidth, std::ptr::null_mut());
        }
    }

    pub fn sync_config(&self, channel: Channel){
        let min_buf_size = 2 * 64 * 16;
        
        if channel == Channel::RX {
            unsafe {
                bladerf_sync_config(self.dev, bladerf_channel_layout_BLADERF_RX_X1, bladerf_format_BLADERF_FORMAT_SC16_Q11, 1024, min_buf_size, 8, 1000);
            }
        }else {
            unsafe {
                bladerf_sync_config(self.dev, bladerf_channel_layout_BLADERF_TX_X1, bladerf_format_BLADERF_FORMAT_SC16_Q11, 1024, min_buf_size, 8, 1000);
            }
        }
    }

    pub fn enable_module(&self, channel: Channel){
        unsafe {
            bladerf_enable_module(self.dev, channel as bladerf_channel, true);
        }
    }
    
    pub fn sync_rx(&self, buffer: &mut [Complex<i16>]){
        unsafe {
            bladerf_sync_rx(self.dev, buffer.as_mut_ptr() as *mut c_void, buffer.len() as c_uint, std::ptr::null_mut(), 1000);
        }
    }
    
    pub fn sync_tx(&self, buffer: &[Complex<i16>]){
        unsafe {
            bladerf_sync_rx(self.dev, buffer.as_ptr() as *mut c_void, buffer.len() as c_uint, std::ptr::null_mut(), 1000);
        }
    }

}