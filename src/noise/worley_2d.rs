use bevy::{asset::embedded_asset, prelude::*, render::{render_graph::RenderLabel, render_resource::{BindGroupLayout, BindGroupLayoutEntries, BindingType, Buffer, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::image::ComputeNoiseSize;

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Clone, Reflect, PartialEq, Eq, Debug)]
#[reflect(Default)]
pub struct Worley2d {
    pub seed: u32,
    pub frequency: u32,
    pub invert: bool,
}

impl Default for Worley2d {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5,
            invert: false,
        }
    }
}

impl ComputeNoise for Worley2d {
    type Gpu = GpuWorley2d;

    fn buffers(&self, render_device: &RenderDevice, _size: ComputeNoiseSize) -> Vec<Buffer> {
        Self::Gpu {
            seed: self.seed,
            frequency: self.frequency,
            invert: self.invert as u32,
        }.buffers(render_device)
    }

    fn shader() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/worley_2d.wgsl".into()
    }

    fn embed_shader(app: &mut App) {
        embedded_asset!(app, "shaders/worley_2d.wgsl");
    }

    fn texture_dimension() -> TextureDimension {
        TextureDimension::D2
    }

    fn shader_def() -> ShaderDefVal {
       "WORLEY_2D".into() 
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(
            "worley2d_noise_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            )
        )
    }
}

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct GpuWorley2d {
    seed: u32,
    frequency: u32,
    invert: u32,
}

impl GpuComputeNoise for GpuWorley2d {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("worley2d_points_buffer"),
                    contents: &bytemuck::cast_slice(&[self.clone()]),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
                }
            ),
        ] 
    }
}