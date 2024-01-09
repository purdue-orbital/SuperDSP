pub mod compute_shaders {
    pub mod pointwise_multiplication_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

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
}


