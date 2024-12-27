use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};

use super::{ComputeNoise, ComputeNoiseType};

#[derive(Clone, Copy, Reflect, PartialEq, Debug, Pod, Zeroable)]
#[reflect(Default)]
#[repr(C)]
pub struct Perlin2d {
    pub seed: u32,
    pub frequency: u32,
    pub flags: u32,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Perlin2dFlags: u32 {
        const TILEABLE = 1 << 0;
        const INVERT = 1 << 1;
        const REMAP = 1 << 2;
        const REMAP_SQRT_2 = 1 << 3;
        const INTERPOLATE_CUBIC = 1 << 4; // quintic interpolation is default
    }
}

impl Default for Perlin2dFlags {
    fn default() -> Self {
        Self::from_bits_retain(Perlin2dFlags::REMAP_SQRT_2.bits())
    }
}

impl Default for Perlin2d {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5,
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
    fn shader() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/perlin_2d.wgsl".into()
    }

    fn embed_shader(app: &mut App) {
        embedded_asset!(app, "shaders/perlin_2d.wgsl");
    }


    fn shader_def() -> ShaderDefVal {
       "PERLIN2D".into() 
    }
}