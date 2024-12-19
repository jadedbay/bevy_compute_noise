#import bevy_compute_noise::util::hash22

@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

struct NoiseParameters {
    seed: u32,
    frequency: u32,
    invert: u32,
};
@group(1) @binding(0)
var<uniform> parameters: NoiseParameters;

const INFINITY = 3.402823e+38;

@compute @workgroup_size(8, 8)
fn noise(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<u32>(invocation_id.xy);
    let texture_size = textureDimensions(texture);
    
    let uv = vec2<f32>(location) / vec2<f32>(texture_size);
    let freq = f32(parameters.frequency);
    
    let scaled_uv = uv * freq;
    
    let cell_id = floor(scaled_uv);
    let local_pos = fract(scaled_uv);
    
    var min_distance = INFINITY;
    for (var x: i32 = -1; x <= 1; x++) {
        for (var y: i32 = -1; y <= 1; y++) {
            let offset = vec2<f32>(f32(x), f32(y));
            
            let id = vec2<f32>(
               (cell_id.x + f32(x) + freq) % freq,
               (cell_id.y + f32(y) + freq) % freq
            );

            let seeded_id = id + vec2<f32>(f32(parameters.seed) * 333, f32(parameters.seed) * 563);
            
            let h = (hash22(seeded_id) * 0.5 + 0.5);
            let point_pos = offset + h;
            
            let d = local_pos - point_pos;
            min_distance = min(min_distance, length(d));
        }
    }

    var result = min_distance;
    if (parameters.invert != 0u) {
        result = 1.0 - result;
    }

    textureStore(texture, location, vec4<f32>(result, 0.0, 0.0, 1.0));
}