// Test Triangle Window

use superdsp::filters_and_windows::hamming;

const N_SHORT: usize = 16;
const N_LONG: usize = 64;
#[test]
pub fn hamming_window_short(){
    let expected = [
        0.08696,
        0.12171,
        0.22067,
        0.36877,
        0.54348,
        0.71818,
        0.86629,
        0.96525,
        1.0,
        0.96525,
        0.86629,
        0.71818,
        0.54348,
        0.36877,
        0.22067,
        0.12171
    ];
    
    let window = hamming::hamming_window::<N_SHORT>(N_SHORT, 0);
    assert_eq!(window.len(), N_SHORT);
    
    // make sure the window is correct to 3 decimal places (floating point error)
    for i in 0..N_SHORT {
        assert!((window[i] - expected[i]).abs() < 0.001);
    }
}

#[test]
fn hamming_window_long(){
    let expected = [
        0.08696,
        0.08915,
        0.09573,
        0.10661,
        0.12171,
        0.14086,
        0.16389,
        0.19058,
        0.22067,
        0.25386,
        0.28985,
        0.32828,
        0.36877,
        0.41096,
        0.45442,
        0.49873,
        0.54348,
        0.58823,
        0.63254,
        0.676,
        0.718182,
        0.758681,
        0.797108,
        0.833093,
        0.86629,
        0.89637,
        0.92306,
        0.94609,
        0.96525,
        0.98034,
        0.99123,
        0.99784,
        1.0,
        0.99784,
        0.99123,
        0.98034,
        0.96525,
        0.94609,
        0.92306,
        0.89637,
        0.86629,
        0.833093,
        0.797108,
        0.758681,
        0.718182,
        0.676,
        0.63254,
        0.58823,
        0.54348,
        0.49873,
        0.45442,
        0.41096,
        0.36877,
        0.32828,
        0.28985,
        0.25386,
        0.22067,
        0.19058,
        0.16389,
        0.14086,
        0.12171,
        0.10661,
        0.09573,
        0.08915
    ];

    let window = hamming::hamming_window::<N_LONG>(N_LONG, 0);
    assert_eq!(window.len(), N_LONG);
    
    // make sure the window is correct to 4 decimal places (floating point error)
    for i in 0..N_LONG {
        assert!((window[i] - expected[i]).abs() < 0.0001);
    }
}

