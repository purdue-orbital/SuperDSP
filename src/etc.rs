/// This will check bit by bit if the pattern is present in the window and then fill the bucket with data that follows the pattern
pub struct SlidingWindow {
    pub window: Vec<u8>,
    
    pub pattern: Vec<u8>,
    
    pub bucket_len: usize,
    pub bucket: Vec<u8>,
    
    pub counter: usize,
    pub index: usize,
    
    pub pattern_found: bool,
}

impl SlidingWindow {
    pub fn new(pattern: Vec<u8>, bucket_len: usize) -> SlidingWindow {
        SlidingWindow {
            window: vec![0; pattern.len()],
            pattern,
            bucket_len,
            bucket: vec![0; bucket_len],
            pattern_found: false,
            counter: 7,
            index: 0,
        }
    }
    
    pub fn push(&mut self, data: u8) -> Option<Vec<u8>> {
        // break up data into bits
        for i in (0..8).rev() {
            let mut bit = (data >> i) & 1;
            
            if !self.pattern_found {
                // slide bits of window
                for j in (0..self.window.len()).rev() {
                    let hold_bit = self.window[j] >> 7;

                    self.window[j] <<= 1;
                    self.window[j] |= bit;

                    bit = hold_bit;
                }
                
                // Check if window matches pattern
                if self.window == self.pattern {
                    self.counter = 7;
                    self.pattern_found = true;
                    
                    self.window = vec![0; self.pattern.len()];
                    self.bucket = vec![0; self.bucket_len];
                }
                
            }else {
                self.bucket[self.index] |= bit << self.counter;
                
                if self.counter == 0 {
                    self.counter = 7;
                    self.index += 1;

                    if self.index == self.bucket_len {
                        self.index = 0;
                        self.pattern_found = false;
                        return Some(self.bucket.clone());
                    }

                    self.bucket[self.index] = 0;
                }else {
                    self.counter -= 1;
                }
            }
        }
        
        None
    }
}