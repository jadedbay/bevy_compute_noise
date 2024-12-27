#define_import_path bevy_compute_noise::perlin3d

#import bevy_render::maths::PI
#import bevy_compute_noise::util::{random_gradient_3d, interpolate_quintic_3d, interpolate_cubic_3d, texture3d as texture}

const TILEABLE: u32 = 1u;
const INVERT: u32 = 2u;
const REMAP: u32 = 4u;
const REMAP_SQRT_3: u32 = 8u;
const INTERPOLATE_CUBIC: u32 = 16u;

struct NoiseParameters {
    seed: u32,
    frequency: u32,
    flags: u32,
};
@group(1) @binding(0) var<uniform> parameters: NoiseParameters;

@compute @workgroup_size(8, 8, 8)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = invocation_id.xyz;

    let value = noise(location, parameters, f32(parameters.frequency));
    textureStore(texture, location, vec4<f32>(value, 0.0, 0.0, 1.0));
}

fn noise(location: vec3<u32>, parameters: NoiseParameters, frequency: f32) -> f32 {
    let texture_size = textureDimensions(texture);
    let uv = vec3<f32>(location) / vec3<f32>(texture_size);

    var value = perlin(uv, parameters, frequency);
    if (parameters.flags & REMAP) != 0u { value = value * 0.5 + 0.5; }
    else if (parameters.flags & REMAP_SQRT_3) != 0 { value = value * sqrt(3.0) * 0.5 + 0.5; }
    if (parameters.flags & INVERT) != 0u { value = 1.0 - value; }

    return value;
}

fn perlin(uv: vec3<f32>, parameters: NoiseParameters, frequency: f32) -> f32 {
    let scaled_uv = uv * frequency;

    let grid_id = floor(scaled_uv) % frequency;
    var grid_uv = fract(scaled_uv);

    let p000 = vec3<u32>(grid_id + vec3<f32>(0.0, 0.0, 0.0));
    var p100 = vec3<u32>(grid_id + vec3<f32>(1.0, 0.0, 0.0));
    var p010 = vec3<u32>(grid_id + vec3<f32>(0.0, 1.0, 0.0));
    var p110 = vec3<u32>(grid_id + vec3<f32>(1.0, 1.0, 0.0));
    var p001 = vec3<u32>(grid_id + vec3<f32>(0.0, 0.0, 1.0));
    var p101 = vec3<u32>(grid_id + vec3<f32>(1.0, 0.0, 1.0));
    var p011 = vec3<u32>(grid_id + vec3<f32>(0.0, 1.0, 1.0));
    var p111 = vec3<u32>(grid_id + vec3<f32>(1.0, 1.0, 1.0));

    if (parameters.flags & TILEABLE) != 0u {
        p100 = p100 % u32(frequency);
        p010 = p010 % u32(frequency);
        p110 = p110 % u32(frequency);
        p001 = p001 % u32(frequency);
        p101 = p101 % u32(frequency);
        p011 = p011 % u32(frequency);
        p111 = p111 % u32(frequency);
    }

    let grad000 = random_gradient_3d(parameters.seed, p000);
    let grad100 = random_gradient_3d(parameters.seed, p100);
    let grad010 = random_gradient_3d(parameters.seed, p010);
    let grad110 = random_gradient_3d(parameters.seed, p110);
    let grad001 = random_gradient_3d(parameters.seed, p001);
    let grad101 = random_gradient_3d(parameters.seed, p101);
    let grad011 = random_gradient_3d(parameters.seed, p011);
    let grad111 = random_gradient_3d(parameters.seed, p111);

    let dist000 = grid_uv;
    let dist100 = grid_uv - vec3<f32>(1.0, 0.0, 0.0);
    let dist010 = grid_uv - vec3<f32>(0.0, 1.0, 0.0);
    let dist110 = grid_uv - vec3<f32>(1.0, 1.0, 0.0);
    let dist001 = grid_uv - vec3<f32>(0.0, 0.0, 1.0);
    let dist101 = grid_uv - vec3<f32>(1.0, 0.0, 1.0);
    let dist011 = grid_uv - vec3<f32>(0.0, 1.0, 1.0);
    let dist111 = grid_uv - vec3<f32>(1.0, 1.0, 1.0);

    let dot000 = dot(grad000, dist000);
    let dot100 = dot(grad100, dist100);
    let dot010 = dot(grad010, dist010);
    let dot110 = dot(grad110, dist110);
    let dot001 = dot(grad001, dist001);
    let dot101 = dot(grad101, dist101);
    let dot011 = dot(grad011, dist011);
    let dot111 = dot(grad111, dist111);

    if (parameters.flags & INTERPOLATE_CUBIC) != 0u { grid_uv = interpolate_cubic_3d(grid_uv); }
    else { grid_uv = interpolate_quintic_3d(grid_uv); } 

    let x00 = mix(dot000, dot100, grid_uv.x);
    let x10 = mix(dot010, dot110, grid_uv.x);
    let x01 = mix(dot001, dot101, grid_uv.x);
    let x11 = mix(dot011, dot111, grid_uv.x);

    let y0 = mix(x00, x10, grid_uv.y);
    let y1 = mix(x01, x11, grid_uv.y);

    let value = mix(y0, y1, grid_uv.z);

    return value;
}