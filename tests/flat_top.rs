// Test Flat Top Window

use superdsp::filters_and_windows::flat_top;

const N_SHORT: usize = 16;

#[test]
pub fn flat_top_window_short(){
    let expected = [
        -0.00042,
        -0.00527,
        -0.02687,
        -0.0627,
        -0.05474,
        0.10175,
        0.44414,
        0.82854,
        1.0,
        0.82854,
        0.44414,
        0.10175,
        -0.05474,
        -0.0627,
        -0.02687,
        -0.00527,
    ];
    
    let window = flat_top::flat_top_window::<N_SHORT>(0);
    assert_eq!(window.len(), N_SHORT);
    
    // make sure the window is correct to 4 decimal places (floating point error)
    for i in 0..N_SHORT {
        assert!((window[i] - expected[i]).abs() < 0.0001);
    }
}