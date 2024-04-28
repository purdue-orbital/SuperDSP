use rustdsp::math::{expj, FRAC_PI_2, PI, sincos};
use rustdsp::math::fixed_point::{f32_to_i32_fixed, fixed_mul, i32_fixed_to_f32};
use core::f32;

#[test]
pub fn sincos_test() {
    let mut e_sum = 0.0;

    for x in -100..100 {
        let i_float = f32::consts::PI * (x as f32 / 100.0);
        let i = fixed_mul(PI, f32_to_i32_fixed(x as f32 / 100.0));

        let sincos = sincos(i);

        let sin = sincos.0;
        let cos = sincos.1;

        e_sum += (i32_fixed_to_f32(sin) - i_float.sin()).abs();
        e_sum += (i32_fixed_to_f32(cos) - i_float.cos()).abs();
    }
    dbg!(e_sum);
    assert!(e_sum < 1.0);
}

#[test]
pub fn exp_test() {
    let c = expj(PI);
    assert_eq!(c.re, -65536);
    assert_eq!(c.im, 0);

    let c = expj(2 * PI);
    assert_eq!(c.re, 65536);
    assert_eq!(c.im, 0);

    let c = expj(FRAC_PI_2);
    assert_eq!(c.re, 0);
    assert_eq!(c.im, -65536);

    let c = expj(3 * FRAC_PI_2);
    assert_eq!(c.re, 0);
    assert_eq!(c.im, -65536);

    let c = expj(-FRAC_PI_2);
    assert_eq!(c.re, 0);
    assert_eq!(c.im, -65536);

    let c = expj(-PI);
    assert_eq!(c.re, -65536);
    assert_eq!(c.im, 0);
}