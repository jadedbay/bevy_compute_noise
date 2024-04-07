@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1)
var<uniform> point_count: vec4<f32>; 

@compute @workgroup_size(8, 8, 1)
fn worley(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(
        i32(invocation_id.x),
        i32(invocation_id.y)
    );

    textureStore(texture, location, point_count);
}