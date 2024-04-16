use bevy::{prelude::*, render::{render_resource::{BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BindingType, BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages, ShaderRef, ShaderStages}, renderer::RenderDevice}};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use rand::Rng;
use crate::prelude::ComputeNoiseQueue;

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Default, Clone, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Worley2D {
    point_count: u32,
}

impl Worley2D {
    pub fn new(point_count: u32) -> Self {
        Self {
            point_count,
        }
    }

    fn generate_points(&self, width: u32, height: u32) -> Vec<Vec2> {
        let mut rng = rand::thread_rng();

        let mut random_points = Vec::new();
        for _ in 0..self.point_count {
            random_points.push(Vec2::new(rng.gen_range(0.0..width as f32), rng.gen_range(0.0..height as f32)));
        }

        random_points
    }
}

impl ComputeNoise for Worley2D {
    type Gpu = GpuWorley2D;
    
    fn gpu_data(&self, width: u32, height: u32) -> Self::Gpu {
        Self::Gpu {
            point_count: self.point_count,
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
    point_count: u32,
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
                    contents: &bytemuck::cast_slice(&[self.point_count]),
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

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct ComputeNoiseEdit<T: ComputeNoise> {
    pub image: Handle<Image>,
    pub noise: T,
}

pub fn update_noise<T: ComputeNoise>(
    mut noise_queue: ResMut<ComputeNoiseQueue<T>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<&ComputeNoiseEdit<T>, Changed<ComputeNoiseEdit<T>>>,
) {
    for noise in query.iter() {
        noise_queue.add_image(&mut images, noise.image.clone(), noise.noise.clone());
    }
}