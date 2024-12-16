use bevy::{asset::embedded_asset, prelude::*, render::{render_graph::RenderLabel, render_resource::{BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BindingType, Buffer, BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::image::ComputeNoiseSize;

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Clone, Reflect, PartialEq, Eq, Debug)]
#[reflect(Default)]
pub struct Worley2d {
    pub seed: u64,
    pub cells: u32,
    pub invert: bool,
}

impl Worley2d {
    pub fn new(seed: u64, cells: u32, invert: bool) -> Self {
        Self {
            seed,
            cells,
            invert,
        }
    }

    fn generate_points(&self, width: u32, height: u32) -> Vec<Vec2> {
        let cell_size = (
            width as f32 / self.cells as f32,
            height as f32 / self.cells as f32
        );

        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut random_points = Vec::new();
        for x in 0..self.cells {
            for y in 0..self.cells {
                let x_range = (x as f32 * cell_size.0)..((x + 1) as f32 * cell_size.0);
                let y_range = (y as f32 * cell_size.1)..((y + 1) as f32 * cell_size.1);
                random_points.push(Vec2::new(rng.gen_range(x_range), rng.gen_range(y_range)));
            }
        }

        random_points
    }
}

impl Default for Worley2d {
    fn default() -> Self {
        Self {
            seed: 0,
            cells: 5,
            invert: false,
        }
    }
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct Worley2dLabel;

impl ComputeNoise for Worley2d {
    type Gpu = GpuWorley2d;

    fn buffers(&self, render_device: &RenderDevice, size: ComputeNoiseSize) -> Vec<Buffer> {
        Self::Gpu {
            cell_count: self.cells,
            points: self.generate_points(size.width(), size.height()),
            invert: self.invert as u32,
        }.buffers(render_device)
    }

    fn shader() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/worley_2d.wgsl".into()
    }

    fn embed_shader(app: &mut App) {
        embedded_asset!(app, "shaders/worley_2d.wgsl");
    }

    fn render_label() -> impl RenderLabel {
        Worley2dLabel
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
pub struct GpuWorley2d {
    cell_count: u32,
    points: Vec<Vec2>,
    invert: u32,
}

impl GpuComputeNoise for GpuWorley2d {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("worley2d_points_buffer"),
                    contents: &bytemuck::cast_slice(self.points.as_slice()),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
                }
            ),
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("worley2d_cell_count_buffer"),
                    contents: &bytemuck::cast_slice(&[self.cell_count, self.invert]),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
                }
            ) 
        ] 
    }
}