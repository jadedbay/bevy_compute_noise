use bevy::{asset::embedded_asset, prelude::*, render::{render_graph::RenderLabel, render_resource::{BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BindingType, Buffer, BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};

use crate::image::ComputeNoiseSize;

use super::{ComputeNoise, GpuComputeNoise};

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

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct Perlin2dLabel;

impl ComputeNoise for Perlin2d {
    type Gpu = GpuPerlin2d;

    fn buffers(&self, render_device: &RenderDevice, _size: ComputeNoiseSize) -> Vec<Buffer> {
        Self::Gpu {
            seed: self.seed,
            frequency: self.frequency,
            octaves: self.octaves,
            invert: self.invert as u32,
            persistence: self.persistence,
        }.buffers(render_device)
    }

    fn shader() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/perlin_2d.wgsl".into()
    }

    fn embed_shader(app: &mut App) {
        embedded_asset!(app, "shaders/perlin_2d.wgsl");
    }

    fn render_label() -> impl RenderLabel {
        Perlin2dLabel
    }

    fn texture_dimension() -> TextureDimension {
        TextureDimension::D2
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(
            "worley2d_noise_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                )
            )
        )
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

impl GpuComputeNoise for GpuPerlin2d {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![    
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("perlin2d_buffer"),
                    contents: &bytemuck::cast_slice(&[self.clone()]),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
                }
            )
        ]
    } 
}
