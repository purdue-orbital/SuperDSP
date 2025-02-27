use num_complex::Complex;

pub fn duplicator(data: &[f32], factor: usize) -> Vec<f32> {
    let mut output = Vec::new();
    for &sample in data {
        for _ in 0..factor {
            output.push(sample);
        }
    }
    output
}

pub fn duplicator_i8(data: &[i8], factor: usize) -> Vec<i8> {
    let mut output = Vec::new();
    for &sample in data {
        for _ in 0..factor {
            output.push(sample);
        }
    }
    output
}

pub fn duplicator_u8(data: &[u8], factor: usize) -> Vec<u8> {
    let mut output = Vec::new();
    for &sample in data {
        for _ in 0..factor {
            output.push(sample);
        }
    }
    output
}

pub fn duplicator_complex(data: &[Complex<f32>], factor: usize) -> Vec<Complex<f32>> {
    let mut output = Vec::new();
    for &sample in data {
        for _ in 0..factor {
            output.push(sample);
        }
    }
    output
}

pub fn unduplicator(data: &[f32], factor: usize, offset: usize) -> Vec<f32> {
    let mut output = Vec::new();
    for i in (0..data.len()).skip(offset).step_by(factor) {
        output.push(data[i]);
    }
    output
}

pub fn unduplicator_i8(data: &[i8], factor: usize, offset: usize) -> Vec<i8> {
    let mut output = Vec::new();
    for i in (0..data.len()).skip(offset).step_by(factor) {
        output.push(data[i]);
    }
    output
}

pub fn unduplicator_u8(data: &[u8], factor: usize, offset: usize) -> Vec<u8> {
    let mut output = Vec::new();
    for i in (0..data.len()).skip(offset).step_by(factor) {
        output.push(data[i]);
    }
    output
}

pub fn unduplicator_complex(data: &[Complex<f32>], factor: usize, offset: usize) -> Vec<Complex<f32>> {
    let mut output = Vec::new();
    for i in (0..data.len()).skip(offset).step_by(factor) {
        output.push(data[i]);
    }
    output
}