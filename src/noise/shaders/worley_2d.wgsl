#define_import_path bevy_compute_noise::worley2d

#import bevy_compute_noise::util::{hash22, INFINITY, texture2d as texture}

struct NoiseParameters {
    seed: u32,
    frequency: f32,
    invert: u32,
};
@group(1) @binding(0)
var<uniform> parameters: NoiseParameters;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let value = noise(invocation_id.xy, parameters);
    textureStore(texture, invocation_id.xy, vec4<f32>(value, 0.0, 0.0, 1.0));
}

fn noise(ulocation: vec2<u32>, parameters: NoiseParameters) -> f32 {
    let location = vec2<f32>(ulocation);
    let texture_size = textureDimensions(texture);
    
    let uv = location / vec2<f32>(texture_size);
    let freq = parameters.frequency;
    
    let scaled_uv = uv * freq;
    
    let cell_id = floor(scaled_uv);
    let local_pos = fract(scaled_uv);
    
    var min_distance = INFINITY;
    for (var x: i32 = -1; x <= 1; x++) {
        for (var y: i32 = -1; y <= 1; y++) {
            let offset = vec2<f32>(f32(x), f32(y));
            
            let id = vec2<f32>(
                fract((cell_id.x + f32(x)) / freq) * freq,
                fract((cell_id.y + f32(y)) / freq) * freq
            );

            let seeded_id = id + vec2<f32>(f32(parameters.seed) * 333, f32(parameters.seed) * 563);
            
            let h = (hash22(seeded_id) * 0.5 + 0.5);
            let point_pos = offset + h;
            
            let d = local_pos - point_pos;
            min_distance = min(min_distance, length(d));
        }
    }

    var value = min_distance;
    if (parameters.invert != 0u) {
        value = 1.0 - value;
    }

    return value;
}