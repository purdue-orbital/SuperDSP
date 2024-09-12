// Test Triangle Window

use superdsp::filters_and_windows::{blackman};

const N_SHORT: usize = 16;
const N_LONG: usize = 64;
#[test]
pub fn blackman_window_short(){
    let expected = [
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 0.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 0.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 1.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 1.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 2.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 2.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 3.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 3.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 4.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 4.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 5.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 5.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 6.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 6.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 7.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 7.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 8.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 8.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 9.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 9.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 10.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 10.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 11.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 11.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 12.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 12.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 13.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 13.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 14.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 14.0) / 16.0),
        0.42 - 0.5 * f32::cos((2.0 * std::f32::consts::PI * 15.0) / 16.0) + 0.08 * f32::cos((4.0 * std::f32::consts::PI * 15.0) / 16.0)
    ];

    let window = blackman::blackman_window::<N_SHORT>(0);
    assert_eq!(window.len(), N_SHORT);

    // make sure the window is correct to 4 decimal places (floating point error)
    for i in 0..N_SHORT {
        assert!((window[i] - expected[i]).abs() < 0.0001);
    }
}
