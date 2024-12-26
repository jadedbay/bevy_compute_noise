use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};


use super::{ComputeNoise, ComputeNoiseType};

#[derive(Clone, Copy, Reflect, PartialEq, Debug, Pod, Zeroable)]
#[reflect(Default)]
#[repr(C)]
pub struct Worley2d {
    pub seed: u32,
    pub frequency: u32,
    pub flags: u32,
}

bitflags::bitflags! { 
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WorleyFlags: u32 {
        const INVERT = 1 << 0;
    }
}

impl Default for Worley2d {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5,
            flags: 0,
        }
    }
}

impl ComputeNoise for Worley2d {
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