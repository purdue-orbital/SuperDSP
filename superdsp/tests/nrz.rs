use rustdsp::prelude::etc::nrz::{nrz, nrz_inv, nrz_inv_i8};

#[test]
pub fn test_nrz(){
    let data = vec![0b10101010, 0b01010101];
    
    let out = nrz(data.as_slice());
    
    assert_eq!(&out, &[-1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0])
}

#[test]
pub fn test_nrz_inv() {
    let data = vec![-1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0];
    
    let out = nrz_inv(data.as_slice());
    
    assert_eq!(&out, &[0b10101010, 0b01010101])
}

#[test]
pub fn test_full_cycle(){
    let data = vec![0b10101010, 0b01010101];
    
    let out = nrz(data.as_slice());
    
    let out = nrz_inv(out.as_slice());
    
    assert_eq!(&out, &[0b10101010, 0b01010101])
}

#[test]
pub fn test_nrz_i8(){
    let data = vec![0b10101010, 0b01010101];
    
    let out = nrz(data.as_slice());
    
    let out = out.iter().map(|&x| x as i8).collect::<Vec<_>>();
    
    assert_eq!(&out, &[-1, 1, -1, 1, -1, 1, -1, 1, 1, -1, 1, -1, 1, -1, 1, -1])
}

#[test]
pub fn test_nrz_inv_i8() {
    let data: Vec<i8> = vec![-1, 1, -1, 1, -1, 1, -1, 1, 1, -1, 1, -1, 1, -1, 1, -1];
    
    let out = nrz_inv_i8(data.as_slice());
    
    assert_eq!(&out, &[0b10101010, 0b01010101])
}

#[test]
pub fn test_full_cycle_i8(){
    let data = vec![0b10101010, 0b01010101];
    
    let out = nrz(data.as_slice());
    
    let out = out.iter().map(|&x| x as i8).collect::<Vec<_>>();
    
    let out = nrz_inv_i8(out.as_slice());
    
    assert_eq!(&out, &[0b10101010, 0b01010101])
}
