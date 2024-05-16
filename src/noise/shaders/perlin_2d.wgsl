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
    let cell = vec2<u32>(vec2<f32>(location) / cell_size);
    let location_in_cell = (vec2<f32>(location) - (cell_size * vec2<f32>(cell))) / cell_size;
    
    var n0 = dot_grid_gradient(cell, location_in_cell, vec2<f32>(0.0, 0.0));
    var n1 = dot_grid_gradient(cell + vec2<u32>(1, 0), location_in_cell, vec2<f32>(1.0, 0.0));
    let ix0 = interpolate_cubic(n0, n1, location_in_cell.x);

    n0 = dot_grid_gradient(cell + vec2<u32>(0, 1), location_in_cell, vec2<f32>(0.0, 1.0));
    n1 = dot_grid_gradient(cell + vec2<u32>(1, 1), location_in_cell, vec2<f32>(1.0, 1.0));
    let ix1 = interpolate_cubic(n0, n1, location_in_cell.x);

    let value = interpolate_cubic(ix0, ix1, location_in_cell.y);

    textureStore(texture, location, vec4<f32>(value, 0.0, 0.0, 0.0));
}

fn get_vector_index(base_cell: vec2<u32>) -> u32 {
    let cell = (base_cell + frequency) % frequency;
    return u32(cell.x * frequency + cell.y);
}

fn dot_grid_gradient(cell: vec2<u32>, relative_location: vec2<f32>, point_location: vec2<f32>) -> f32 {
    let gradient = vectors[get_vector_index(cell)];
    
    let distance_vector = relative_location - point_location;

    return dot(gradient, distance_vector);
}

fn interpolate_cubic(a0: f32, a1: f32, w: f32) -> f32 {
    return (a1 - a0) * (3.0 - w * 2.0) * w * w + a0;
}