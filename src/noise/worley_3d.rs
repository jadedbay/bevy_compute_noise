// use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{binding_types::uniform_buffer_sized, BindGroupLayoutEntryBuilder, Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};
// use bytemuck::{Pod, Zeroable};


// use crate::render::pipeline::noise_texture_3d;

// use super::{ComputeNoise, ComputeNoiseType};

// #[derive(Clone, Copy, Reflect, PartialEq, Debug, Pod, Zeroable)]
// #[reflect(Default)]
// #[repr(C)]
// pub struct Worley3d {
//     pub seed: u32,
//     pub frequency: u32,
//     pub flags: u32,
// }

// impl Default for Worley3d {
//     fn default() -> Self {
//         Self {
//             seed: 0,
//             frequency: 5,
//             flags: 0,
//         }
//     }
// }

// impl ComputeNoise for Worley3d {
//     fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
//         vec![
//             render_device.create_buffer_with_data(
//                 &BufferInitDescriptor {
//                     label: Some("worley3d_points_buffer"),
//                     contents: &bytemuck::cast_slice(&[self.clone()]),
//                     usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
//                 }
//             ),
//         ]
//     }
    
//     fn texture_dimension() -> TextureDimension {
//         TextureDimension::D3
//     }
// }

// impl ComputeNoiseType for Worley3d {
//     fn shader() -> ShaderRef {
//         "embedded://bevy_compute_noise/noise/shaders/worley_3d.wgsl".into()
//     }

//     fn embed_shader(app: &mut App) {
//         embedded_asset!(app, "shaders/worley_3d.wgsl");
//     }

//     fn shader_def() -> ShaderDefVal {
//        "WORLEY3D".into() 
//     }


//     fn bind_group_layout_entries() -> Vec<BindGroupLayoutEntryBuilder> {
//         vec![
//             noise_texture_3d(),
//             uniform_buffer_sized(false, None),
//         ]
//     }
// }