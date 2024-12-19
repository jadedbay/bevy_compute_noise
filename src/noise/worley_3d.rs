use bevy::{asset::embedded_asset, prelude::*, render::{render_graph::RenderLabel, render_resource::{BindGroupLayout, BindGroupLayoutEntries, BindingType, Buffer, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::image::ComputeNoiseSize;

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Clone, Reflect, PartialEq, Eq, Debug)]
#[reflect(Default)]
pub struct Worley3d {
    pub seed: u64,
    pub cells: u32,
    pub invert: bool,
}

impl Worley3d {
    pub fn new(seed: u64, cells: u32, invert: bool) -> Self {
        Self {
            seed,
            cells,
            invert,
        }
    }

    fn generate_points(&self, width: u32, height: u32, depth: u32) -> Vec<Vec4> {
        let cell_size = (
            width as f32 / self.cells as f32, 
            height as f32 / self.cells as f32,
            depth as f32 / self.cells as f32,
        );

        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut random_points = Vec::new();
        for x in 0..self.cells {
            for y in 0..self.cells {
                for z in 0..self.cells {
                    let x_range = (x as f32 * cell_size.0)..((x + 1) as f32 * cell_size.0);
                    let y_range = (y as f32 * cell_size.1)..((y + 1) as f32 * cell_size.1);
                    let z_range = (z as f32 * cell_size.2)..((z + 1) as f32 * cell_size.2);
                    random_points.push(
                        Vec4::new(
                            rng.gen_range(x_range),
                            rng.gen_range(y_range),
                            rng.gen_range(z_range),
                            0.0
                        )
                    );
                }
            }
        }

        random_points
    }
}

impl Default for Worley3d {
    fn default() -> Self {
        Self {
            seed: 0,
            cells: 5,
            invert: false,
        }
    }
}

impl ComputeNoise for Worley3d {
    type Gpu = GpuWorley3d;

    fn buffers(&self, render_device: &RenderDevice, size: ComputeNoiseSize) -> Vec<Buffer> {
        Self::Gpu {
            cell_count: self.cells,
            points: self.generate_points(size.width(), size.height(), size.depth()),
            invert: self.invert as u32,
        }.buffers(render_device)
    }

    fn shader() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/shaders/worley_3d.wgsl".into()
    }

    fn embed_shader(app: &mut App) {
        embedded_asset!(app, "shaders/worley_3d.wgsl");
    }

    fn texture_dimension() -> TextureDimension {
        TextureDimension::D3
    }

    fn shader_def() -> ShaderDefVal {
       "WORLEY_3D".into() 
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(
            "worley3d_noise_layout",
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
pub struct GpuWorley3d {
    cell_count: u32,
    points: Vec<Vec4>,
    invert: u32,
}

impl GpuComputeNoise for GpuWorley3d {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("worley3d_points_buffer"),
                    contents: &bytemuck::cast_slice(self.points.as_slice()),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
                }
            ),
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("worley3d_buffer"),
                    contents: &bytemuck::cast_slice(&[self.cell_count, self.invert]),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
                }
            )
        ]
    } 
}