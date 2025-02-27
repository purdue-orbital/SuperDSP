
vulkano_shaders::shader! {
    ty: "compute",
    src: r"
    #version 450
    #extension  GL_EXT_shader_16bit_storage : require

    layout(set = 0, binding = 0) buffer Source {
        float16_t arr[];
    } source;

    layout(set = 0, binding = 1) buffer Target {
        float16_t arr[];
    } target;

    void main() {
        uint x = gl_GlobalInvocationID.x;
        target.arr[x] = source.arr[x];
    }
    "
}
