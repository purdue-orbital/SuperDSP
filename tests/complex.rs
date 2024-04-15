use std::f32::consts::PI;
use rustdsp::math::complex::Complex;

#[test]
pub fn sincos(){
    
    let c = Complex::new(1.0,1.0);
    
    let mut e_sum = 0.0;

    for x in -100..100 {
        let i = PI * (x as f32/100.0);
        
        let sincos = Complex::sincos(i);
        
        let sin = sincos.0;
        let cos = sincos.1;
        
        e_sum += (sin - i.sin()).abs();
        e_sum += (cos - i.cos()).abs();

        //dbg!((sin - i.sin()).abs());
        
        if (cos - i.cos()).abs() > 1.0{
            dbg!(i);
            dbg!((cos - i.cos()));
        }
        
        //dbg!((cos - i.cos()).abs());
    }
    
    dbg!(e_sum);
}