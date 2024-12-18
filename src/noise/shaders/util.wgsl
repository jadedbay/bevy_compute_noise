#define_import_path bevy_compute_noise::util

#import bevy_render::maths::PI

fn interpolate_cubic(a0: f32, a1: f32, w: f32) -> f32 {
    return (a1 - a0) * (3.0 - w * 2.0) * w * w + a0;
}

fn interpolate_quintic(a0: f32, a1: f32, w: f32) -> f32 {
    let w3 = w * w * w;
    let w4 = w3 * w;
    let w5 = w4 * w;
    return a0 + (a1 - a0) * (6.0 * w5 - 15.0 * w4 + 10.0 * w3);
}

fn random_gradient(seed: u32, i: vec2<i32>) -> vec2<f32> {
    let w = 32u;
    let s = w / 2u;
    var a = u32(i.x) + seed;
    var b = u32(i.y) + seed;

    a *= 3284157443u;
    b ^= (b << s) | (b >> (w - s));
    b *= 1911520717u;

    a ^= (b << s) | (b >> (w - s));
    a *= 2048419325u;

    let random: f32 = f32(a) * (PI / f32(~0u >> 1));

    let v = vec2<f32>(sin(random), cos(random));

    return v;
}
