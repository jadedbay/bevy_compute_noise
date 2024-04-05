@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8, 1)
fn worley(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(
        i32(num_workgroups.x) * 8 + i32(invocation_id.x),
        i32(num_workgroups.y) * 8 + i32(invocation_id.y)
    );
    
    textureStore(texture, location, vec4<f32>(0.0, 1.0, 0.0, 1.0));
}