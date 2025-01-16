use bevy::{reflect::Reflect, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages}, renderer::RenderDevice}};
use crate::{render::pipeline::NoiseOp, shader::ComputeNoiseShader};

use super::{ComputeNoise, ComputeNoiseGenerator};

#[derive(Clone, Reflect)]
pub struct Fbm<T: ComputeNoiseGenerator> {
    pub noise: T,
    pub octaves: u32,
    pub lacunarity: f32,
    pub persistence: f32,
    pub flags: u32,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FbmFlags: u32 {
        const INVERT = 1 << 0;
        const BILLOWY = 1 << 1;
    }
}

impl<T: ComputeNoiseGenerator> ComputeNoise for Fbm<T> {
    const NOISE_OP: NoiseOp = NoiseOp::Generator; 

    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("combined_fbm_noise_buffer"),
                    contents: &[bytemuck::cast_slice(&[
                        self.octaves,
                        self.lacunarity.to_bits(),
                        self.persistence.to_bits(),
                        self.flags,
                    ]),
                    bytemuck::cast_slice(&[self.noise.clone()])].concat(),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
                }
            )
        ]
    }
}

impl<T: ComputeNoiseGenerator> Default for Fbm<T> {
    fn default() -> Self {
        Self {
            noise: T::default(),
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
            flags: 0,
        }
    }
}

impl<T: ComputeNoiseGenerator> ComputeNoiseShader for Fbm<T> {
    fn function_name() -> &'static str {
        "fbm"
    }

    fn import_path() -> &'static str {
        "bevy_compute_noise::fbm"
    }

    fn struct_name() -> Option<&'static str> {
        Some("Config")
    }
}