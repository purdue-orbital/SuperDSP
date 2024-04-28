use rustdsp::math::fixed_point::{f32_to_i32_fixed, fixed_mul, i32_fixed_to_f32};
use rustdsp::math::sqrt;

#[test]
pub fn f32_to_fixed(){
    let test1 = f32_to_i32_fixed(0.5);
    let expected1 = 32768;
    
    let test2 = f32_to_i32_fixed(0.1);
    let expected2 = 6554;

    let test3 = f32_to_i32_fixed(110.11);
    let expected3 = 7216169;
    
    let test4 = f32_to_i32_fixed(-114.33);
    let expected4 = -7492731;
    
    assert_eq!(test1,expected1);
    assert_eq!(test2,expected2);
    assert_eq!(test3,expected3);
    assert_eq!(test4,expected4);
}

#[test]
pub fn fixed_to_f32(){
    let test1 = i32_fixed_to_f32(3210);
    let expected1 = 0.048980713;
    
    let test2 = i32_fixed_to_f32(8855334);
    let expected2 = 135.12167;

    let test3 = i32_fixed_to_f32(54325367);
    let expected3 = 828.93933;
    
    let test4 = i32_fixed_to_f32(-69);
    let expected4 = -0.0010528564;

    assert_eq!(test1,expected1);
    assert_eq!(test2,expected2);
    assert_eq!(test3,expected3);
    assert_eq!(test4,expected4);
}

#[test]
pub fn fixed_point_multiplication(){
    let test1 = fixed_mul(327680000,327680);
    let expected1 = 1638400000;
    
    let test2 = fixed_mul(-425984000,327680);
    let expected2 = -2129920000;
    
    let test3 = fixed_mul(3277,32768);
    let expected3 = 1638;

    assert_eq!(test1,expected1);
    assert_eq!(test2,expected2);
    assert_eq!(test3,expected3);
}

#[test]
pub fn round_trip_fixed_i32(){
    let test1 = 0.5;
    let test2 = 1.5;
    let test3 = -10.5;
    let test4 = 4.0;
    
    assert_eq!(i32_fixed_to_f32(f32_to_i32_fixed(test1)),test1);
    assert_eq!(i32_fixed_to_f32(f32_to_i32_fixed(test2)),test2);
    assert_eq!(i32_fixed_to_f32(f32_to_i32_fixed(test3)),test3);
    assert_eq!(i32_fixed_to_f32(f32_to_i32_fixed(test4)),test4);
}

#[test]
pub fn sqrt_test(){
    let test1 = sqrt(1638400);
    let expected1 = 327680;

    let test2 = sqrt(6553600);
    let expected2 = 655360;

    let test3 = sqrt(1654784);
    let expected3 = 329314;
    
    

    assert_eq!(test1,expected1);
    assert_eq!(test2,expected2);
    assert_eq!(test3,expected3);
}
