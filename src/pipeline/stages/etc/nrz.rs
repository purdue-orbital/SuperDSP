use crate::prelude::Data;

pub fn nrz(data: &[u8]) -> Vec<f32>{
    let mut output = Vec::new();
    for byte in data {
        for i in 0..8 {
            let bit = (byte >> i) & 1u8;
            output.push(if bit == 0u8 { -1.0 } else { 1.0 });
        }
    }
    output
}

pub fn nrz_i8(data: &[u8]) -> Vec<i8>{
    let mut output = Vec::new();
    for byte in data {
        for i in 0..8 {
            let bit = (byte >> i) & 1u8;
            output.push(if bit == 0u8 { -1 } else { 1 });
        }
    }
    output
}

pub fn nrz_inv(data: &[f32]) -> Vec<u8>{
    let mut output = Vec::new();
    let mut byte = 0u8;
    let mut i = 0;
    for bit in data {
        if *bit == 1.0 {
            byte |= 1 << i;
        }
        i += 1;
        if i == 8 {
            output.push(byte);
            byte = 0;
            i = 0;
        }
    }
    output
}

pub fn nrz_inv_i8(data: &[i8]) -> Vec<u8>{
    let mut output = Vec::new();
    let mut byte = 0u8;
    let mut i = 0;
    for bit in data {
        if *bit == 1 {
            byte |= 1 << i;
        }
        i += 1;
        if i == 8 {
            output.push(byte);
            byte = 0;
            i = 0;
        }
    }
    output
}