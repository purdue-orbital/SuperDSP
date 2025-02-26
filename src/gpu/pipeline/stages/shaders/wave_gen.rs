vulkano_shaders::shader! {
        ty: "compute",
        src: r"
        #version 450

        layout(set = 0, binding = 0) buffer Settings {
            float phi;
            float phi_offset;
            
            uint taps;
        } settings;

        layout(set = 0, binding = 1) buffer Taps {
            float arr[];
        } taps;

        void main() {
            uint x = gl_GlobalInvocationID.x;
            float phi = settings.phi * float(x) + settings.phi_offset;
            taps.arr[x] = sin(phi);
            
            if (x == settings.taps - 1) {
                settings.phi_offset += phi;
                settings.phi_offset = mod(settings.phi_offset, 2.0 * 3.14159265359);
            }
        }
        "
}
