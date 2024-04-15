@group(0) @binding(0)
var texture: texture_storage_2d<r8unorm, write>;
@group(1) @binding(0)
var<storage, read> points: array<vec2<f32>>;
@group(1) @binding(1)
var<storage, read> point_count: u32;

@compute @workgroup_size(8, 8, 1)
fn noise(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(
        i32(invocation_id.x),
        i32(invocation_id.y)
    );

    var min_distance = distance(vec2<f32>(location), points[0]);
    for (var i: u32 = 1u; i < point_count; i = i + 1u) {
        let current_distance = distance(vec2<f32>(location), points[i]);
        if (current_distance < min_distance) {
            min_distance = current_distance;
        }
    }

    let normalized_distance = min_distance / 50.;

    textureStore(texture, location, vec4<f32>(normalized_distance, 0.0, 0.0, 0.0));
}