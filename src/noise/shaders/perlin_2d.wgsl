#define_import_path bevy_compute_noise::perlin2d

#import bevy_render::maths::PI
#import bevy_compute_noise::util::{random_gradient, interpolate_quintic, texture2d as texture}

struct NoiseParameters {
    seed: u32,
    frequency: f32,
    invert: u32,
};
@group(1) @binding(0) var<uniform> parameters: NoiseParameters;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = invocation_id.xy;

    let value = noise(location, parameters);
    textureStore(texture, location, vec4<f32>(value, 0.0, 0.0, 1.0));
}

fn noise(location: vec2<u32>, parameters: NoiseParameters) -> f32 {
    let texture_size = textureDimensions(texture);

    let frequency = parameters.frequency;
    let pixel = vec2<f32>(location) * f32(frequency) / vec2<f32>(texture_size);
    var value = perlin(pixel, parameters);
    if (parameters.invert != 0u) {
        value = 1.0 - value;
    }

    return value;
}

fn perlin(pixel: vec2<f32>, parameters: NoiseParameters) -> f32 {
    let seed = parameters.seed;
    let frequency = parameters.frequency;
    let f = pixel;
    let i = vec2<i32>(f);
    let s = f - vec2<f32>(i);

    var n0 = dot_grid_gradient(seed, i, f, frequency);
    var n1 = dot_grid_gradient(seed, i + vec2<i32>(1, 0), f, frequency);
    let ix0 = interpolate_quintic(n0, n1, s.x);

    n0 = dot_grid_gradient(seed, i + vec2<i32>(0, 1), f, frequency);
    n1 = dot_grid_gradient(seed, i + vec2<i32>(1, 1), f, frequency);
    let ix1 = interpolate_quintic(n0, n1, s.x);

    let value = interpolate_quintic(ix0, ix1, s.y);

    return value;
}

fn dot_grid_gradient(seed: u32, i: vec2<i32>, f: vec2<f32>, frequency: f32) -> f32 {
    let wrapped_i = vec2<f32>(vec2<f32>(i) * frequency % 1.0);
    let gradient = random_gradient(seed, wrapped_i);
    let distance_vector = f - vec2<f32>(i);

    return dot(gradient, distance_vector);
}