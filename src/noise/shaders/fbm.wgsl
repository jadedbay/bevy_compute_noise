#ifdef PERLIN2D
    #import bevy_compute_noise::perlin2d::{NoiseParameters, noise}
    @group(1) @binding(1) var<uniform> parameters: NoiseParameters;
#endif
#ifdef WORLEY2D
    #import bevy_compute_noise::worley2d::{NoiseParameters, noise}
    @group(1) @binding(1) var<uniform> parameters: NoiseParameters;
#endif
#ifdef WORLEY3D
    #import bevy_compute_noise::worley3d::{NoiseParameters, noise}
    @group(1) @binding(1) var<uniform> parameters: NoiseParameters;
#endif

#ifdef 2D
    #import bevy_compute_noise::util::texture2d as texture
#endif
#ifdef 3D
    #import bevy_compute_noise::util::texture3d as texture
#endif

struct FbmSettings {
    octaves: u32,
    lacunarity: f32,
    persistence: f32,
}
@group(1) @binding(0) var<uniform> fbm_settings: FbmSettings;

#ifdef 2D
    @compute @workgroup_size(8, 8)
#endif
#ifdef 3D
    @compute @workgroup_size(8, 8, 8)
#endif
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    #ifdef 2D
        let location = invocation_id.xy; 
    #endif
    #ifdef 3D
        let location = invocation_id.xyz; 
    #endif

    var value = 0.0;

    let max_amplitude = (1.0 - pow(fbm_settings.persistence, f32(fbm_settings.octaves))) / (1.0 - fbm_settings.persistence);
    var amplitude = 1.0 / max_amplitude;
    var frequency = f32(parameters.frequency);

    for(var i = 0u; i < fbm_settings.octaves; i++) {
       value += noise(location, parameters, frequency) * amplitude;
       
       frequency *= fbm_settings.lacunarity;
       amplitude *= fbm_settings.persistence;
    }


    textureStore(texture, location, vec4<f32>(value, 0.0, 0.0, 1.0));
}