use bevy::{reflect::Reflect, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, TextureDimension}, renderer::RenderDevice}};
use crate::noise::{ComputeNoise, ComputeNoiseType};

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
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("combined_fbm_noise_buffer"),
                    contents: &[bytemuck::cast_slice(&[
                        self.octaves,
                        self.lacunarity.to_bits(),
                        self.persistence.to_bits(),
                        0u32,
                    ]),
                    bytemuck::cast_slice(&[self.noise.clone()])].concat(),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
                }
            )
        ]
    }
}