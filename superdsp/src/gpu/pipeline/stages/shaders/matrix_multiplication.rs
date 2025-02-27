mod matrix_multiplication {
    vulkano_shaders::shader! {
    ty: "compute",
    src: r"
    #version 450
    #extension GL_EXT_shader_16bit_storage : require
    #extension GL_EXT_shader_explicit_arithmetic_types_float16 : require

    layout(set = 0, binding = 0) buffer Left_Matrix {
        float16_t arr[];
    } left_matrix;
    
    layout(set = 0, binding = 1) buffer Right_Matrix {
        float16_t arr[];
    } right_matrix;

    layout(set = 0, binding = 2) buffer Target {
        float16_t arr[];
    } target;

    void main() {
        uint x = gl_GlobalInvocationID.x;
        uint y = gl_GlobalInvocationID.y;
        
        uint x_len = gl_WorkGroupSize.x;
        
        target.arr[x + (y * x_len)] = float16_t(0.0);

        for (int i = 0; i < x_len; ++i) {
            target.arr[x + (y * x_len)] += left_matrix.arr[(y * x_len) + i] * right_matrix.arr[(i * x_len) + y];
        }
    }
    "
}
}