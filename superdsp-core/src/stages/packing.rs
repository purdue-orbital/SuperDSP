use alloc::vec;
use alloc::vec::Vec;
use crate::ResMut;

pub fn pack<const N: u8>(mut bytes: ResMut<Vec<u8>>){
    *bytes = __pack__(bytes.as_slice(), N);
}

pub fn unpack<const N: u8>(mut bytes: ResMut<Vec<u8>>){
    let mut m = Vec::new();
    for x in bytes.iter() {
        m.extend_from_slice(&__unpack__(*x, N));   
    }
    
    *bytes = m;
}

/// Takes a byte and unpacks it. Unpack_len denotes how many bits per an element. 
/// This unpacks into big endian
pub fn __unpack__(byte: u8, unpack_len: u8) -> Vec<u8>{
    let mut out = Vec::new();
    let mask: u8 = 2u8.pow(unpack_len as u32) - 1;
    
    for x in (0..8).step_by(unpack_len as usize) {
        out.push((byte >> x) & mask);
    }
    
    out
}

/// Takes a segments and packs with some number of other samples (pack_len). 
/// This assumes big endian
pub fn __pack__(samples: &[u8], pack_len: u8) -> Vec<u8>{
    let mut out = Vec::new();
    let mut byte: u8 = 0;
    let mut count: u8 = 0;
    
    for sample in samples {
        byte |= *sample << count;
        
        count += pack_len;
        
        if count >= 9 - pack_len {
            out.push(byte);
            byte = 0;
            
            count = 0;
        }
    }
    
    // add the rest
    if count != 0 {
        out.push(byte);
    }
    
    out
}

#[test]
pub fn test_pack_f32() {
    let arr = __pack__(&[1, 2, 3, 1], 2);
    assert_eq!(arr, vec![0b01111001]);
    
    let unpacked = __unpack__(0b11001010, 2);
    assert_eq!(unpacked.as_slice(), &[0b10,0b10,0b00,0b11]);
    
    let full = __pack__(unpacked.as_slice(), 2);
    assert_eq!(full.as_slice(), &[0b11001010]);
}