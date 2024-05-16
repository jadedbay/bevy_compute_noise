@group(0) @binding(0)
var texture: texture_storage_2d<r8unorm, write>;
@group(0) @binding(1)
var<storage, read> texture_size: vec2<f32>;

@group(1) @binding(0)
var<storage, read> vectors: array<vec2<f32>>;
@group(1) @binding(1)
var<storage, read> frequency: u32;

@compute @workgroup_size(8, 8)
fn noise(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<u32>(
        invocation_id.x,
        invocation_id.y
    );

    let cell_size = texture_size / f32(frequency);
    let i = vec2<u32>(vec2<f32>(location) / cell_size);
    let f = vec2<f32>(location) / cell_size;

    let s = f - vec2<f32>(i);
    
    var n0 = dot_grid_gradient(i, f, vec2<u32>(0, 0));
    var n1 = dot_grid_gradient(i, f, vec2<u32>(1, 0));
    let ix0 = interpolate_cubic(n0, n1, s.x);

    n0 = dot_grid_gradient(i, f, vec2<u32>(0, 1));
    n1 = dot_grid_gradient(i, f, vec2<u32>(1, 1));
    let ix1 = interpolate_cubic(n0, n1, s.x);

    let value = interpolate_cubic(ix0, ix1, s.y);

    textureStore(texture, location, vec4<f32>((value + 1.0) / 2.0, 0.0, 0.0, 0.0));
}

fn get_vector_index(cell: vec2<u32>) -> u32 {
    let wrapped_cell = (cell + frequency) % frequency;
    return wrapped_cell.x * frequency + wrapped_cell.y;
}

fn dot_grid_gradient(i: vec2<u32>, f: vec2<f32>, corner: vec2<u32>) -> f32 {
    let gradient = vectors[get_vector_index(i + corner)];
    
    let distance_vector = f - vec2<f32>(i + corner);

    return dot(gradient, distance_vector);
}

fn interpolate_cubic(a0: f32, a1: f32, w: f32) -> f32 {
    return (a1 - a0) * (3.0 - w * 2.0) * w * w + a0;
}