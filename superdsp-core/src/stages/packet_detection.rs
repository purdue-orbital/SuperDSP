use alloc::vec::Vec;
use core::ops::Mul;
use nalgebra::{SMatrix, SVector};
use num::Complex;
use crate::{Res, ResMut, Scheduler};

pub const CCSDS_PREAMBLE: u16 = 0b1110101110010000;

pub fn PacketDetction<const N: usize>(s: &mut Scheduler){
    s.add_resource(PacketDetectionSettingsF32::<N>{
        matrix: SMatrix::zeros(),
        threshold: Default::default(),
        buffer: SMatrix::zeros(),
    });
    
    s.add_resource(0isize)
}

pub struct PacketDetectionSettingsF32<const N: usize> {
    pub matrix: SMatrix<Complex<f32>, N, 1>,

    pub threshold: Complex<f32>,
    
    pub buffer: SMatrix<Complex<f32>,1,N>,
}

pub fn cross_correlation_f32<const N: usize>(input: Res<Vec<Complex<f32>>>, mut settings: ResMut<PacketDetectionSettingsF32<N>>, mut signal: ResMut<isize>){
    for (index, x) in settings.buffer.data.0.iter_mut().enumerate(){
        x[0] = input[index];
    }
    
    let out =  settings.buffer.mul(settings.matrix);

    for (index, x) in out.iter().enumerate() {
        if x.re >= settings.threshold.re && x.im >= settings.threshold.im {
            *signal = index as isize;

            return;
        }
    }

    *signal = -1;
}