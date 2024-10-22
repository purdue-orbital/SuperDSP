use superdsp::etc::SlidingWindow;

#[test]
fn test_sliding() {
    let pattern = vec![0x00, 0xFF, 0xAA, 0x55];
    let bucket_len = 4;
    
    let mut sw = SlidingWindow::new(pattern, bucket_len);
    
    let data = vec![0x00, 0xFF, 0xAA, 0x55, 0xFF, 0xAA, 0x55, 0xAA, 0xFF, 0x00, 0xFF, 0xAA, 0x55, 0x0f, 0x0f, 0xf4, 0x0f, 0x0f, 0xf4, 0x0f];
    
    let mut buckets = Vec::new();
    for byte in data {
        if let Some(bucket) = sw.push(byte) {
            buckets.push(bucket);
        }
    }
    
    assert_eq!(buckets, vec![vec![0xFF, 0xAA, 0x55, 0xAA], vec![0x0f, 0x0f, 0xf4, 0x0f]]);
}