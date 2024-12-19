#import bevy_render::maths::PI
#import bevy_compute_noise::util::{random_gradient, interpolate_quintic}

@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1)
var<storage, read> texture_size: vec2<f32>;

struct NoiseParameters {
    seed: u32,
    frequency: u32,
    octaves: u32,
    invert: u32,
    persistence: f32,
};
@group(1) @binding(0) var<storage, read> parameters: NoiseParameters;

@compute @workgroup_size(8, 8)
fn noise(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<u32>(
        invocation_id.x,
        invocation_id.y
    );

    // let base_frequency = parameters.frequency;
    // var frequency = parameters.frequency;
    // var amplitude = 1.0;

    // var value = 0.0;

    // for (var index: u32 = 0; index < parameters.octaves; index++) {
    //     let pixel = vec2<f32>(location) * f32(frequency) / texture_size;
    //     value += perlin(pixel, i32(frequency)) * amplitude;

    //     frequency *= 2u;
    //     amplitude /= 2.0;
    // }

    // value = (value + 1.0) / 2.0;

    // if (parameters.invert != 0u) {
    //     value = 1.0 - value;
    // }

    // value = mix(textureLoad(texture, location).r, value, parameters.persistence);

    let texture_size = textureDimensions(texture);

    let frequency = parameters.frequency;
    let pixel = vec2<f32>(location) * f32(frequency) / vec2<f32>(texture_size);
    let value = perlin(pixel, i32(frequency));

    textureStore(texture, location, vec4<f32>(value, 0.0, 0.0, 1.0));
}

fn perlin(pixel: vec2<f32>, frequency: i32) -> f32 {
    let seed = parameters.seed;
    let f = pixel;
    let i = vec2<i32>(f);
    let s = f - vec2<f32>(i);

    var n0 = dot_grid_gradient(i, f, frequency);
    var n1 = dot_grid_gradient(i + vec2<i32>(1, 0), f, frequency);
    let ix0 = interpolate_quintic(n0, n1, s.x);

    n0 = dot_grid_gradient(i + vec2<i32>(0, 1), f, frequency);
    n1 = dot_grid_gradient(i + vec2<i32>(1, 1), f, frequency);
    let ix1 = interpolate_quintic(n0, n1, s.x);

    let value = interpolate_quintic(ix0, ix1, s.y);

    return value;
}

fn dot_grid_gradient(i: vec2<i32>, f: vec2<f32>, frequency: i32) -> f32 {
    let gradient = random_gradient(parameters.seed, i % frequency + 1);
    let distance_vector = f - vec2<f32>(i);

    return dot(gradient, distance_vector);
}