#ifdef 2D
    #import bevy_compute_noise::util::texture2d as texture
    #ifdef PERLIN
        #import bevy_compute_noise::perlin_2d::{Perlin as Noise, perlin_2d as noise_fn}
    #endif
    #ifdef WORLEY
        #import bevy_compute_noise::worley_2d::{Worley as Noise, worley_2d as noise_fn}
    #endif
#endif
#ifdef 3D
    #import bevy_compute_noise::util::texture3d as texture
    #ifdef PERLIN
        #import bevy_compute_noise::perlin_3d::{Perlin as Noise, perlin_3d as noise_fn}
    #endif
    #ifdef WORLEY
        #import bevy_compute_noise::worley_3d::{Worley as Noise, worley_3d as noise_fn}
    #endif
#endif

struct Config {
    octaves: u32,
    lacunarity: f32,
    persistence: f32,
    _padding: u32,
    noise: Noise,
}
@group(0) @binding(1) var<uniform> config: Config;

#ifdef 2D
    @compute @workgroup_size(32, 32)
#endif
#ifdef 3D
    @compute @workgroup_size(8, 8, 8)
#endif
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let texture_size = textureDimensions(texture);
    #ifdef 2D
        let location = invocation_id.xy; 
        let uv = vec2<f32>(location) / vec2<f32>(texture_size);
    #endif
    #ifdef 3D
        let location = invocation_id.xyz; 
        let uv = vec3<f32>(location) / vec3<f32>(texture_size);
    #endif

    var value = 0.0;

    let max_amplitude = (1.0 - pow(config.persistence, f32(config.octaves))) / (1.0 - config.persistence);
    var amplitude = 1.0 / max_amplitude;
    var noise = config.noise;

    for(var i = 0u; i < config.octaves; i++) {
       value += noise_fn(uv, noise) * amplitude;
       
       noise.frequency *= config.lacunarity;
       amplitude *= config.persistence;
    }

    textureStore(texture, location, vec4<f32>(value, 0.0, 0.0, 1.0));
}