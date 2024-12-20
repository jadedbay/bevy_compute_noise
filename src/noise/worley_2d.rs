use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};


use super::{ComputeNoise, ComputeNoiseType};

#[derive(Clone, Reflect, PartialEq, Debug)]
#[reflect(Default)]
pub struct Worley2d {
    pub seed: u32,
    pub frequency: f32,
    pub invert: bool,
}

impl Default for Worley2d {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5.0,
            invert: false,
        }
    }
}

impl ComputeNoise for Worley2d {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("worley2d_points_buffer"),
                    contents: &bytemuck::cast_slice(&[GpuWorley2d::from(self.clone())]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
                }
            ),
        ] 
    }

    fn texture_dimension() -> TextureDimension {
        TextureDimension::D2
    }
}

impl ComputeNoiseType for Worley2d {

    fn shader() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/worley_2d.wgsl".into()
    }

    fn embed_shader(app: &mut App) {
        embedded_asset!(app, "shaders/worley_2d.wgsl");
    }

    fn shader_def() -> ShaderDefVal {
       "WORLEY2D".into() 
    }
}

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct GpuWorley2d {
    seed: u32,
    frequency: f32,
    invert: u32,
}

impl From<Worley2d> for GpuWorley2d {
    fn from(value: Worley2d) -> Self {
        Self {
            seed: value.seed,
            frequency: value.frequency,
            invert: value.invert as u32,
        }
    }
}