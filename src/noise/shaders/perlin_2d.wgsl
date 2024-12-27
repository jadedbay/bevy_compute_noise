#define_import_path bevy_compute_noise::perlin2d

#import bevy_render::maths::PI
#import bevy_compute_noise::util::{random_gradient_2d, interpolate_quintic, interpolate_cubic, texture2d as texture}

const TILEABLE: u32 = 1u;
const INVERT: u32 = 2u;
const REMAP: u32 = 4u;
const REMAP_SQRT_2: u32 = 8u;
const INTERPOLATE_CUBIC: u32 = 16u;

struct NoiseParameters {
    seed: u32,
    frequency: u32,
    flags: u32,
};
@group(1) @binding(0) var<uniform> parameters: NoiseParameters;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = invocation_id.xy;

    let value = noise(location, parameters, f32(parameters.frequency));
    textureStore(texture, location, vec4<f32>(value, 0.0, 0.0, 1.0));
}

fn noise(location: vec2<u32>, parameters: NoiseParameters, frequency: f32) -> f32 {
    let texture_size = textureDimensions(texture);
    let uv = vec2<f32>(location) / vec2<f32>(texture_size);

    var value = perlin(uv, parameters, frequency);
    if (parameters.flags & REMAP) != 0u { value = value * 0.5 + 0.5; }
    else if (parameters.flags & REMAP_SQRT_2) != 0 { value = value * sqrt(2.0) * 0.5 + 0.5; }
    if (parameters.flags & INVERT) != 0u { value = 1.0 - value; }

    return value;
}

fn perlin(uv: vec2<f32>, parameters: NoiseParameters, frequency: f32) -> f32 {
    let scaled_uv = uv * frequency;

    let grid_id = floor(scaled_uv);
    var grid_uv = fract(scaled_uv);

    let p00 = vec2<u32>(grid_id + vec2<f32>(0.0, 0.0));
    var p10 = vec2<u32>(grid_id + vec2<f32>(1.0, 0.0));
    var p01 = vec2<u32>(grid_id + vec2<f32>(0.0, 1.0));
    var p11 = vec2<u32>(grid_id + vec2<f32>(1.0, 1.0));

    if (parameters.flags & TILEABLE) != 0u {
        p10 = p10 % u32(frequency);
        p01 = p01 % u32(frequency);
        p11 = p11 % u32(frequency);
    }

    let grad00 = random_gradient_2d(parameters.seed, p00);
    let grad10 = random_gradient_2d(parameters.seed, p10);
    let grad01 = random_gradient_2d(parameters.seed, p01);
    let grad11 = random_gradient_2d(parameters.seed, p11);

    let dist00 = grid_uv;
    let dist10 = grid_uv - vec2<f32>(1.0, 0.0);
    let dist01 = grid_uv - vec2<f32>(0.0, 1.0);
    let dist11 = grid_uv - vec2<f32>(1.0, 1.0);

    let dot00 = dot(grad00, dist00);
    let dot10 = dot(grad10, dist10);
    let dot01 = dot(grad01, dist01);
    let dot11 = dot(grad11, dist11);

    if (parameters.flags & INTERPOLATE_CUBIC) != 0u { grid_uv = interpolate_cubic(grid_uv); }
    else { grid_uv = interpolate_quintic(grid_uv); } 

    let b = mix(dot00, dot10, grid_uv.x);
    let t = mix(dot01, dot11, grid_uv.x);

    let value = mix(b, t, grid_uv.y);

    return value;
}