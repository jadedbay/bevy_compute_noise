@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@group(1) @binding(0)
var<storage, read> points: array<vec2<f32>>;

struct NoiseParameters {
    cell_count: u32,
    invert: u32,
};
@group(1) @binding(1)
var<storage, read> parameters: NoiseParameters;

const INFINITY = 3.402823e+38;

@compute @workgroup_size(8, 8)
fn noise(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<u32>(
        invocation_id.x,
        invocation_id.y
    );

    let texture_size = textureDimensions(texture);

    let cell_size = vec2<f32>(texture_size) / f32(parameters.cell_count);
    let cell = vec2<u32>(vec2<f32>(location) / cell_size);

    var min_distance = INFINITY;
    for (var x: i32 = -1; x <= 1; x++) {
        for (var y: i32 = -1; y <= 1; y++) {
            let point_data = get_point(texture_size, vec2<i32>(cell) + vec2<i32>(x, y));
            let index = u32(point_data.x);
            let cell_offset = vec2<f32>(point_data.y, point_data.z);

            let current_distance = distance(vec2<f32>(location), points[index] + cell_offset);
            if (current_distance < min_distance) {
                min_distance = current_distance;
            }
        }
    }

    var normalized_distance = min_distance / distance(vec2<f32>(0.0, 0.0), cell_size);

    if (parameters.invert != 0u) {
        normalized_distance = 1.0 - normalized_distance;
    }

    textureStore(texture, location, vec4<f32>(normalized_distance, 0.0, 0.0, 0.0));
}

fn get_point(texture_size: vec2<u32>, base_cell: vec2<i32>) -> vec3<f32> {
    let cell_count = i32(parameters.cell_count);

    var cell = (base_cell + cell_count) % cell_count;
    var cell_offset = vec2<f32>(
        select(0.0, sign(f32(base_cell.x)) * f32(texture_size.x), cell.x != base_cell.x),
        select(0.0, sign(f32(base_cell.y)) * f32(texture_size.y), cell.y != base_cell.y)
    );

    let index = f32(cell.x * cell_count + cell.y);

    return vec3<f32>(index, cell_offset);
}