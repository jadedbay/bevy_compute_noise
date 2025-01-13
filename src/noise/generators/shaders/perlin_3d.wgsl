#define_import_path bevy_compute_noise::perlin_3d

#import bevy_render::maths::PI
#import bevy_compute_noise::util::{random_gradient_3d, interpolate_quintic_3d, interpolate_cubic_3d, texture3d as texture}

const TILEABLE: u32 = 1u;
const REMAP: u32 = 2u;
const INTERPOLATE_CUBIC: u32 = 4u;

struct Perlin {
    seed: u32,
    frequency: f32,
    flags: u32,
};
@group(1) @binding(0) var<uniform> perlin: Perlin;

@compute @workgroup_size(8, 8, 8)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = invocation_id.xyz;
    let texture_size = textureDimensions(texture);
    let uv = vec3<f32>(location) / vec3<f32>(texture_size);

    let value = perlin_3d(uv, perlin);
    textureStore(texture, location, vec4<f32>(value, 0.0, 0.0, 1.0));
}

fn perlin_3d(uv: vec3<f32>, perlin: Perlin) -> f32 {
    var frequency = perlin.frequency;
    if (perlin.flags & TILEABLE) != 0u { frequency = floor(frequency); }
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

    if (perlin.flags & TILEABLE) != 0u {
        p100 = p100 % u32(frequency);
        p010 = p010 % u32(frequency);
        p110 = p110 % u32(frequency);
        p001 = p001 % u32(frequency);
        p101 = p101 % u32(frequency);
        p011 = p011 % u32(frequency);
        p111 = p111 % u32(frequency);
    }

    let grad000 = random_gradient_3d(perlin.seed, p000);
    let grad100 = random_gradient_3d(perlin.seed, p100);
    let grad010 = random_gradient_3d(perlin.seed, p010);
    let grad110 = random_gradient_3d(perlin.seed, p110);
    let grad001 = random_gradient_3d(perlin.seed, p001);
    let grad101 = random_gradient_3d(perlin.seed, p101);
    let grad011 = random_gradient_3d(perlin.seed, p011);
    let grad111 = random_gradient_3d(perlin.seed, p111);

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

    if (perlin.flags & INTERPOLATE_CUBIC) != 0u { grid_uv = interpolate_cubic_3d(grid_uv); }
    else { grid_uv = interpolate_quintic_3d(grid_uv); } 

    let x00 = mix(dot000, dot100, grid_uv.x);
    let x10 = mix(dot010, dot110, grid_uv.x);
    let x01 = mix(dot001, dot101, grid_uv.x);
    let x11 = mix(dot011, dot111, grid_uv.x);

    let y0 = mix(x00, x10, grid_uv.y);
    let y1 = mix(x01, x11, grid_uv.y);

    var value = mix(y0, y1, grid_uv.z) * 1.154701;

    if (perlin.flags & REMAP) != 0u { value = value * 0.5 + 0.5; }

    return value;
}