use bevy::{reflect::Reflect, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};


use super::{ComputeNoise, ComputeNoiseType};

#[derive(Clone, Reflect, Default)] // TODO: manual default impl
pub struct Fbm<T: ComputeNoise> {
    pub noise: T,
    pub octaves: u32,
    pub lacunarity: f32,
    pub persistence: f32,
}

impl<T: ComputeNoiseType> ComputeNoise for Fbm<T> {
    fn texture_dimension() -> TextureDimension {
        T::texture_dimension()
    }
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        let mut buffers = vec![    
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("perlin2d_buffer"),
                    contents: &bytemuck::cast_slice(&[GpuFbm::from(self.clone())]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
                }
            )
        ];
        buffers.extend(self.noise.buffers(render_device));
        buffers
    }
}

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct GpuFbm {
    pub octaves: u32,
    pub lacunarity: f32,
    pub persistence: f32,
}

impl<T: ComputeNoiseType> From<Fbm<T>> for GpuFbm {
    fn from(value: Fbm<T>) -> Self {
        Self {
            octaves: value.octaves,
            lacunarity: value.lacunarity,
            persistence: value.persistence,
        }
    }
}