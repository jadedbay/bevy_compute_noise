use bevy::{prelude::*, render::{render_resource::{BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BindingType, BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderRef, ShaderStages}, renderer::RenderDevice}};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use rand::Rng;

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Default, Clone, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Worley2D {
    #[inspector(min = 1)]
    cells_x: u32,
    cells_y: u32,
}

impl Worley2D {
    pub fn new(cells: (u32, u32)) -> Self {
        Self {
            cells_x: cells.0,
            cells_y: cells.1,
        }
    }

    fn generate_points(&self, width: u32, height: u32) -> Vec<Vec2> {
        let cell_size = (width as f32 / self.cells_x as f32, height as f32 / self.cells_y as f32);

        let mut rng = rand::thread_rng();

        let mut random_points = Vec::new();
        for x in 0..self.cells_x {
            for y in 0..self.cells_y {
                let x_range = (x as f32 * cell_size.0)..((x + 1) as f32 * cell_size.0);
                let y_range = (y as f32* cell_size.1)..((y + 1) as f32 * cell_size.1);
                random_points.push(Vec2::new(rng.gen_range(x_range), rng.gen_range(y_range)));
            }
        }

        let mut duplicated_random_points = Vec::new();

        for x in 0..=1 {
            for y in 0..=1 {
                duplicated_random_points.extend(random_points.iter().map(|p| *p + Vec2::new(x as f32 * width as f32, y as f32 * height as f32)));
            }
        }

        duplicated_random_points
    }
}

impl ComputeNoise for Worley2D {
    type Gpu = GpuWorley2D;
    
    fn gpu_data(&self, width: u32, height: u32) -> Self::Gpu {
        Self::Gpu {
            cell_count: [self.cells_x, self.cells_y],
            points: self.generate_points(width, height),
        }
    }

    fn shader() -> ShaderRef {
        "shaders/worley_2d.wgsl".into()
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(
            "worley_noise_layout",
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
pub struct GpuWorley2D {
    cell_count: [u32; 2],
    points: Vec<Vec2>,
}

impl GpuComputeNoise for GpuWorley2D {
    fn to_bind_group(&self, render_device: &RenderDevice, layout: &BindGroupLayout) -> BindGroup {
        let points_buffer = render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                    label: None,
                    contents: &bytemuck::cast_slice(self.points.as_slice()),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
            });
        
        let point_count_buffer = render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                    label: None,
                    contents: &bytemuck::cast_slice(&self.cell_count),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
            });

        render_device.create_bind_group(
            Some("noise_bind_group".into()),
            layout,
            &BindGroupEntries::sequential((
                BufferBinding {
                    buffer: &points_buffer,
                    offset: 0,
                    size: None,
                },
                BufferBinding {
                    buffer: &point_count_buffer,
                    offset: 0,
                    size: None,
                },
        )))
    }
}

#[test]
fn test() {
    let cell = [6, 3];
    let cell_count = [5, 2];

    
    let clump = [cell[0] / cell_count[0], cell[1] / cell_count[1]];
    let mut index = (cell_count[0] * cell_count[1] * 2 * clump[0]) + (cell[0] - cell_count[0] * clump[0]) * cell_count[1];
    let y = cell_count[1] * cell_count[0] * clump[1] + (cell[1] - cell_count[1] * clump[1]);
    
    dbg!(y);
    index += y;

    println!("{}", index);
}