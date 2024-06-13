use std::f32::consts::PI;
use rustdsp::math::sincos::sincos;

#[test]
pub fn sincos_test(){
    let mut test = 0.0;
    let step = PI/1000.0;
    
    for x in -1000..1000{
        let theta = step * x as f32;
        let out = sincos(theta);
        
        if (out.0 - theta.sin()).abs() > 0.02{
            dbg!(theta);
            dbg!(out.0 - theta.sin());
        }
        
        assert!((out.0 - theta.sin()).abs() < 0.02);
        assert!((out.1 - theta.cos()).abs() < 0.02);
    }
    
}