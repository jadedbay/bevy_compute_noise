use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};

use crate::render::pipeline::NoiseOp;

use super::{ComputeNoise, ComputeNoiseGenerator};

#[derive(Clone, Copy, Reflect, PartialEq, Debug, Pod, Zeroable)]
#[reflect(Default)]
#[repr(C)]
pub struct Perlin {
    pub seed: u32,
    pub frequency: f32,
    pub flags: u32,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PerlinFlags: u32 {
        const TILEABLE = 1 << 0;
        const REMAP = 1 << 1;
        const INTERPOLATE_CUBIC = 1 << 2; // quintic interpolation is default
    }
}

impl Default for PerlinFlags {
    fn default() -> Self {
        Self::from_bits_retain(PerlinFlags::REMAP.bits())
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5.0,
            flags: PerlinFlags::default().bits(),
        }
    }
}

impl ComputeNoise for Perlin {
    const NOISE_OP: NoiseOp = NoiseOp::Generator; 

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
}

impl ComputeNoiseGenerator for Perlin {
    fn shader_2d() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/generators/shaders/perlin_2d.wgsl".into()
    }

    fn shader_3d() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/generators/shaders/perlin_3d.wgsl".into()
    }

    fn embed_shaders(app: &mut App) {
        embedded_asset!(app, "shaders/perlin_2d.wgsl");
        embedded_asset!(app, "shaders/perlin_3d.wgsl");
    }

    fn shader_def() -> ShaderDefVal {
       "PERLIN".into() 
    }
}