@group(0) @binding(0)
var texture: texture_storage_3d<r8unorm, write>;
@group(0) @binding(1)
var<storage, read> texture_size: vec3<f32>;

@group(1) @binding(0)
var<storage, read> points: array<vec3<f32>>;
@group(1) @binding(1)
var<storage, read> cell_count: u32;

const INFINITY = 3.402823e+38;

@compute @workgroup_size(8, 8, 8)
fn noise(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec3<u32>(
        invocation_id.x,
        invocation_id.y,
        invocation_id.z,
    );

    let cell_size = texture_size / f32(cell_count);
    let cell = vec3<u32>(vec3<f32>(location) / cell_size);

    var distance = INFINITY;
    for (var x: i32 = -1; x <= 1; x = x + 1) {
        for (var y: i32 = -1; y <= 1; y = y + 1) {
            for (var z: i32 = -1; z <= 1; z = z + 1) {
                let point_data = get_point(vec3<i32>(cell) + vec3<i32>(x, y, z), i32(cell_count), texture_size);
                let index = u32(point_data.x);
                let cell_offset = vec3<f32>(point_data.y, point_data.z, point_data.w);

                let current_distance = distance(vec3<f32>(location), points[index] + cell_offset);
                if (current_distance < distance) {
                    distance = current_distance;
                }
            }
        }

        let normalized_distance = 1.0 - (distance / distance(vec3<f32>(0.0, 0.0, 0.0), cell_size));

        textureStore(texture, location, vec4<f32>(normalized_distance, 0.0, 0.0, 0.0));
    }
}

fn get_point(base_cell: vec3<i32>, cell_count: i32, texture_size: vec3<f32>) -> vec4<f32> {
    var cell = (base_cell + cell_count) % cell_count;
    var cell_offset = vec3<f32>(
        select(0.0, sign(f32(base_cell.x)) * texture_size.x, cell.x != base_cell.x),
        select(0.0, sign(f32(base_cell.y)) * texture_size.y, cell.y != base_cell.y),
        select(0.0, sign(f32(base_cell.z)) * texture_size.z, cell.z != base_cell.z),
    );

    let index = f32(cell.x * cell_count * cell_count + cell.y * cell_count + cell.z);

    return vec4<f32>(index, cell_offset);
}