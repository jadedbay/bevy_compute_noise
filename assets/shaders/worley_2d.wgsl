@group(0) @binding(0)
var texture: texture_storage_2d<r8unorm, write>;
@group(0) @binding(1)
var<storage, read> texture_size: vec2<f32>;

@group(1) @binding(0)
var<storage, read> points: array<vec2<f32>>;
@group(1) @binding(1)
var<storage, read> cell_count: vec2<u32>;

const INFINITY = 3.402823e+38;

@compute @workgroup_size(8, 8, 1)
fn noise(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<u32>(
        invocation_id.x,
        invocation_id.y
    );

    let cell_size = texture_size.x / f32(cell_count.x);
    var cell = vec2<u32>(vec2<f32>(location) / cell_size);
    cell = vec2<u32>(cell.x - 1, cell.y - 1);

    // var distance = INFINITY;
    // for (var x: i32 = -1; x <= 1; x = x + 1) {
    //     for (var y: i32 = -1; y <= 1; y = y + 1) {
    //         let index = get_index(vec2<u32>(vec2<i32>(cell) + vec2<i32>(x, y)), cell_count);
    //         let current_distance = distance(vec2<f32>(location), points[index]);
    //         if (current_distance < distance) {
    //             distance = current_distance;
    //         }
    //     }
    // }

    var distance = INFINITY;
    for (var i: u32 = 0u; i < 25u; i = i + 1u) {
        let current_distance = distance(vec2<f32>(location), points[i]);
        if (current_distance < distance) {
            distance = current_distance;
        }
    }

    let normalized_distance = 1.0 - (distance / (texture_size.x / f32(cell_count.x)));

    textureStore(texture, location, vec4<f32>(normalized_distance, 0.0, 0.0, 0.0));
}

fn get_index(cell: vec2<u32>, cell_count: vec2<u32>) -> u32 {
    let clump = cell / cell_count;
    var x = (cell_count.x * cell_count.y * 2 * clump.x) + (cell.x - cell_count.x * clump.x) * cell_count.y + 1;
    let y = cell_count.y * cell_count.x * clump.y + (cell.y - cell_count.y * clump.y);

    return x + y;
}