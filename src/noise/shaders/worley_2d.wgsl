@group(0) @binding(0)
var texture: texture_storage_2d<r8unorm, write>;
@group(0) @binding(1)
var<storage, read> texture_size: vec2<f32>;

@group(1) @binding(0)
var<storage, read> points: array<vec2<f32>>;
@group(1) @binding(1)
var<storage, read> cell_count: u32;

const INFINITY = 3.402823e+38;

@compute @workgroup_size(8, 8)
fn noise(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<u32>(
        invocation_id.x,
        invocation_id.y
    );

    let cell_size = texture_size / f32(cell_count);
    let cell = vec2<u32>(vec2<f32>(location) / cell_size);

    var distance = INFINITY;
    for (var x: i32 = -1; x <= 1; x++) {
        for (var y: i32 = -1; y <= 1; y++) {
            let point_data = get_point(vec2<i32>(cell) + vec2<i32>(x, y));
            let index = u32(point_data.x);
            let cell_offset = vec2<f32>(point_data.y, point_data.z);

            let current_distance = distance(vec2<f32>(location), points[index] + cell_offset);
            if (current_distance < distance) {
                distance = current_distance;
            }
        }
    }

    let normalized_distance = 1.0 - (distance / distance(vec2<f32>(0.0, 0.0), cell_size));

    textureStore(texture, location, vec4<f32>(normalized_distance, 0.0, 0.0, 0.0));
}

fn get_point(base_cell: vec2<i32>) -> vec3<f32> {
    let cell_count = i32(cell_count);

    var cell = (base_cell + cell_count) % cell_count;
    var cell_offset = vec2<f32>(
        select(0.0, sign(f32(base_cell.x)) * texture_size.x, cell.x != base_cell.x),
        select(0.0, sign(f32(base_cell.y)) * texture_size.y, cell.y != base_cell.y)
    );

    let index = f32(cell.x * cell_count + cell.y);

    return vec3<f32>(index, cell_offset);
}