/// Creates a flat top filter of size n
/// 
/// 
/// uses matlab constants, these constants will change the filter.
/// i - Offset add offset maybe other var to take in?
pub fn flat_top_window<const N: usize>(offset:usize) -> [f32; N] {
    const A_0: f32 = 0.21557895;
    const A_1: f32 = 0.41663158;
    const A_2: f32 = 0.277263158;
    const A_3: f32 = 0.083578947;
    const A_4: f32 = 0.006947368;
    let mut filter: [f32;N] = [0.0; N];
    for i in 0..N{
      let n = (i as f32) - (offset as f32);
       filter[i] = A_0 - A_1 * libm::cosf((2 as f32 * core::f32::consts::PI * n as f32)/N as f32) + A_2 * libm::cosf((4 as f32 * core::f32::consts::PI * n as f32)/N as f32) - A_3 * libm::cosf((6 as f32 * core::f32::consts::PI * n as f32)/N as f32) + A_4 * libm::cosf((8 as f32 * core::f32::consts::PI * n as f32)/N as f32);
    }
   filter
}