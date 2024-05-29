const PI: f32 = 3.141592653589793;

@group(0) @binding(0)
var texture: texture_storage_2d<r8unorm, write>;
@group(0) @binding(1)
var<storage, read> texture_size: vec2<f32>;

struct NoiseParameters {
    seed: u32,
    frequency: u32,
    octaves: u32,
};
@group(1) @binding(0) var<storage, read> parameters: NoiseParameters;

@compute @workgroup_size(8, 8)
fn noise(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<u32>(
        invocation_id.x,
        invocation_id.y
    );

    let base_frequency = parameters.frequency;
    var frequency = parameters.frequency;
    var amplitude = 1.0;

    var value = 0.0;

    for (var index: u32 = 0; index < parameters.octaves; index++) {
        let pixel = vec2<f32>(location) * f32(frequency) / texture_size;
        value += perlin(pixel, i32(frequency)) * amplitude;

        frequency *= 2u;
        amplitude /= 2.0;
    }

    textureStore(texture, location, vec4<f32>((value + 1.0) / 2.0, 0.0, 0.0, 0.0));
}

fn perlin(pixel: vec2<f32>, frequency: i32) -> f32 {
    let f = pixel;
    let i = vec2<i32>(f);
    let s = f - vec2<f32>(i);

    var n0 = dot_grid_gradient(i, f, frequency);
    var n1 = dot_grid_gradient(i + vec2<i32>(1, 0), f, frequency);
    let ix0 = interpolate_cubic(n0, n1, s.x);

    n0 = dot_grid_gradient(i + vec2<i32>(0, 1), f, frequency);
    n1 = dot_grid_gradient(i + vec2<i32>(1, 1), f, frequency);
    let ix1 = interpolate_cubic(n0, n1, s.x);

    let value = interpolate_cubic(ix0, ix1, s.y);

    return value;
}

fn dot_grid_gradient(i: vec2<i32>, f: vec2<f32>, frequency: i32) -> f32 {
    let gradient = random_gradient(i % frequency + 1);

    let distance_vector = f - vec2<f32>(i);

    return dot(gradient, distance_vector);
}

fn interpolate_cubic(a0: f32, a1: f32, w: f32) -> f32 {
    return (a1 - a0) * (3.0 - w * 2.0) * w * w + a0;
}

fn random_gradient(i: vec2<i32>) -> vec2<f32> {
    let w = 32u;
    let s = w / 2u;
    var a = u32(i.x) + parameters.seed;
    var b = u32(i.y) + parameters.seed;

    a *= 3284157443u;
    b ^= (b << s) | (b >> (w - s));
    b *= 1911520717u;

    a ^= (b << s) | (b >> (w - s));
    a *= 2048419325u;

    let random: f32 = f32(a) * (PI / f32(~0u >> 1));

    let v = vec2<f32>(sin(random), cos(random));

    return v;
}
