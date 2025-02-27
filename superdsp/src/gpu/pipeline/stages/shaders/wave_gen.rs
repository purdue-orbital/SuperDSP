vulkano_shaders::shader! {
        ty: "compute",
        src: r"
        #version 450
        #extension  GL_EXT_shader_16bit_storage : require
        
        #define M_PI 3.1415926535897932384626433832795

        layout(set = 0, binding = 0) buffer Settings {
            float phi;
            float time_offset;
            float time;
            
            uint sample_rate;
            uint taps;
        } settings;

        layout(set = 0, binding = 1) buffer Taps {
            float16_t arr[];
        } taps;

        void main() {
            uint x = gl_GlobalInvocationID.x;
            float phi = settings.phi * (x + settings.time_offset) * settings.time;
            
            taps.arr[x] = float16_t(cos(phi));
            
            if (x == settings.taps - 1) {
                settings.time_offset += settings.taps;
                settings.time_offset = mod(settings.time_offset, settings.sample_rate);
            }
        }
        "
}
