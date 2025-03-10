use alloc::vec;
use alloc::vec::Vec;

/// Takes a byte and unpacks it. Unpack_len denotes how many bits per an element. 
/// This unpacks into big endian
pub fn __unpack_f32__(byte: u8, unpack_len: u8) -> Vec<u8>{
    let mut out = Vec::new();
    let mask: u8 = 2u8.pow(unpack_len as u32) - 1;
    
    for x in (0..8).step_by(unpack_len as usize) {
        out.push((byte >> x) & mask);
    }
    
    out
}

/// Takes a segments and packs with some number of other samples (pack_len). 
/// This assumes big endian
pub fn __pack_f32__(samples: &[u8], pack_len: u8) -> Vec<u8>{
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
    let arr = __pack_f32__(&[1, 2, 3, 1], 2);
    assert_eq!(arr, vec![0b01111001]);
    
    let unpacked = __unpack_f32__(0b11001010, 2);
    assert_eq!(unpacked.as_slice(), &[0b10,0b10,0b00,0b11]);
    
    let full = __pack_f32__(unpacked.as_slice(), 2);
    assert_eq!(full.as_slice(), &[0b11001010]);
}