pub mod compute_shaders {
    pub mod pointwise_multiplication_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

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

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

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
                        for(int i = 0; i < src1.data.length() && i <= gl_GlobalInvocationID.x; i++){
                            dest.data[gl_GlobalInvocationID.x] += src1.data[i] * src2.data[gl_GlobalInvocationID.x - i];
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
                        float value;
                    } scal;

                    layout(set = 1, binding = 1) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] *= scal.value;
                    }
                ",
        }
    }

    pub mod sin_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] = sin(dest.data[gl_GlobalInvocationID.x]);
                    }
                ",
        }
    }

    pub mod cos_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] = cos(dest.data[gl_GlobalInvocationID.x]);
                    }
                ",
        }
    }

    pub mod mod_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Scalar {
                        float value;
                    } scal;

                    layout(set = 1, binding = 1) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                                                                                                                             // Fucking rounding errors
                        dest.data[gl_GlobalInvocationID.x] -= scal.value * floor((dest.data[gl_GlobalInvocationID.x] / scal.value) + 0.0001);
                    }
                ",
        }
    }

    pub mod add_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Scalar {
                        float data[];
                    } src;

                    layout(set = 1, binding = 1) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] += src.data[gl_GlobalInvocationID.x];
                    }
                ",
        }
    }

    pub mod scalar_add_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Scalar {
                        float data;
                    } scalar;

                    layout(set = 1, binding = 1) buffer Source {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] += scalar.data;
                    }
                ",
        }
    }

    pub mod copy_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source {
                        float data[];
                    } src;

                    layout(set = 1, binding = 1) buffer Dest {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] = src.data[gl_GlobalInvocationID.x];
                    }
                ",
        }
    }

    pub mod fetch_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer Source {
                        float data[];
                    } src;

                    layout(set = 1, binding = 1) buffer Indexes {
                        float data[];
                    } indexes;

                    layout(set = 2, binding = 2) buffer Dest {
                        float data[];
                    } dest;

                    void main() {
                        dest.data[gl_GlobalInvocationID.x] = src.data[int(indexes.data[gl_GlobalInvocationID.x])];
                    }
                ",
        }
    }

    pub mod dft_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    #define M_PI 3.1415926535897932384626433832795

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer I_Source {
                        float data[];
                    } i_src;

                    layout(set = 1, binding = 1) buffer Q_Source {
                        float data[];
                    } q_src;

                    layout(set = 2, binding = 2) buffer I_Dest {
                        float data[];
                    } i_dest;

                    layout(set = 3, binding = 3) buffer Q_Dest {
                        float data[];
                    } q_dest;

                    void main() {
                        float step = -2 * M_PI * (float(gl_GlobalInvocationID.x) / float(i_src.data.length()));
                        float phi = 0;
                        uint thread = gl_GlobalInvocationID.x;

                        for (int i = 0; i < i_src.data.length(); i++){
                            phi = step * i;

                            // Set i value
                            i_dest.data[thread] += i_src.data[thread] * cos(phi) - q_src.data[thread] * sin(phi);
                            q_dest.data[thread] += i_src.data[thread] * sin(phi) + q_src.data[thread] * cos(phi);
                        }
                    }
                ",
        }
    }
    pub mod idft_f32 {
        vulkano_shaders::shader! {
                ty: "compute",
                src: r"
                    #version 460

                    #define M_PI 3.1415926535897932384626433832795

                    layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

                    layout(set = 0, binding = 0) buffer I_Source {
                        float data[];
                    } i_src;

                    layout(set = 1, binding = 1) buffer Q_Source {
                        float data[];
                    } q_src;

                    layout(set = 2, binding = 2) buffer I_Dest {
                        float data[];
                    } i_dest;

                    layout(set = 3, binding = 3) buffer Q_Dest {
                        float data[];
                    } q_dest;

                    void main() {
                        float step = 2 * M_PI * (float(gl_GlobalInvocationID.x) / float(i_src.data.length()));
                        float phi = 0;
                        uint thread = gl_GlobalInvocationID.x;

                        for (int i = 0; i < i_src.data.length(); i++){
                            phi = step * i;

                            // Set i value
                            i_dest.data[thread] += i_src.data[thread] * cos(phi) - q_src.data[thread] * sin(phi);
                            q_dest.data[thread] += i_src.data[thread] * sin(phi) + q_src.data[thread] * cos(phi);
                        }
                        
                        i_dest.data[thread] /= float(i_src.data.length());
                        q_dest.data[thread] /= float(i_src.data.length());
                    }
                ",
        }
    }
}

