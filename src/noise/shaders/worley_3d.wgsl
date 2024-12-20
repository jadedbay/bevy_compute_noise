#define_import_path bevy_compute_noise::worley3d

#import bevy_compute_noise::util::{hash33, INFINITY}

@group(0) @binding(0)
var texture: texture_storage_3d<rgba8unorm, read_write>;

struct NoiseParameters {
    seed: u32,
    frequency: u32,
    invert: u32,
};
@group(1) @binding(0)
var<uniform> parameters: NoiseParameters;

@compute @workgroup_size(8, 8, 8)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let value = noise(invocation_id, parameters);
    textureStore(texture, invocation_id, vec4<f32>(value, 0.0, 0.0, 0.0));
}

fn noise(location: vec3<u32>, parameters: NoiseParameters) -> f32 {
    let texture_size = textureDimensions(texture);

    let uv = vec3<f32>(location) / vec3<f32>(texture_size);
    let freq = f32(parameters.frequency);
    
    let scaled_uv = uv * freq;
    
    let cell_id = floor(scaled_uv);
    let local_pos = fract(scaled_uv);

    var min_distance = INFINITY;
    for (var x: i32 = -1; x <= 1; x++) {
        for (var y: i32 = -1; y <= 1; y++) {
            for (var z: i32 = -1; z <= 1; z++) {
                let offset = vec3<f32>(f32(x), f32(y), f32(z));
            
                let id = vec3<f32>(
                    (cell_id.x + f32(x) + freq) % freq,
                    (cell_id.y + f32(y) + freq) % freq,
                    (cell_id.z + f32(z) + freq) % freq,
                );

                let seeded_id = id + vec3<f32>(f32(parameters.seed) * 333, f32(parameters.seed) * 563, f32(parameters.seed) * 122);
                
                let h = (hash33(seeded_id) * 0.5 + 0.5);
                let point_pos = offset + h;
                
                let d = local_pos - point_pos;
                min_distance = min(min_distance, length(d));
            }
        }
    }

    // var normalized_distance = min_distance / distance(vec3<f32>(0.0, 0.0, 0.0), cell_size);

    var value = min_distance;
    if (parameters.invert != 0u) {
        value = 1.0 - value;
    }

    return value;
}