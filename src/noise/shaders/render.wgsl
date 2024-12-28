#import bevy_compute_noise::perlin2d::{NoiseParameters, noise, perlin}
@group(0) @binding(1) var<uniform> parameters: NoiseParameters;
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct FbmSettings {
    octaves: u32,
    lacunarity: f32,
    persistence: f32,
}
@group(1) @binding(0) var<uniform> fbm_settings: FbmSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let value = perlin(in.uv, parameters, f32(parameters.frequency));
    return vec4<f32>(value, value, value, 1.0);
}