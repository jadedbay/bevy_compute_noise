use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{binding_types::uniform_buffer_sized, BindGroupLayoutEntryBuilder, Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};


use crate::render::pipeline::noise_texture_2d;

use super::{ComputeNoise, ComputeNoiseType};

#[derive(Clone, Copy, Reflect, PartialEq, Debug, Pod, Zeroable)]
#[reflect(Default)]
#[repr(C)]
pub struct Worley {
    pub seed: u32,
    pub frequency: f32,
    pub flags: u32,
}

bitflags::bitflags! { 
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WorleyFlags: u32 {
        const TILEABLE = 1 << 0;
        const INVERT = 1 << 1;
    }
}

impl Default for Worley {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5.0,
            flags: 0,
        }
    }
}

impl ComputeNoise for Worley {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("worley2d_points_buffer"),
                    contents: &bytemuck::cast_slice(&[self.clone()]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
                }
            ),
        ] 
    }

    fn texture_dimension() -> TextureDimension {
        TextureDimension::D2
    }
}

impl ComputeNoiseType for Worley {
    fn shader_2d() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/worley_2d.wgsl".into()
    }

    fn shader_3d() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/worley_3d.wgsl".into()
    }

    fn embed_shaders(app: &mut App) {
        embedded_asset!(app, "shaders/worley_2d.wgsl");
        embedded_asset!(app, "shaders/worley_3d.wgsl");
    }

    fn shader_def() -> ShaderDefVal {
       "WORLEY".into() 
    }

    fn bind_group_layout_entries() -> Vec<BindGroupLayoutEntryBuilder> {
        vec![
            noise_texture_2d(),
            uniform_buffer_sized(false, None),
        ]
    }
}