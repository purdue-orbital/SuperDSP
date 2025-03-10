use alloc::vec::Vec;
use core::f32::consts::PI;
use libm::{cos, cosf, sinf};
use nalgebra::{SMatrix, SVector};
use num::Complex;
use crate::{Res, ResMut};
use crate::stages::packet_detection::PacketDetectionSettingsF32;
use crate::stages::packing::__unpack_f32__;

/// .0 is equal to n and .1 is n-1
pub fn intermediate_sequence_f32(x: &[Complex<f32>], n: isize, radians_a_sample: f32) -> (Complex<f32>, Complex<f32>){
    // Base case (-1 and -2)
    if n <= -1 {
        return (Complex::new(0.0, 0.0), Complex::new(0.0, 0.0));
    }

    let s_n_1_2 = intermediate_sequence_f32(x,n-1, radians_a_sample);
    let s_n_2_3 = intermediate_sequence_f32(x,n-2, radians_a_sample);

    let s = x[n as usize] + 2.0 * cosf(radians_a_sample) * s_n_1_2.0 - s_n_1_2.1;
    let s_1 = x[n as usize] + 2.0 * cosf(radians_a_sample) * s_n_2_3.0 - s_n_2_3.1;


    (s,s_1)
}

pub fn goertzel_algorithm_f32(x: &[Complex<f32>], n: usize, radians_a_sample: f32) -> Complex<f32> {
    let s = intermediate_sequence_f32(x, n as isize, radians_a_sample);

    s.0 - Complex::new(cosf(radians_a_sample), sinf(radians_a_sample)) * s.1
}

#[derive(Default)]
pub struct FSKSettings{
    pub channel_0: usize,
    pub channel_1: usize,

    pub radians_a_sample_c0: f32,
    pub radians_a_sample_c1: f32,
    
    pub c_radian_c0: f32,
    pub c_radian_c1: f32,
}

impl FSKSettings {
    pub fn create_packet_detection_settings<const N: usize>(&self, preamble: &[u8], sps: usize) -> PacketDetectionSettingsF32<N>{
        let mut temp = Vec::with_capacity(preamble.len() * 8);
        let mut modded = Vec::with_capacity(preamble.len() * 8);
        
        let mut matrix = SMatrix::zeros();
        
        for x in preamble.iter(){
            for _ in 0..sps{
                temp.extend_from_slice(__unpack_f32__(*x,1).as_slice());
            }
        }
        
        let mut settings_copy = FSKSettings{
            channel_0: self.channel_0,
            channel_1: self.channel_1,
            radians_a_sample_c0: self.radians_a_sample_c0,
            radians_a_sample_c1: self.radians_a_sample_c1,
            c_radian_c0: self.c_radian_c0,
            c_radian_c1: self.c_radian_c1,
        };
        
        for x in temp.iter(){
            __fsk_mod__(&mut settings_copy, *x, &mut modded);
            
            for (i,y) in modded.iter().enumerate(){
                matrix.data.0[0][i] = y.conj();
            } 
        }
        
        
        PacketDetectionSettingsF32{
            matrix,
            threshold: Default::default(),
            buffer: SMatrix::zeros(),
        }
    }
    
}

/// Does 1 bit at a time
pub fn fsk_demod_f32(fsk_settings: Res<FSKSettings>, input: Res<Vec<Complex<f32>>>, mut output: ResMut<u8>){
    let c0 = goertzel_algorithm_f32(input.as_slice(), fsk_settings.channel_0, fsk_settings.radians_a_sample_c0);
    let c1 = goertzel_algorithm_f32(input.as_slice(), fsk_settings.channel_1, fsk_settings.radians_a_sample_c1);

    *output = if c0.re > c1.re {0} else {1};
}

/// Input is assumed to be 1 bit
pub fn fsk_mod_f32(mut fsk_settings: ResMut<FSKSettings>, input: Res<u8>, mut output: ResMut<Vec<Complex<f32>>>) {
    __fsk_mod__(&mut *fsk_settings, *input, &mut *output);
}

fn __fsk_mod__(fsk_settings: &mut FSKSettings, input: u8, output: &mut [Complex<f32>]){
    if input == 1 {
        for x in output.iter_mut(){
            *x = Complex::new(cosf(fsk_settings.c_radian_c1), sinf(fsk_settings.c_radian_c1));

            fsk_settings.c_radian_c1 += fsk_settings.radians_a_sample_c1;
            fsk_settings.c_radian_c1 %= 2.0 * PI;
        }
    } else {
        for x in output.iter_mut(){
            *x = Complex::new(cosf(fsk_settings.c_radian_c0), sinf(fsk_settings.c_radian_c0));

            fsk_settings.c_radian_c0 += fsk_settings.radians_a_sample_c0;
            fsk_settings.c_radian_c0 %= 2.0 * PI;
        }
    };
}
