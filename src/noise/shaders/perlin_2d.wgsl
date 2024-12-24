#define_import_path bevy_compute_noise::perlin2d

#import bevy_render::maths::PI
#import bevy_compute_noise::util::{random_gradient, interpolate_quintic, interpolate_cubic, texture2d as texture}

const INVERT: u32 = 1u;
const REMAP: u32 = 2u;
const REMAP_SQRT_2: u32 = 4u;
const INTERPOLATE_CUBIC: u32 = 8u;

struct NoiseParameters {
    seed: u32,
    frequency: f32,
    flags: u32,
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
    let pixel = vec2<f32>(location) / vec2<f32>(texture_size);

    var value = perlin(pixel, parameters);
    if (parameters.flags & REMAP) != 0u { value = value * 0.5 + 0.5; }
    else if (parameters.flags & REMAP_SQRT_2) != 0 { value = value * sqrt(2.0) * 0.5 + 0.5; }
    if (parameters.flags & INVERT) != 0u { value = 1.0 - value; }

    return value;
}

fn perlin(pixel: vec2<f32>, parameters: NoiseParameters) -> f32 {
    let seed = parameters.seed;
    let frequency = parameters.frequency;

    let uv = pixel * frequency;

    let grid_id = floor(uv) % frequency;
    var grid_uv = fract(uv);

    let bl = vec2<u32>(grid_id + vec2<f32>(0.0, 0.0));
    let br = vec2<u32>((grid_id + vec2<f32>(1.0, 0.0)) % frequency);
    let tl = vec2<u32>((grid_id + vec2<f32>(0.0, 1.0)) % frequency);
    let tr = vec2<u32>((grid_id + vec2<f32>(1.0, 1.0)) % frequency);

    let grad_bl = random_gradient(seed, bl);
    let grad_br = random_gradient(seed, br);
    let grad_tl = random_gradient(seed, tl);
    let grad_tr = random_gradient(seed, tr);

    let dist_bl = grid_uv;
    let dist_br = grid_uv - vec2<f32>(1.0, 0.0);
    let dist_tl = grid_uv - vec2<f32>(0.0, 1.0);
    let dist_tr = grid_uv - vec2<f32>(1.0, 1.0);

    let dot_bl = dot(grad_bl, dist_bl);
    let dot_br = dot(grad_br, dist_br);
    let dot_tl = dot(grad_tl, dist_tl);
    let dot_tr = dot(grad_tr, dist_tr);

    if (parameters.flags & INTERPOLATE_CUBIC) != 0u { grid_uv = interpolate_cubic(grid_uv); }
    else { grid_uv = interpolate_quintic(grid_uv); } 

    let b = mix(dot_bl, dot_br, grid_uv.x);
    let t = mix(dot_tl, dot_tr, grid_uv.x);

    let value = mix(b, t, grid_uv.y);

    return value;
}