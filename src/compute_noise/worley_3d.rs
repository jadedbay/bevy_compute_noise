use bevy::{prelude::*, render::{render_graph::RenderLabel, render_resource::{BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BindingType, BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::image::ComputeNoiseSize;

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Default, Clone, Reflect, InspectorOptions, PartialEq, Eq, Debug)]
#[reflect(InspectorOptions)]
pub struct Worley3d {
    pub seed: u64,
    pub cells: u32,
}

impl Worley3d {
    pub fn new(seed: u64, cells: u32) -> Self {
        Self {
            seed,
            cells,
        }
    }

    fn generate_points(&self, width: u32, height: u32, depth: u32) -> Vec<Vec3> {
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
                    random_points.push(Vec3::new(rng.gen_range(x_range), rng.gen_range(y_range), rng.gen_range(z_range)));
                }
            }
        }

        random_points
    }
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct Worley3dLabel;

impl ComputeNoise for Worley3d {
    type Gpu = GpuWorley3d;

    fn gpu_data(&self, size: ComputeNoiseSize) -> Self::Gpu {
        Self::Gpu {
            cell_count: self.cells,
            points: self.generate_points(size.width(), size.height(), size.depth()),
        }
    }

    fn shader() -> ShaderRef {
        "shaders/worley_3d.wgsl".into()
    }

    fn render_label() -> impl RenderLabel {
        Worley3dLabel
    }

    fn texture_dimension() -> TextureDimension {
        TextureDimension::D3
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
    points: Vec<Vec3>,
}

impl GpuComputeNoise for GpuWorley3d {
    fn bind_group(&self, render_device: &RenderDevice, layout: &BindGroupLayout) -> BindGroup {
        let points_buffer = render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                label: Some("worley3d_points_buffer"),
                contents: &bytemuck::cast_slice(self.points.as_slice()),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
            }
        );
        
        let cell_count_buffer = render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                label: Some("worley3d_cell_buffer"),
                contents: &bytemuck::cast_slice(&[self.cell_count]),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
            }
        );

        render_device.create_bind_group(
            Some("worley3d_bind_group".into()),
            layout,
            &BindGroupEntries::sequential((
                BufferBinding {
                    buffer: &points_buffer,
                    offset: 0,
                    size: None,
                },
                BufferBinding {
                    buffer: &cell_count_buffer,
                    offset: 0,
                    size: None,
                },
        )))
    }
}