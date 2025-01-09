// use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{binding_types::uniform_buffer_sized, BindGroupLayoutEntryBuilder, Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};
// use bytemuck::{Pod, Zeroable};

// use crate::render::pipeline::noise_texture_3d;

// use super::{ComputeNoise, ComputeNoiseType};

// #[derive(Clone, Copy, Reflect, PartialEq, Debug, Pod, Zeroable)]
// #[reflect(Default)]
// #[repr(C)]
// pub struct Perlin3d {
//     pub seed: u32,
//     pub frequency: u32,
//     pub flags: u32,
// }

// bitflags::bitflags! {
//     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//     pub struct Perlin3dFlags: u32 {
//         const TILEABLE = 1 << 0;
//         const INVERT = 1 << 1;
//         const REMAP = 1 << 2;
//         const REMAP_SQRT_3 = 1 << 3;
//         const INTERPOLATE_CUBIC = 1 << 4; // quintic interpolation is default
//     }
// }

// impl Default for Perlin3dFlags {
//     fn default() -> Self {
//         Self::from_bits_retain(Perlin3dFlags::REMAP_SQRT_3.bits())
//     }
// }

// impl Default for Perlin3d {
//     fn default() -> Self {
//         Self {
//             seed: 0,
//             frequency: 5,
//             flags: (Perlin3dFlags::REMAP_SQRT_3).bits(),
//         }
//     }
// }

// impl ComputeNoise for Perlin3d {
//     fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> { 
//         vec![    
//             render_device.create_buffer_with_data(
//                 &BufferInitDescriptor {
//                     label: Some("perlin2d_buffer"),
//                     contents: &bytemuck::cast_slice(&[self.clone()]),
//                     usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
//                 }
//             )
//         ]
//     }

//     fn texture_dimension() -> TextureDimension {
//         TextureDimension::D3
//     }
// }

// impl ComputeNoiseType for Perlin3d {
//     fn shader_2d() -> ShaderRef {
//         "embedded://bevy_compute_noise/noise/shaders/perlin_3d.wgsl".into()
//     }

//     fn shader_3d() -> ShaderRef {
//         "embedded://bevy_compute_noise/noise/shaders/perlin_3d.wgsl".into()
//     }

//     fn embed_shader(app: &mut App) {
//         embedded_asset!(app, "shaders/perlin_3d.wgsl");
//     }


//     fn shader_def() -> ShaderDefVal {
//        "PERLIN3D".into() 
//     }

//     fn bind_group_layout_entries() -> Vec<BindGroupLayoutEntryBuilder> {
//         vec![
//             noise_texture_3d(),
//             uniform_buffer_sized(false, None),
//         ]
//     }
// }