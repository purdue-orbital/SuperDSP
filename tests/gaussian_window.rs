use superdsp::filters_and_windows::gaussian_window;

const N_SHORT: usize = 16;
const N_LONG: usize = 64;
const SIGMA: f32 = 0.4;

#[test]
pub fn gaussian_window_short(){
    // Expected values for a gaussian window with N = 16 and sigma = 0.4
    let expected: [f32; 16] = [
        0.04394,
        0.09136,
        0.17242,
        0.29502,
        0.45783,
        0.64439,
        0.82258,
        0.95234,
        1.0,
        0.95234,
        0.82258,
        0.64439,
        0.45783,
        0.29502,
        0.17242,
        0.09136,
    ];
    
    let window = gaussian_window::gaussian_window::<N_SHORT>(0, SIGMA);
    assert_eq!(window.len(), N_SHORT);
    
    // make sure the window is correct to 4 decimal places (floating point error)
    for i in 0..N_SHORT {
        assert!((window[i] - expected[i]).abs() < 0.0001);
    }
}



