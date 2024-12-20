use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};


use super::{ComputeNoise, ComputeNoiseType};

#[derive(Clone, Reflect, PartialEq, Eq, Debug)]
#[reflect(Default)]
pub struct Worley3d {
    pub seed: u32,
    pub frequency: u32,
    pub invert: bool,
}

impl Default for Worley3d {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5,
            invert: false,
        }
    }
}

impl ComputeNoise for Worley3d {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("worley3d_points_buffer"),
                    contents: &bytemuck::cast_slice(&[GpuWorley3d::from(self.clone())]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
                }
            ),
        ]
    }
    
    fn texture_dimension() -> TextureDimension {
        TextureDimension::D3
    }
}

impl ComputeNoiseType for Worley3d {
    fn shader() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/worley_3d.wgsl".into()
    }

    fn embed_shader(app: &mut App) {
        embedded_asset!(app, "shaders/worley_3d.wgsl");
    }

    fn shader_def() -> ShaderDefVal {
       "WORLEY_3D".into() 
    }
}

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct GpuWorley3d {
    seed: u32,
    frequency: u32,
    invert: u32,
}

impl From<Worley3d> for GpuWorley3d {
    fn from(value: Worley3d) -> Self {
        Self {
            seed: value.seed,
            frequency: value.frequency,
            invert: value.invert as u32,
        }
    }
}