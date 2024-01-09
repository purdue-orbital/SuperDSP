use rustdsp::math::vulkan::Vulkan;

fn main() {

    let arr1 = vec![3.0;1e6 as usize];
    let arr2 = vec![50.0;1e6 as usize];

    let v = Vulkan::default();

    let buffer1 = v.store_to_vram(arr1.as_slice());
    let buffer2 = v.store_to_vram(arr2.as_slice());

    let mut builder = v.create_command_builder();

    builder.elementwise_multiply_f32(buffer1.clone(),buffer2.clone());

    let built = builder.build();

    v.run_command_builder(built).wait(None).unwrap();

    dbg!(buffer2.read().unwrap().to_vec().len());
}