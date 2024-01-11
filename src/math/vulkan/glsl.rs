pub mod compute_shaders {
    pub mod pointwise_multiplication_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source {
                        float data[];
                    } src;

                    layout(set = 1, binding = 1) buffer Destentaion {
                        float data[];
                    } dest;


                    void main() {
                        uint idx = gl_GlobalInvocationID.x;
                        dest.data[idx] *= src.data[idx];
                    }
                ",
        }
    }

    pub mod convolution_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source1 {
                        float data[];
                    } src1;

                    layout(set = 1, binding = 1) buffer Source2 {
                        float data[];
                    } src2;

                    layout(set = 2, binding = 2) buffer Destentaion {
                        float data[];
                    } dest;

                    void main() {
                        for(int i = 0; i < src2.data.length(); i++){
                            dest.data[gl_GlobalInvocationID.x + i] += src1.data[gl_GlobalInvocationID.x] * src2.data[i];
                        }
                    }
                ",
        }
    }

    pub mod scalar_multiplication_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 32, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Scalar {
                        float data;
                    } scalar;

                    layout(set = 1, binding = 1) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] *= scalar.data;
                    }
                ",
        }
    }
}


