#define_import_path bevy_compute_noise::util

#import bevy_render::maths::PI
#import bevy_pbr::utils::{rand_vec2f, rand_f}

@group(0) @binding(0) var texture2d: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(0) var texture3d: texture_storage_3d<rgba8unorm, read_write>;

const INFINITY = 3.402823e+38;

const UI0 = 1597334673u;
const UI1 = 3812015801u;
const UI2 = vec2<u32>(UI0, UI1);
const UI3 = vec3<u32>(UI0, UI1, 2798796415u);
const UIF = (1.0 / f32(0xffffffffu));

fn hash22(p: vec2<f32>) -> vec2<f32> {
    var q = vec2<u32>(vec2<i32>(p));
    q = (q.x ^ q.y) * UI2;
    return -1.0 + 2.0 * vec2<f32>(q) * UIF;
}

fn hash33(p: vec3<f32>) -> vec3<f32> {
    var q = vec3<u32>(vec3<i32>(p)) * UI3;
    q = (q.x ^ q.y ^ q.z) * UI3 ;
    return -1.0 + 2.0 * vec3<f32>(q) * UIF;
}

fn remap(x: f32, a: f32, b: f32, c: f32, d: f32) {
    return (((x - a) / (b - a)) * (d - c)) + c;
}

fn interpolate_cubic(w: vec2<f32>) -> vec2<f32> {
   return (3.0 - w * 2.0) * w * w;
}

fn interpolate_quintic(w: vec2<f32>) -> vec2<f32> {
   let w3 = w * w * w;
   let w4 = w3 * w;
   let w5 = w4 * w;
   return 6.0 * w5 - 15.0 * w4 + 10.0 * w3;
}

fn interpolate_cubic_3d(w: vec3<f32>) -> vec3<f32> {
   return (3.0 - w * 2.0) * w * w;
}

fn interpolate_quintic_3d(w: vec3<f32>) -> vec3<f32> {
   let w3 = w * w * w;
   let w4 = w3 * w;
   let w5 = w4 * w;
   return 6.0 * w5 - 15.0 * w4 + 10.0 * w3;
}

fn random_gradient_2d(seed: u32, pos: vec2<u32>) -> vec2<f32> {
    var state = seed + pos.x * 1597u + pos.y * 51749u;
    let v = rand_vec2f(&state) * 2.0 - 1.0;
    return normalize(v);
}

fn rand_vec3f(state: ptr<function, u32>) -> vec3<f32> {
    return vec3(rand_f(state), rand_f(state), rand_f(state));
}

fn random_gradient_3d(seed: u32, pos: vec3<u32>) -> vec3<f32> {
    var state = seed + pos.x * 1597u + pos.y * 51749u + pos.z * 241u;
    let v = rand_vec3f(&state) * 2.0 - 1.0;
    return normalize(v);
}