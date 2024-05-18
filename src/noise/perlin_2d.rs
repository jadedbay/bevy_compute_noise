use bevy::{asset::embedded_asset, prelude::*, render::{render_graph::RenderLabel, render_resource::{BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BindingType, BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use crate::image::ComputeNoiseSize;

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Default, Clone, Reflect, InspectorOptions, PartialEq, Eq, Debug)]
#[reflect(InspectorOptions)]
pub struct Perlin2d {
    pub frequency: u32,
    pub octaves: u32,
}

impl Perlin2d {
    pub fn new(frequency: u32, octaves: u32) -> Self {
        Self {
            frequency,
            octaves
        }
    }
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct Perlin2dLabel;

impl ComputeNoise for Perlin2d {
    type Gpu = GpuPerlin2d;

    fn gpu_data(&self, _size: ComputeNoiseSize) -> Self::Gpu {
        Self::Gpu {
            frequency: self.frequency,
            octaves: self.octaves,
        }
    }

    fn shader() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/perlin_2d.wgsl".into()
    }

    fn embed_asset(app: &mut App) {
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

#[derive(Clone, Default)]
pub struct GpuPerlin2d {
    frequency: u32,
    octaves: u32,
}

impl GpuComputeNoise for GpuPerlin2d {
    fn bind_group(&self, render_device: &RenderDevice, layout: &BindGroupLayout) -> BindGroup {
        let frequency_buffer = render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                label: Some("perlin2d_frequency_buffer"),
                contents: &bytemuck::cast_slice(&[self.frequency]),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
            }
        );

        let octaves_buffer = render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                label: Some("perlin2d_octaves_buffer"),
                contents: &bytemuck::cast_slice(&[self.octaves]),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
            }
        );

        render_device.create_bind_group(
            Some("perlin2d_bind_group".into()),
            layout,
            &BindGroupEntries::sequential((
                BufferBinding {
                    buffer: &frequency_buffer,
                    offset: 0,
                    size: None,
                },
                BufferBinding {
                    buffer: &octaves_buffer,
                    offset: 0,
                    size: None,
                },
            ))
        )
    }
}
