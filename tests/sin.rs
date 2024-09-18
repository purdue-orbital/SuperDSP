// Test Sin Window
// Test values as derived from desmos correct to 4 decimal places

use superdsp::filters_and_windows::sin;

const N_SHORT: usize = 16;
const N_LONG: usize = 32;
#[test]
pub fn sin_window_short(){
    let expected = [
        0.0, //0
        0.19509, //1
        0.38268, //2
        0.55557, //3
        0.70711, //4
        0.83147, //5
        0.92388, //6
        0.98079, //7
        1.0, //8
        0.98079, //9
        0.92388, //10
        0.83147, //11
        0.70711, //12
        0.55557, //13
        0.38268, //14
        0.19509 //15
    ];
    
    let window = sin::sin_window::<N_SHORT>(0);
    assert_eq!(window.len(), N_SHORT);
    
    // make sure the window is correct to 4 decimal places (floating point error)
    for i in 0..N_SHORT {
        assert!((window[i] - expected[i]).abs() < 0.0001);
    }
}

#[test]
fn sin_window_long(){
    let expected = [
        0.0, //0
        0.09802, //1
        0.19509, //2
        0.29028, //3
        0.38268, //4 
        0.4714, //5
        0.55557, //6
        0.63439, //7
        0.70711, //8
        0.77301, //9
        0.83147, //10
        0.88192, //11
        0.92388, //12
        0.95694, //13
        0.98079, //14
        0.99518, //15
        1.0, //16
        0.99518, //17
        0.98079, //18
        0.95694, //19
        0.92388, //20
        0.88192, //21
        0.83147, //22
        0.77301, //23
        0.70711, //24
        0.63439, //25
        0.55557, //26
        0.4714, //27
        0.38268, //28
        0.29028, //29
        0.19509, //30
        0.09802 //31
    ];
    
    let window = sin::sin_window::<N_LONG>(0);
    assert_eq!(window.len(), N_LONG);
    
    // make sure the window is correct to 4 decimal places (floating point error)
    for i in 0..N_LONG {
        assert!((window[i] - expected[i]).abs() < 0.0001);
    }
}
