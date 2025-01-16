#import bevy_compute_noise::perlin::{Perlin, perlin_2d}
#import bevy_compute_noise::invert::invert

@group(0) @binding(0) var texture1: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1) var<uniform> perlin1: Perlin;

@compute @workgroup_size(32, 32)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let coords = invocation_id.xy;
    let texture_size = textureDimensions(texture1);
    let uv = vec2<f32>(coords) / vec2<f32>(texture_size);
    var value;

    value = perlin_2d(uv, perlin1);
    value = invert(value);

    textureStore(texture1, coords, vec4<f32>(value, 0.0, 0.0, 0.0));
}