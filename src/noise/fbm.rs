use bevy::{app::App, reflect::Reflect, render::{render_resource::{BindGroupLayout, BindGroupLayoutEntries, BindingType, Buffer, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};

use crate::image::ComputeNoiseSize;

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Clone, Reflect, Default)] // TODO: manual default impl
pub struct Fbm<T: ComputeNoise> {
    pub noise: T,
    pub octaves: u32,
    pub frequency: f32,
    pub lacunarity: f32,
    pub persistence: f32,
}

// impl<T: ComputeNoise> ComputeNoise for Fbm<T> {
//     type Gpu = GpuFbm;

//     fn embed_shader(_app: &mut App) {}
//     fn shader() -> ShaderRef {"".into()}
//     fn shader_def() -> ShaderDefVal {"".into()}
//     fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
//         render_device.create_bind_group_layout(
//             "unused",
//             &BindGroupLayoutEntries::sequential(
//                 ShaderStages::COMPUTE,
//                 (
//                     BindingType::Buffer {
//                         ty: BufferBindingType::Storage { read_only: true },
//                         has_dynamic_offset: false,
//                         min_binding_size: None,
//                     },
//                 )
//             )
//         )
//     }
//     fn texture_dimension() -> TextureDimension {
//         T::texture_dimension()
//     }
//     fn buffers(&self, render_device: &RenderDevice, size: ComputeNoiseSize) -> Vec<Buffer> {
//         Self::Gpu {
            
//         }
//     }
// }

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct GpuFbm {
    pub octaves: u32,
    pub frequency: f32,
    pub lacunarity: f32,
    pub persistence: f32,
}

impl GpuComputeNoise for GpuFbm {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("fbm_buffer"),
                    contents: &bytemuck::cast_slice(&[self.clone()]),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
                }
            )
        ]
    }
}