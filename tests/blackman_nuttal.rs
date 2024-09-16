// Test Blackman-Nuttal Window

use superdsp::filters_and_windows::blackman_nuttall::{self};

const N_SHORT: usize = 16;
const N_LONG: usize = 64;
#[test]
pub fn blackman_nuttall_window_short() {
    let expected = [
        0.0003628,
        0.00415908,
        0.025205567,
        0.089622437,
        0.2269824,
        0.444360497,
        0.701958233,
        0.916185585,
        1.0,
        0.916185585,
        0.701958233,
        0.444360497,
        0.2269824,
        0.089622437,
        0.025205567,
        0.00415908,
    ];

    let window = blackman_nuttall::blackman_nuttall::<N_SHORT>(0);
    assert_eq!(window.len(), N_SHORT);

    // make sure the window is correct to 4 decimal places (floating point error)
    for i in 0..N_SHORT {
        assert!((window[i] - expected[i]).abs() < 0.0001);
    }
}

#[test]
fn blackman_nuttall_window_long() {
    let expected = [
        0.0003628,
        0.000551804,
        0.001157539,
        0.00229591,
        0.00415908,
        0.007013467,
        0.011196015,
        0.017107999,
        0.025205567,
        0.035986351,
        0.049971711,
        0.067684467,
        0.089622437,
        0.116228516,
        0.1478585,
        0.18474829,
        0.2269824,
        0.274465952,
        0.326902335,
        0.383778618,
        0.444360497,
        0.507698101,
        0.572643358,
        0.637878968,
        0.701958233,
        0.763354282,
        0.820516516,
        0.871931566,
        0.916185585,
        0.952024557,
        0.978409225,
        0.994561553,
        1.0,
        0.994561553,
        0.978409225,
        0.952024557,
        0.916185585,
        0.871931566,
        0.820516516,
        0.763354282,
        0.701958233,
        0.637878968,
        0.572643358,
        0.507698101,
        0.444360497,
        0.383778618,
        0.326902335,
        0.274465952,
        0.2269824,
        0.18474829,
        0.1478585,
        0.116228516,
        0.089622437,
        0.067684467,
        0.049971711,
        0.035986351,
        0.025205567,
        0.017107999,
        0.011196015,
        0.007013467,
        0.00415908,
        0.00229591,
        0.001157539,
        0.000551804,
    ];

    let window = blackman_nuttall::blackman_nuttall::<N_LONG>(0);
    assert_eq!(window.len(), N_LONG);

    // make sure the window is correct to 4 decimal places (floating point error)
    for i in 0..N_LONG {
        println!("{}: {}", i, window[i]);
        assert!((window[i] - expected[i]).abs() < 0.0001);
    }
}
