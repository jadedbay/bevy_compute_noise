#define_import_path bevy_compute_noise::worley_2d

#import bevy_compute_noise::util::{hash22, INFINITY, texture2d as texture}

const TILEABLE: u32 = 1u;

struct Worley {
    seed: u32,
    frequency: f32,
    flags: u32,
};
@group(0) @binding(1)
var<uniform> worley: Worley;

@compute @workgroup_size(32, 32)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = invocation_id.xy;
    let texture_size = textureDimensions(texture);
    let uv = vec2<f32>(location) / vec2<f32>(texture_size);
    
    let value = worley_2d(uv, worley);
    textureStore(texture, location, vec4<f32>(value, 0.0, 0.0, 1.0));
}

fn worley_2d(uv: vec2<f32>, worley: Worley) -> f32 {
    let frequency = worley.frequency;
    let scaled_uv = uv * frequency;
    
    let cell_id = floor(scaled_uv);
    let local_pos = fract(scaled_uv);
    
    var min_distance = INFINITY;
    for (var x: i32 = -1; x <= 1; x++) {
        for (var y: i32 = -1; y <= 1; y++) {
            let offset = vec2<f32>(f32(x), f32(y));
            
            let id = select(
                cell_id + vec2<f32>(f32(x), f32(y)),
                vec2<f32>(
                    fract((cell_id.x + f32(x)) / frequency) * frequency,
                    fract((cell_id.y + f32(y)) / frequency) * frequency
                ),
                (worley.flags & TILEABLE) != 0u
            );

            let seeded_id = id + vec2<f32>(f32(worley.seed) * 333, f32(worley.seed) * 563);
            
            let h = (hash22(seeded_id) * 0.5 + 0.5);
            let point_pos = offset + h;
            
            let d = local_pos - point_pos;
            min_distance = min(min_distance, length(d));
        }
    }

    var value = min_distance;

    return value;
}