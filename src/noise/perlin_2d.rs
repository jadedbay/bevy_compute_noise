use bevy::{asset::embedded_asset, prelude::*, render::{render_graph::RenderLabel, render_resource::{BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BindingType, BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use crate::image::ComputeNoiseSize;

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Clone, Reflect, PartialEq, Eq, Debug)]
#[reflect(Default)]
pub struct Perlin2d {
    pub seed: u32,
    pub frequency: u32,
    pub octaves: u32,
    pub invert: bool,
}

impl Perlin2d {
    pub fn new(seed: u32, frequency: u32, octaves: u32, invert: bool) -> Self {
        Self {
            seed,
            frequency,
            octaves,
            invert
        }
    }
}

impl Default for Perlin2d {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5,
            octaves: 4,
            invert: false,
        }
    }
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct Perlin2dLabel;

impl ComputeNoise for Perlin2d {
    type Gpu = GpuPerlin2d;

    fn gpu_data(&self, _size: ComputeNoiseSize) -> Self::Gpu {
        Self::Gpu {
            seed: self.seed,
            frequency: self.frequency,
            octaves: self.octaves,
            invert: self.invert as u32,
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
                )
            )
        )
    }
}

#[derive(Clone, Default)]
pub struct GpuPerlin2d {
    seed: u32,
    frequency: u32,
    octaves: u32,
    invert: u32,
}

impl GpuComputeNoise for GpuPerlin2d {
    fn bind_group(&self, render_device: &RenderDevice, layout: &BindGroupLayout) -> BindGroup {
        let perlin_buffer = render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                label: Some("perlin2d_buffer"),
                contents: &bytemuck::cast_slice(&[self.seed, self.frequency, self.octaves, self.invert]),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
            }
        );

        render_device.create_bind_group(
            Some("perlin2d_bind_group".into()),
            layout,
            &BindGroupEntries::sequential((
                BufferBinding {
                    buffer: &perlin_buffer,
                    offset: 0,
                    size: None,
                },
            ))
        )
    }
}
