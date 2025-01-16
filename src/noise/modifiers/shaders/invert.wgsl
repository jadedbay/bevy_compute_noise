#ifdef 2D
    @group(0) @binding(0) var input_texture: texture_storage_2d<rgba8unorm, read_write>;
    @group(0) @binding(1) var output_texture: texture_storage_2d<rgba8unorm, read_write>;
    @compute @workgroup_size(32, 32)
#endif
#ifdef 3D
    @group(0) @binding(0) var input_texture: texture_storage_3d<rgba8unorm, read_write>;
    @group(0) @binding(1) var output_texture: texture_storage_3d<rgba8unorm, read_write>;
    @compute @workgroup_size(8, 8, 8)
#endif
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    #ifdef 2D
        let location = invocation_id.xy; 
    #endif
    #ifdef 3D
        let location = invocation_id.xyz; 
    #endif

    var value = textureLoad(input_texture, location);
    value = invert(value);
    textureStore(output_texture, location, value);
}

fn invert(value: f32) -> f32 {
    return 1.0 - value;
}