
vulkano_shaders::shader! {
    ty: "compute",
    src: r"
    #version 450

    layout(set = 0, binding = 0) buffer Source {
        float arr[];
    } source;

    layout(set = 0, binding = 1) buffer Target {
        float arr[];
    } target;

    void main() {
        uint x = gl_GlobalInvocationID.x;
        target.arr[x] = source.arr[x];
    }
    "
}
