use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};

use super::{ComputeNoise, ComputeNoiseType};

#[derive(Clone, Reflect, PartialEq, Debug)]
#[reflect(Default)]
pub struct Perlin2d {
    pub seed: u32,
    pub frequency: u32,
    pub octaves: u32,
    pub invert: bool,
    pub persistence: f32,
}

impl Default for Perlin2d {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5,
            octaves: 4,
            invert: false,
            persistence: 1.0,
        }
    }
}

impl ComputeNoise for Perlin2d {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> { 
        vec![    
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("perlin2d_buffer"),
                    contents: &bytemuck::cast_slice(&[GpuPerlin2d::from(self.clone())]),
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
       "PERLIN_2D".into() 
    }
}

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct GpuPerlin2d {
    seed: u32,
    frequency: u32,
    octaves: u32,
    invert: u32,
    persistence: f32,
}

impl From<Perlin2d> for GpuPerlin2d {
    fn from(value: Perlin2d) -> Self {
        Self {
            seed: value.seed,
            frequency: value.frequency,
            octaves: value.octaves,
            invert: value.invert as u32,
            persistence: value.persistence,
        }
    }
}