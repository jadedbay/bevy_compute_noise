#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(101) var texture: texture_2d<f32>;
@group(2) @binding(102) var texture_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let value = textureSample(texture, texture_sampler, mesh.uv);
    return vec4<f32>(vec3<f32>(value.x), 1.0);
}