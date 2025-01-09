use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{binding_types::uniform_buffer_sized, BindGroup, BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntryBuilder, Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};

use crate::render::pipeline::noise_texture_2d;

use super::{ComputeNoise, ComputeNoiseType};

#[derive(Clone, Copy, Reflect, PartialEq, Debug, Pod, Zeroable)]
#[reflect(Default)]
#[repr(C)]
pub struct Perlin2d {
    pub seed: u32,
    pub frequency: f32,
    pub flags: u32,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Perlin2dFlags: u32 {
        const TILEABLE = 1 << 0;
        const INVERT = 1 << 1;
        const REMAP = 1 << 2;
        const REMAP_SQRT = 1 << 3;
        const INTERPOLATE_CUBIC = 1 << 4; // quintic interpolation is default
    }
}

impl Default for Perlin2dFlags {
    fn default() -> Self {
        Self::from_bits_retain(Perlin2dFlags::REMAP_SQRT.bits())
    }
}

impl Default for Perlin2d {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5.0,
            flags: Perlin2dFlags::default().bits(),
        }
    }
}

impl ComputeNoise for Perlin2d {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> { 
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("perlin2d_buffer"),
                    contents: &bytemuck::cast_slice(&[self.clone()]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
                }
            )
        ]
    }

    fn texture_dimension() -> TextureDimension {
        TextureDimension::D2
    }
}

impl ComputeNoiseType for Perlin2d {
    fn shader_2d() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/perlin_2d.wgsl".into()
    }

    fn shader_3d() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/perlin_3d.wgsl".into()
    }

    fn embed_shaders(app: &mut App) {
        embedded_asset!(app, "shaders/perlin_2d.wgsl");
        embedded_asset!(app, "shaders/perlin_3d.wgsl");
    }

    fn shader_def() -> ShaderDefVal {
       "PERLIN".into() 
    }

    fn bind_group_layout_entries() -> Vec<BindGroupLayoutEntryBuilder> {
        vec![
            noise_texture_2d(),
            uniform_buffer_sized(false, None),
        ]
    }
}