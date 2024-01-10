use rustdsp::math::vulkan::Vulkan;

fn main() {

    let arr1 = vec![1.0,2.0,3.0];
    let arr2 = vec![0.0,1.0,0.5];

    let v = Vulkan::default();

    let buffer1 = v.store_to_vram(arr1.as_slice());
    let buffer2 = v.store_to_vram(arr2.as_slice());

    let mut builder = v.create_command_builder();

    let out= builder.convolution_f32(buffer1.clone(),buffer2.clone());

    let built = builder.build();

    v.run_command_builder(built).wait(None).unwrap();

    dbg!(out.read().unwrap().to_vec());
}