use crate::filters::fir::rectangular_pulse_shaping;

/// This implements a NRZ encoder which is used to convert bits into a rectangular pulse wave
pub fn nrz(bytes: &[u8], samples_per_symbol:usize)->Vec<f32>{
    let lc = level_converter(bytes);

    rectangular_pulse_shaping(lc.as_slice(),samples_per_symbol-1)
}

/// This will convert bytes into individual bits and make 0 = -1 and 1 = 1
fn level_converter(bytes:&[u8])->Vec<f32>{
    let mut out = Vec::with_capacity(8*bytes.len());

    for x in bytes{
        for y in (0..8).rev(){
            out.push(
                if ((x>>y) & 1) == 1{
                    1.0
                }else {
                    -1.0
                }
            )
        }
    }

    out
}


