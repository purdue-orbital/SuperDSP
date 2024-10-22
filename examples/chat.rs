use std::thread::{sleep, spawn};
use superdsp::BladeRF;
use superdsp::etc::SlidingWindow;
use superdsp::fsk::{fsk_demod, fsk_mod};

fn main() {
    let mut radio = BladeRF::new();
    radio.open().unwrap();

    unsafe { radio.configure_rx(); }
    unsafe { radio.configure_tx(); }
    
    let mut sliding_window = SlidingWindow::new(vec![0xFF, 0xAA, 0x55, 0xAA], 4);

    spawn(move || unsafe {
        // send a message periodically
        loop {
            sleep(std::time::Duration::from_secs(10));
            let data = vec![0xFF, 0xAA, 0x55, 0xAA, 0xFF, 0xAA, 0x55, 0xAA, 0xf4, 0x0f, 0x0f, 0xf4, 0x0f, 0x0f, 0xf4, 0x0f];
            let modulated = fsk_mod(radio.frequency, radio.sample_rate, radio.bandwidth as u64, &data);
            radio.transmit(modulated);
        }
    });
    
    unsafe {
        loop {
            // read from radio
            let data = radio.receive();
            
            // demodulate
            let data = fsk_demod(radio.sample_rate, radio.bandwidth as u64, &data);
            
            // add to sliding window
            for x in data {
                if let Some(bucket) = sliding_window.push(x) {
                    println!("{:?}", bucket);
                }
            }
        }
    }
}